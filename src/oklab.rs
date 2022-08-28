#![allow(dead_code)]
use crate::rgb;

#[derive(Copy, Clone, Debug)]
pub struct Oklab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

impl Oklab {
    pub fn oklab_to_lrgb(self) -> rgb::LRgb {
        let l_ = self.l + 0.3963377774 * self.a + 0.2158037573 * self.b;
        let m_ = self.l - 0.1055613458 * self.a - 0.0638541728 * self.b;
        let s_ = self.l - 0.0894841775 * self.a - 1.291485548 * self.b;
        let l = l_.powi(3);
        let m = m_.powi(3);
        let s = s_.powi(3);
        rgb::Rgb {
            r: 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
            g: -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
            b: -0.0041960863 * l - 0.7034186147 * m + 1.707614701 * s,
        }
    }

    pub fn oklab_to_srgb(self) -> rgb::SRgb {
        self.oklab_to_lrgb().lrgb_to_srgb()
    }

    pub fn oklab_difference(self, other: Oklab) -> f64 {
        // Euclidian distance as Oklab is supposed to be orthogonal
        ((self.l - other.l).powi(2) + (self.a - other.a).powi(2) + (self.b - other.b).powi(2))
            .sqrt()
    }
}
