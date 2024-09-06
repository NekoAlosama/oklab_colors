#![allow(dead_code)]

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// Implementation of RGB colors from the `rgb`` crate, modified for personal use.
///
/// Standard RGB color.
#[allow(non_camel_case_types)]
pub struct sRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// Linear light RGB color.
#[allow(non_camel_case_types)]
pub struct lRGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl std::fmt::Display for sRGB {
    /// Display as an sRGB tuple: `(123, 45, 6)`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sRGB({}, {}, {})", self.r, self.g, self.b)
    }
}
impl std::fmt::Display for lRGB {
    /// Display as an lRGB tuple: `(0.123, 0.45, 0.6)`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lRGB({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Default for sRGB {
    /// Default to pure black: `(0, 0, 0)`.
    fn default() -> sRGB {
        sRGB { r: 0, g: 0, b: 0 }
    }
}

impl Default for lRGB {
    /// Default to pure black: `(0.0, 0.0, 0.0)`.
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
            r: linearize(self.r as f64 / 255.0),
            g: linearize(self.g as f64 / 255.0),
            b: linearize(self.b as f64 / 255.0),
        }
    }

    pub fn min(self) -> u8 {
        self.r.min(self.g).min(self.b)
    }
    pub fn max(self) -> u8 {
        self.r.max(self.g).max(self.b)
    }

    pub fn all_colors() -> impl Iterator<Item = sRGB> + Clone + Send {
        // itertools calls this the cartesian product: (0,0,0),(0,0,1),...(0,0,255),(0,1,0),...(255,255,254),(255,255,255)
        itertools::iproduct!(0..=255, 0..=255, 0..=255).map(|(r, g, b)| sRGB { r, g, b })
    }
}

impl lRGB {
    // Note: This is not a good way to clamp sRGB colors
    // The .clamp() is only to prevent over/underflows from rounding errors
    pub fn to_srgb(self) -> sRGB {
        sRGB {
            r: ((255.0 * gamma(self.r)).round()).clamp(0.0, 255.0) as u8,
            g: ((255.0 * gamma(self.g)).round()).clamp(0.0, 255.0) as u8,
            b: ((255.0 * gamma(self.b)).round()).clamp(0.0, 255.0) as u8,
        }
    }

    pub fn min(self) -> f64 {
        self.r.min(self.g).min(self.b)
    }

    pub fn max(self) -> f64 {
        self.r.max(self.g).max(self.b)
    }
}

/// Undoes gamma correction to convert from sRGB to lRGB
pub fn linearize(u: f64) -> f64 {
    if u >= 0.04045 {
        ((u + 0.055) / 1.055).powf(2.4)
    } else {
        u / 12.92
    }
}

/// Applies gamma correction to convert from lRGB to sRGB
pub fn gamma(u: f64) -> f64 {
    if u >= 0.0031308 {
        u.powf(1.0 / 2.4).mul_add(1.055, -0.055)
    } else {
        12.92 * u
    }
}

#[cfg(test)]
mod tests {
    use crate::rgb;

    #[test]
    fn gamma_to_linear() {
        let test = 0.5_f64;
        assert!(rgb::gamma(rgb::linearize(test)) - 0.5 < 1e-6);
    }

    #[test]
    fn srgb_to_lrgb() {
        assert_eq!(
            rgb::sRGB {
                r: 255,
                g: 128,
                b: 127
            },
            (rgb::sRGB {
                r: 255,
                g: 128,
                b: 127
            })
            .to_lrgb()
            .to_srgb()
        );
    }
}
