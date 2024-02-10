use crate::rgb;
use parking_lot::Mutex;
use rayon::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Oklab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
    pub d65_reference_l: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Oklch {
    pub l: f64,
    pub c: f64,
    pub h: f64,
    pub d65_reference_l: bool,
}

impl Default for Oklab {
    fn default() -> Self {
        Oklab {
            l: 0.0,
            a: 0.0,
            b: 0.0,
            d65_reference_l: false,
        }
    }
}
impl Default for Oklch {
    fn default() -> Oklch {
        Oklch {
            l: 0.0,
            c: 0.0,
            h: 0.0,
            d65_reference_l: false,
        }
    }
}

impl Oklab {
    pub fn to_d65_reference(self) -> Oklab {
        Oklab {
            l: self
                .l
                .mul_add(60300.0, -7519.0)
                .mul_add(self.l.mul_add(60300.0, -7519.0), 56015520.0)
                .sqrt()
                .mul_add(0.01, 603.0 * self.l)
                .mul_add(1.0 / 1030.0, -0.103),
            d65_reference_l: true,
            ..self
        }
    }
    pub fn from_d65_reference(self) -> Oklab {
        Oklab {
            l: (self.l.mul_add(51500.0, 10609.0) / self.l.recip().mul_add(1809.0, 60300.0)),
            d65_reference_l: false,
            ..self
        }
    }

    pub fn chroma(self) -> f64 {
        self.a.hypot(self.b)
    }
    pub fn hue(self) -> f64 {
        // Returns hue angle
        self.b.atan2(self.a)
    }

    pub fn delta_h(self, other: Oklab) -> f64 {
        // DE94 formula
        // Returns 0.0 if using colors with no chroma (in this case, check if chroma is good enough)
        ((self.a - other.a).powi(2) + (self.b - other.b).powi(2)
            - (self.chroma() - other.chroma()).powi(2))
        .abs() // Absolute value since value might be negative because of subtraction
        .sqrt()
    }
    pub fn delta_h_relative(self, other: Oklab) -> f64 {
        // Ongoing research: Multiplies delta_h() by a relative multiplier
        // self is the reference color, and other is the sample color
        self.delta_h(other) * (other.chroma() / (self.a.powi(2) + self.b.powi(2)))
    }

    pub fn delta_e(self, other: Oklab) -> f64 {
        // Euclidian distance color difference formula
        // Value range: 0.0 - 1.0 (black vs. white)
        ((self.l - other.l).powi(2) + (self.a - other.a).powi(2) + (self.b - other.b).powi(2))
            .sqrt()
    }

    pub fn to_lrgb(self) -> rgb::lRGB {
        let l_ = self
            .a
            .mul_add(0.3963377774, self.b.mul_add(0.2158037573, self.l));
        let m_ = self
            .a
            .mul_add(-0.1055613458, self.b.mul_add(-0.0638541728, self.l));
        let s_ = self
            .a
            .mul_add(-0.0894841775, self.b.mul_add(-1.291485548, self.l));
        let l = l_.powi(3);
        let m = m_.powi(3);
        let s = s_.powi(3);
        rgb::lRGB {
            r: l.mul_add(4.0767416621, m.mul_add(-3.3077115913, 0.2309699292 * s)),
            g: l.mul_add(-1.2684380046, m.mul_add(2.6097574011, -0.3413193965 * s)),
            b: l.mul_add(-0.0041960863, m.mul_add(-0.7034186147, 1.707614701 * s)),
        }
    }

    pub fn to_srgb(self) -> rgb::sRGB {
        // RGB clipping
        // You might want to use the other to_srgb_* functions
        self.to_lrgb().to_srgb()
    }

    pub fn to_oklch(self) -> Oklch {
        Oklch {
            l: self.l,
            c: self.chroma(),
            h: self.hue(),
            d65_reference_l: self.d65_reference_l,
        }
    }

    pub fn to_srgb_closest(self) -> rgb::sRGB {
        // Finds the sRGB value that is closest to the given Oklab

        // Early exit; should work
        if self.to_lrgb().min() > 0.0_f64.next_down() && self.to_lrgb().max() < 1.0_f64.next_up() {
            return self.to_srgb();
        }

        let saved_delta = Mutex::new(f64::MAX);
        let saved_color = Mutex::new(rgb::sRGB { r: 0, g: 0, b: 0 });

        // Despite parallelization, this is still rather slow
        rgb::sRGB::all_colors()
            .par_bridge()
            .map(|thing| thing.to_oklab())
            .for_each(|sample| {
                let delta = self.delta_e(sample);
                {
                    let mut locked_saved_delta = saved_delta.lock();
                    let mut locked_saved_color = saved_color.lock();

                    if delta < *locked_saved_delta {
                        *locked_saved_delta = delta;
                        *locked_saved_color = sample.to_srgb();
                    }
                }
            });

        saved_color.into_inner()
    }

    pub fn to_srgb_opposite(self) -> rgb::sRGB {
        // Finds the SRgb value that is very far away to the given Oklab

        let saved_delta = Mutex::new(f64::MIN);
        let saved_color = Mutex::new(rgb::sRGB { r: 0, g: 0, b: 0 });

        // All opposite colors are known to be the 1-bit values
        itertools::iproduct!([0, 255], [0, 255], [0, 255])
            .map(|(r, g, b)| rgb::sRGB { r, g, b })
            .par_bridge()
            .map(|thing| thing.to_oklab())
            .for_each(|sample| {
                let delta = self.delta_e(sample);
                {
                    let mut locked_saved_delta = saved_delta.lock();
                    let mut locked_saved_color = saved_color.lock();

                    if delta > *locked_saved_delta {
                        *locked_saved_delta = delta;
                        *locked_saved_color = sample.to_srgb();
                    }
                }
            });

        saved_color.into_inner()
    }
}

impl Oklch {
    pub fn to_oklab(self) -> Oklab {
        Oklab {
            l: self.l,
            a: self.c * self.h.cos(),
            b: self.c * self.h.sin(),
            d65_reference_l: self.d65_reference_l,
        }
    }

    pub fn to_srgb(self) -> rgb::sRGB {
        self.to_oklab().to_srgb()
    }

    pub fn to_srgb_closest(self) -> rgb::sRGB {
        self.to_oklab().to_srgb_closest()
    }
}

impl rgb::sRGB {
    pub fn to_oklab(self) -> Oklab {
        self.to_lrgb().to_oklab()
    }
    pub fn to_oklch(self) -> Oklch {
        self.to_lrgb().to_oklab().to_oklch()
    }
}

impl rgb::lRGB {
    pub fn to_oklab(self) -> Oklab {
        let l = self.r.mul_add(
            0.4122214708,
            self.g.mul_add(0.5363325363, 0.0514459929 * self.b),
        );
        let m = self.r.mul_add(
            0.2119034982,
            self.g.mul_add(0.6806995451, 0.1073969566 * self.b),
        );
        let s = self.r.mul_add(
            0.0883024619,
            self.g.mul_add(0.2817188376, 0.6299787005 * self.b),
        );
        let l_ = l.cbrt();
        let m_ = m.cbrt();
        let s_ = s.cbrt();
        Oklab {
            l: 0.2104542553 * l_ + 0.793617785 * m_ - 0.0040720468 * s_,
            a: 1.9779984951 * l_ - 2.428592205 * m_ + 0.4505937099 * s_,
            b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.808675766 * s_,
            d65_reference_l: false,
        }
    }
    pub fn to_oklch(self) -> Oklch {
        self.to_oklab().to_oklch()
    }
}
