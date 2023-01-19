#![allow(dead_code)]
use crate::oklab::*;

// Implementation from the rgb crate, modified for personal use
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

#[derive(Copy, Clone, Debug)]
pub struct AllSRgb {
    next_color: SRgb,
    stop: bool,
}

// Iterator that goes over the whole RGB gamut, starting at a particular value and continuing
// (0,0,0), (0,0,1), (0,0,2)... (0,0,255), (0,1,0)... (255,255,255), None
impl Iterator for AllSRgb {
    type Item = SRgb;

    fn next(&mut self) -> Option<Self::Item> {
        // Save our curent color to return later
        let current_color = self.next_color;

        if self.stop {
            return None
        }

        // Increment b by 1, then increrment g if b exceeds 255, then increment r if g exceeds 255, and then stop == true if r exceeds 255
        let mut r = (current_color.r, false);
        let mut g = (current_color.g, false);
        let b = current_color.b.overflowing_add(1);

        // Will be all true if current_color == SRgb { r: 255, g: 255, b: 255 }, so calling .next() after should return nothing
        if b.1 {
            g = current_color.g.overflowing_add(1);
            if g.1 {
                r = current_color.r.overflowing_add(1);
                if r.1 {
                    self.stop = true;
                }
            }
        }

        self.next_color = SRgb { r: r.0, g: g.0, b: b.0 };


        Some(current_color)
    }
}

// Didn't implement new() since I can't really start with SRgb { r: 0, g: 0, b: -1 }
impl Default for AllSRgb {
    fn default() -> Self {
        Self {
            next_color: SRgb { r: 0, g: 0, b: 0 },
            stop: false,
        }
    }
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

    pub fn srgb_to_oklab(self) -> Oklab {
        self.srgb_to_lrgb().lrgb_to_oklab()
    }

    pub fn srgb_to_oklch(self) -> Oklch {
        self.srgb_to_lrgb().lrgb_to_oklab().oklab_to_oklch()
    }

    pub fn min(self) -> u8 {
        self.r.min(self.g).min(self.b)
    }

    pub fn max(self) -> u8 {
        self.r.max(self.g).max(self.b)
    }
}

impl Rgb<f64> {
    // Note: This is not a good way to clamp sRGB colors
    // The .clamp() is only to prevent over/underflows from rounding errors
    pub fn lrgb_to_srgb(self) -> SRgb {
        Rgb {
            r: ((255.0 * to_gamma(self.r)).round()).clamp(0.0, 255.0) as u8,
            g: ((255.0 * to_gamma(self.g)).round()).clamp(0.0, 255.0) as u8,
            b: ((255.0 * to_gamma(self.b)).round()).clamp(0.0, 255.0) as u8,
        }
    }

    pub fn lrgb_to_oklab(self) -> Oklab {
        let l = 0.4122214708 * self.r + 0.5363325363 * self.g + 0.0514459929 * self.b;
        let m = 0.2119034982 * self.r + 0.6806995451 * self.g + 0.1073969566 * self.b;
        let s = 0.0883024619 * self.r + 0.2817188376 * self.g + 0.6299787005 * self.b;
        let l_ = l.cbrt();
        let m_ = m.cbrt();
        let s_ = s.cbrt();
        Oklab {
            l: 0.2104542553 * l_ + 0.793617785 * m_ - 0.0040720468 * s_,
            a: 1.9779984951 * l_ - 2.428592205 * m_ + 0.4505937099 * s_,
            b: 0.0259040371 * l_ + 0.7827717662 * m_ - 0.808675766 * s_,
        }
    }

    pub fn min(self) -> f64 {
        self.r.min(self.g).min(self.b)
    }

    pub fn max(self) -> f64 {
        self.r.max(self.g).max(self.b)
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
