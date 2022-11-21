#![allow(dead_code)]
use crate::rgb::*;
use parking_lot::{Mutex, RwLock};
use rayon::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Oklab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl Oklab {
    pub fn oklab_to_lrgb(self) -> LRgb {
        let l_ = self.l + 0.3963377774 * self.a + 0.2158037573 * self.b;
        let m_ = self.l - 0.1055613458 * self.a - 0.0638541728 * self.b;
        let s_ = self.l - 0.0894841775 * self.a - 1.291485548 * self.b;
        let l = l_.powi(3);
        let m = m_.powi(3);
        let s = s_.powi(3);
        Rgb {
            r: 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            b: -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s,
        }
    }

    pub fn oklab_to_srgb(self) -> SRgb {
        // RGB clipping
        // You might want to use the other oklab_to_srgb_* functions
        self.oklab_to_lrgb().lrgb_to_srgb()
    }

    pub fn oklab_to_oklch(self) -> Oklch {
        Oklch {
            l: self.l,
            c: self.a.hypot(self.b),
            h: self.b.atan2(self.a),
        }
    }

    pub fn delta_l(self, other: Oklab) -> f64 {
        self.l - other.l
    }
    pub fn delta_a(self, other: Oklab) -> f64 {
        self.a - other.a
    }
    pub fn delta_b(self, other: Oklab) -> f64 {
        self.b - other.b
    }
    pub fn delta_c(self, other: Oklab) -> f64 {
        self.a.hypot(self.b) - other.a.hypot(other.b)
    }
    pub fn delta_h(self, other: Oklab) -> f64 {
        // Idea from svgeesus, being that we're finding the length of the angular arc between these two colors
        (self.delta_a(other).powi(2) + self.delta_b(other).powi(2) - self.delta_c(other).powi(2))
            .abs() // Absolute value since value might be negative because of subtraction
            .sqrt()
    }

    pub fn delta_eok(self, other: Oklab) -> f64 {
        // Euclidian distance color difference formula
        // svgeesus' idea was to use the delta_l, delta_c, and delta_h functions, but it reduces to this anyways
        (self.delta_l(other).powi(2) + self.delta_a(other).powi(2) + self.delta_b(other).powi(2))
            .sqrt()
    }

    pub fn delta_hyab(self, other: Oklab) -> f64 {
        // Hybrid absolute and Euclidian distance formula
        // Shown to be better for large color differences compared to DE2000 for CIELAB, unknown for Oklab
        self.delta_l(other).abs()
            + (self.delta_a(other).powi(2) + self.delta_b(other).powi(2)).sqrt()
    }

    /*
    pub fn delta_eok_original(self, other: Oklab) -> f64 {
        // Here for posterity, is slower than delta_eok()
        // svgeesus' idea was to use this, but it reduces to delta_eok() anyways
        (self.delta_l(other).powi(2) + self.delta_c(other).powi(2) + self.delta_h(other).powi(2))
            .sqrt()
    }
    */

    pub fn oklab_to_srgb_closest(self) -> SRgb {
        // Finds the SRgb value that is closest to the given Oklab
        // HyAB is shown to maintain L, but chroma and hue may shift

        let saved_delta = RwLock::new(f64::MAX);
        let saved_color = Mutex::new(SRgb { r: 0, g: 0, b: 0 });

        // Despite parallelization, this is still rather slow
        AllSRgb::default()
            .par_bridge()
            .map(|thing| thing.srgb_to_oklab())
            .for_each(|sample| {
                let delta = self.delta_hyab(sample);

                if delta < *saved_delta.read() {
                    *saved_delta.write() = delta;
                    *saved_color.lock() = sample.oklab_to_srgb();
                }
            });

        saved_color.into_inner()
    }

    pub fn oklab_to_srgb_clamp_c(self) -> SRgb {
        // Finds the SRgb value that is closest to the given Oklab
        // Chroma is restricted by a multiplier less than or equal to 1
        // Maintains L and hue, chroma can change
        let self_oklch = self.oklab_to_oklch();
        let mut mult = 0.5;

        for exp in 2..=52 {
            let sample = Oklch {
                c: self_oklch.c * mult,
                ..self_oklch
            }
            .oklch_to_oklab()
            .oklab_to_lrgb();

            // Within 1/4th of a pixel value, probably fine for this purpose
            // f64::EPSILON is too strict
            if sample.min() < -1.0 / (4.0 * 255.0) || sample.min() > 1.0_f64 + 1.0 / (4.0 * 255.0) {
                mult -= 2.0_f64.powi(-exp);
            } else {
                mult += 2.0_f64.powi(-exp);
            }
        }

        let output = Oklch {
            c: self_oklch.c * mult,
            ..self_oklch
        }
        .oklch_to_oklab()
        .oklab_to_srgb();

        output
    }
}

use std::ops::{Add, Div, Mul, Sub};
impl Add<Oklab> for Oklab {
    type Output = Oklab;

    fn add(self, other: Oklab) -> Oklab {
        Oklab {
            l: self.l + other.l,
            a: self.a + other.a,
            b: self.b + other.b,
        }
    }
}
impl Add<f64> for Oklab {
    type Output = Oklab;

    fn add(self, other: f64) -> Oklab {
        Oklab {
            l: self.l + other,
            a: self.a + other,
            b: self.b + other,
        }
    }
}
impl Sub<Oklab> for Oklab {
    type Output = Oklab;

    fn sub(self, other: Oklab) -> Oklab {
        Oklab {
            l: self.l - other.l,
            a: self.a - other.a,
            b: self.b - other.b,
        }
    }
}
impl Sub<f64> for Oklab {
    type Output = Oklab;

    fn sub(self, other: f64) -> Oklab {
        Oklab {
            l: self.l - other,
            a: self.a - other,
            b: self.b - other,
        }
    }
}
impl Mul<Oklab> for Oklab {
    type Output = Oklab;

    fn mul(self, other: Oklab) -> Oklab {
        Oklab {
            l: self.l * other.l,
            a: self.a * other.a,
            b: self.b * other.b,
        }
    }
}
impl Mul<f64> for Oklab {
    type Output = Oklab;

    fn mul(self, other: f64) -> Oklab {
        Oklab {
            l: self.l * other,
            a: self.a * other,
            b: self.b * other,
        }
    }
}
impl Div<Oklab> for Oklab {
    type Output = Oklab;

    fn div(self, other: Oklab) -> Oklab {
        Oklab {
            l: self.l / other.l,
            a: self.a / other.a,
            b: self.b / other.b,
        }
    }
}
impl Div<f64> for Oklab {
    type Output = Oklab;

    fn div(self, other: f64) -> Oklab {
        Oklab {
            l: self.l / other,
            a: self.a / other,
            b: self.b / other,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Oklch {
    pub l: f64,
    pub c: f64,
    pub h: f64,
}

impl Oklch {
    pub fn oklch_to_oklab(self) -> Oklab {
        Oklab {
            l: self.l,
            a: self.c * self.h.cos(),
            b: self.c * self.h.sin(),
        }
    }

    pub fn oklch_to_srgb(self) -> SRgb {
        self.oklch_to_oklab().oklab_to_srgb()
    }
}
