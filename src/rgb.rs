// Implementation from the rgb crate, modified for personal use
#![allow(non_camel_case_types)]

use itertools;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct sRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct lRGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl std::fmt::Display for sRGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}
impl std::fmt::Display for lRGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Default for sRGB {
    fn default() -> sRGB {
        sRGB { r: 0, g: 0, b: 0 }
    }
}

impl Default for lRGB {
    fn default() -> lRGB {
        lRGB {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl sRGB {
    pub fn to_lrgb(self) -> lRGB {
        lRGB {
            r: to_linear(self.r as f64 / 255.0),
            g: to_linear(self.g as f64 / 255.0),
            b: to_linear(self.b as f64 / 255.0),
        }
    }

    pub fn min(self) -> u8 {
        self.r.min(self.g).min(self.b)
    }
    pub fn max(self) -> u8 {
        self.r.max(self.g).max(self.b)
    }

    // itertools calls this the cartesian product: (0,0,0),(0,0,1),...(0,0,255),(0,1,0),...(255,255,254),(255,255,255)
    pub fn all_colors() -> impl Iterator<Item = sRGB> + Clone {
        itertools::iproduct!(0..=255, 0..=255, 0..=255).map(|(r, g, b)| sRGB { r, g, b })
    }
}

impl lRGB {
    // Note: This is not a good way to clamp sRGB colors
    // The .clamp() is only to prevent over/underflows from rounding errors
    pub fn to_srgb(self) -> sRGB {
        sRGB {
            r: ((255.0 * to_gamma(self.r)).round()).clamp(0.0, 255.0) as u8,
            g: ((255.0 * to_gamma(self.g)).round()).clamp(0.0, 255.0) as u8,
            b: ((255.0 * to_gamma(self.b)).round()).clamp(0.0, 255.0) as u8,
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
        (u.mul_add(200.0, 11.0)).powf(2.4)
    } else {
        u / 12.92
    }
}

fn to_gamma(u: f64) -> f64 {
    if u >= 0.0031308 {
        u.powf(1.0 / 2.4).mul_add(1.055, -0.055)
    } else {
        12.92 * u
    }
}
