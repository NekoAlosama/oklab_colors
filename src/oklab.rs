#![allow(dead_code)]
use crate::rgb::*;
use parking_lot::Mutex;
use rayon::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Oklab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

const K_1: f64 = 0.206;
const K_2: f64 = 0.03;
const K_3: f64 = (1.0 + K_1) / (1.0 + K_2);

impl Oklab {
    pub const BLACK: Oklab = Oklab {
        l: 0.0,
        a: 0.0,
        b: 0.0,
    };

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
    
    pub fn ref_l(self) -> Oklab {
        Oklab {
            l: (K_3 * self.l - K_1
                + ((K_3 * self.l - K_1).powi(2) + 4.0 * K_2 * K_3 * self.l).sqrt())
                / 2.0,
            ..self
        }
    }

    pub fn unref_l(self) -> Oklab {
        Oklab {
            l: (self.l * (self.l + K_1)) / (K_3 * (self.l + K_2)),
            ..self
        }
    }

    pub fn mix(self, other: Oklab) -> Oklab {
        // Linear mixing
        (self + other) / 2.0
    }

    pub fn chroma(self) -> f64 {
        self.a.hypot(self.b)
    }
    pub fn hue(self) -> f64 {
        // Returns hue angle
        self.b.atan2(self.a)
    }
    pub fn oklab_to_oklch(self) -> Oklch {
        Oklch {
            l: self.l,
            c: self.chroma(),
            h: self.hue(),
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
        // The difference in the amount of chroma
        // NOT the Euclidian distance between the two (a,b) pairs (see delta_h_alt)
        self.chroma() - other.chroma()
    }

    pub fn delta_h(self, other: Oklab) -> f64 {
        // DE94 formula
        // Returns 0.0 if using colors with no chroma (in this case, check if chroma is good enough)
        (self.delta_a(other).powi(2) + self.delta_b(other).powi(2) - self.delta_c(other).powi(2))
            .abs() // Absolute value since value might be negative because of subtraction
            .sqrt()
    }

    pub fn delta_h_relative(self, other: Oklab) -> f64 {
        // Multiplies delta_h() by a relative multiplier
        // self is the reference color, and other is the sample color
        self.delta_h(other) * (other.chroma() / (self.a.powi(2) + self.b.powi(2)))
    }

    pub fn delta_e_eok(self, other: Oklab) -> f64 {
        // Euclidian distance color difference formula
        // Value range: 0.0 - 1.0 (black vs. white)
        (self.delta_l(other).powi(2) + self.delta_a(other).powi(2) + self.delta_b(other).powi(2))
            .sqrt()
    }

    pub fn delta_e_hyab(self, other: Oklab) -> f64 {
        // Hybrid absolute and Euclidian distance formula
        // Shown to be better for large color differences compared to DE2000 for CIELAB, unknown for Oklab
        // Higher weight towards L differences
        // Value range: 0.0 - 1.178988628052311 (black vs. yellow gives upper bound; black vs. white gives 1.0)
        self.delta_l(other).abs() + self.delta_a(other).hypot(self.delta_b(other))
    }

    pub fn oklab_to_srgb_closest(self) -> SRgb {
        // Finds the SRgb value that is closest to the given Oklab

        // Early exit; should work
        if self.oklab_to_lrgb().min() > -f64::EPSILON
            && self.oklab_to_lrgb().max() < 1.0 + f64::EPSILON
        {
            return self.oklab_to_srgb();
        }

        let saved_delta = Mutex::new(f64::MAX);
        let saved_color = Mutex::new(SRgb { r: 0, g: 0, b: 0 });

        // Despite parallelization, this is still rather slow
        SRgb::all_colors()
            .par_bridge()
            .map(|thing| thing.srgb_to_oklab())
            .for_each(|sample| {
                let delta = self.delta_e_hyab(sample);

                let mut locked_saved_delta = saved_delta.lock();
                let mut locked_saved_color = saved_color.lock();

                if delta < *locked_saved_delta {
                    *locked_saved_delta = delta;
                    *locked_saved_color = sample.oklab_to_srgb();
                }
            });

        saved_color.into_inner()
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

    pub fn oklch_to_srgb_closest(self) -> SRgb {
        self.oklch_to_oklab().oklab_to_srgb_closest()
    }

    pub fn mix(self, other: Oklch) -> Oklch {
        // Linear mixing
        // Properly mixes hue
        Oklch {
            l: (self.l + other.l) / 2.0,
            c: (self.c + other.c) / 2.0,
            // Mixes according to the shortest path
            h: match (self.h - other.h).abs() < std::f64::consts::PI {
                true => (self.h + other.h) / 2.0,
                false => (self.h + other.h) / 2.0 + std::f64::consts::PI,
            },
        }
    }
}

impl Add<Oklch> for Oklch {
    type Output = Oklch;

    fn add(self, other: Oklch) -> Oklch {
        Oklch {
            l: self.l + other.l,
            c: self.c + other.c,
            h: self.h + other.h,
        }
    }
}
impl Add<f64> for Oklch {
    type Output = Oklch;

    fn add(self, other: f64) -> Oklch {
        Oklch {
            l: self.l + other,
            c: self.c + other,
            h: self.h + other,
        }
    }
}
impl Sub<Oklch> for Oklch {
    type Output = Oklch;

    fn sub(self, other: Oklch) -> Oklch {
        Oklch {
            l: self.l - other.l,
            c: self.c - other.c,
            h: self.h - other.h,
        }
    }
}
impl Sub<f64> for Oklch {
    type Output = Oklch;

    fn sub(self, other: f64) -> Oklch {
        Oklch {
            l: self.l - other,
            c: self.c - other,
            h: self.h - other,
        }
    }
}
impl Mul<Oklch> for Oklch {
    type Output = Oklch;

    fn mul(self, other: Oklch) -> Oklch {
        Oklch {
            l: self.l * other.l,
            c: self.c * other.c,
            h: self.h * other.h,
        }
    }
}
impl Mul<f64> for Oklch {
    type Output = Oklch;

    fn mul(self, other: f64) -> Oklch {
        Oklch {
            l: self.l * other,
            c: self.c * other,
            h: self.h * other,
        }
    }
}
impl Div<Oklch> for Oklch {
    type Output = Oklch;

    fn div(self, other: Oklch) -> Oklch {
        Oklch {
            l: self.l / other.l,
            c: self.c / other.c,
            h: self.h / other.h,
        }
    }
}
impl Div<f64> for Oklch {
    type Output = Oklch;

    fn div(self, other: f64) -> Oklch {
        Oklch {
            l: self.l / other,
            c: self.c / other,
            h: self.h / other,
        }
    }
}
