#![allow(dead_code)]
use crate::oklab;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

pub type SRgb = Rgb<u8>;
pub type LRgb = Rgb<f64>;

impl Rgb<u8> {
    pub fn srgb_to_lrgb(self) -> LRgb {
        Rgb {
            r: to_linear(self.r as f64 / 255.0),
            g: to_linear(self.g as f64 / 255.0),
            b: to_linear(self.b as f64 / 255.0),
        }
    }

    pub fn srgb_to_oklab(self) -> oklab::Oklab {
        self.srgb_to_lrgb().lrgb_to_oklab()
    }

    pub fn srgb_to_oklch(self) -> oklab::Oklch {
        self.srgb_to_lrgb().lrgb_to_oklab().oklab_to_oklch()
    }
}

impl Rgb<f64> {
    pub fn lrgb_to_srgb(self) -> SRgb {
        Rgb {
            r: ((255.0 * to_gamma(self.r)).round()).clamp(0.0, 255.0) as u8,
            g: ((255.0 * to_gamma(self.g)).round()).clamp(0.0, 255.0) as u8,
            b: ((255.0 * to_gamma(self.b)).round()).clamp(0.0, 255.0) as u8,
        }
    }

    pub fn lrgb_to_oklab(self) -> oklab::Oklab {
        let l = 0.4122214708 * self.r + 0.5363325363 * self.g + 0.0514459929 * self.b;
        let m = 0.2119034982 * self.r + 0.6806995451 * self.g + 0.1073969566 * self.b;
        let s = 0.0883024619 * self.r + 0.2817188376 * self.g + 0.6299787005 * self.b;
        let l_ = l.cbrt();
        let m_ = m.cbrt();
        let s_ = s.cbrt();
        oklab::Oklab {
            l: 0.2104542553 * l_ + 0.793617785 * m_ - 0.0040720468 * s_,
            a: 1.9779984951 * l_ - 2.428592205 * m_ + 0.4505937099 * s_,
            b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.808675766 * s_,
        }
    }
}

fn to_linear(u: f64) -> f64 {
    if u >= 0.04045 {
        ((u + 0.055) / (1.055)).powf(2.4)
    } else {
        u / 12.92
    }
}

fn to_gamma(u: f64) -> f64 {
    if u >= 0.0031308 {
        1.055 * u.powf(1.0 / 2.4) - 0.055
    } else {
        12.92 * u
    }
}
