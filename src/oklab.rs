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
            .sqrt()
    }

    pub fn delta_eok(self, other: Oklab) -> f64 {
        // Color difference formula
        // svgeesus' idea was to use the delta_l, delta_c, and delta_h functions, but it reduces to a normal Euclidian distance
        (self.delta_l(other).powi(2) + self.delta_a(other).powi(2) + self.delta_b(other).powi(2))
            .sqrt()
    }

    /*
    fn delta_eok_original(self, other: Oklab) -> f64 {
        // Here for posterity
        // Do not use as delta_h() can give you NaN, messing up fold() operations
        (self.delta_l(other).powi(2) + self.delta_c(other).powi(2) + self.delta_h(other).powi(2)).sqrt()
    }
    */
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
}
