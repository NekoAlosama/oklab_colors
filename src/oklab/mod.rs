#![allow(dead_code)]

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

impl std::fmt::Display for Oklab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Oklab({}, {}, {})", self.l, self.a, self.b)
    }
}
impl std::fmt::Display for Oklch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Oklch({}, {}, {})", self.l, self.c, self.h)
    }
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
    pub const BLACK: Oklab = Oklab {
        l: 0.0,
        a: 0.0,
        b: 0.0,
        d65_reference_l: false,
    };
    pub const WHITE: Oklab = Oklab {
        l: 1.0,
        a: 0.0,
        b: 0.0,
        d65_reference_l: false,
    };

    /// Oklab does not have a white point reference by default, which is supposed to improve its lightness accuracy.
    /// However, some applications do require a reference.
    /// Ottoson developed this D65 lightness estimate for use in a color picker, which is supposed to show all colors under a single hue.
    ///
    /// I'm unsure whether I should use this when iterating through all sRGB colors, as Oklab is a transformation of sRGB, but sRGB has the D65 white point.
    pub fn to_d65_white(self) -> Oklab {
        if self.d65_reference_l {
            return self;
        }
        Oklab {
            l: 0.1
                * (self
                    .l
                    .mul_add(36.3609 / 1.0609, -44.019 / 5.15)
                    .mul_add(self.l, 1.0609)
                    .sqrt()
                    + self.l.mul_add(6.03 / 1.03, -1.03)),
            d65_reference_l: true,
            ..self
        }
    }
    pub fn to_unreferenced_white(self) -> Oklab {
        if self.d65_reference_l {
            return Oklab {
                l: self.l * (self.l.mul_add(51.5, 10.609) / self.l.mul_add(60.3, 1.809)),
                d65_reference_l: false,
                ..self
            };
        }
        self
    }

    /// Perceived colorfulness against a gray background of the same lightness `self.l`
    pub fn chroma(self) -> f64 {
        self.a.hypot(self.b)
    }
    /// Hue angle, where `0.0` is on the positive `self.a` axis, representing redness.
    pub fn hue(self) -> f64 {
        self.b.atan2(self.a)
    }
    /// Officially undefined, the following is an interpretation of saturation.
    ///
    /// The common idea of saturation is chroma relative to lightness, so use `self.chroma() / self.l` if you think it's better.
    ///
    /// This interpretation is that saturation is chroma relative to "total perceived color sensation", or chroma and lightness combined in some way.
    /// In this case, `self.delta_E_ab(Oklab::BLACK)` is considered to be chroma and lightness combined. However, delta_E_Hyab might be similar
    /// Note that there isn't a definition for "relative lightness".
    pub fn saturation(self) -> (f64, f64) {
        (
            self.chroma() / self.delta_E_ab(Oklab::BLACK),
            self.chroma() / self.delta_E_Hyab(Oklab::BLACK),
        )
    }

    /// Not to be confused with delta lowercase h, meaning difference in hue angles.
    /// Use `self.hue() - other.hue()`, maybe with `.abs()`, instead.
    ///
    /// Signed hue contribution delta.
    /// This is supposed to be the mean of the chroma cord lengths, if looking at the colors on a `self.a` vs `self.b` graph.
    ///
    /// I have no idea what you're supposed to do with this by itself, but it can be used in an alternative way to compute `delta_E_ab`.
    #[allow(non_snake_case)]
    pub fn delta_H(self, other: Oklab) -> f64 {
        // DE94 formula
        2.0 * (self.chroma() * other.chroma()).sqrt() * ((self.hue() - other.hue()) / 2.0).sin()
    }

    /// Euclidian distance formula.
    ///
    /// Highest delta_E_ab generated from pure black vs. pure white.
    #[allow(non_snake_case)]
    pub fn delta_E_ab(self, other: Oklab) -> f64 {
        let l_difference = self.l - other.l;
        let a_difference = self.a - other.a;
        let b_difference = self.b - other.b;
        (l_difference.mul_add(
            l_difference,
            a_difference.mul_add(a_difference, b_difference.powi(2)),
        ))
        .sqrt()
    }

    /// Hybrid taxicab/Manhattan and Euclidian distances formula.
    /// Shown to provide better agreement on colors with high difference values for CIELAB.
    ///
    /// Highest delta_E_Hyab generated from pure black vs. pure yellow.
    // I wonder how delta_l + delta_c is compared to delta_l + sqrt(delta_a^2 + delta_b^2)
    #[allow(non_snake_case)]
    pub fn delta_E_Hyab(self, other: Oklab) -> f64 {
        let l_difference = self.l - other.l;
        let a_difference = self.a - other.a;
        let b_difference = self.b - other.b;
        l_difference.abs() + (a_difference.mul_add(a_difference, b_difference.powi(2))).sqrt()
    }

    pub fn to_oklch(self) -> Oklch {
        Oklch {
            l: self.l,
            c: self.chroma(),
            h: self.hue(),
            d65_reference_l: self.d65_reference_l,
        }
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

    /// Plain RGB clipping.
    /// You might want to use the other to_srgb_* functions.
    pub fn to_srgb(self) -> rgb::sRGB {
        self.to_lrgb().to_srgb()
    }
    /// Finds the sRGB value that is closest to the given Oklab. Very slow.
    pub fn to_srgb_closest(self) -> rgb::sRGB {
        // Early exit; should work
        if (rgb::gamma(self.to_lrgb().min()) >= -1e-6)
            && (rgb::gamma(self.to_lrgb().max()) <= 1.0 + 1e-6)
        {
            return self.to_srgb();
        }

        let saved_delta = Mutex::new(f64::MAX);
        let saved_color = Mutex::new(rgb::sRGB { r: 0, g: 0, b: 0 });

        // Despite parallelization, this is still rather slow
        rgb::sRGB::all_colors()
            .par_bridge()
            .map(|thing| thing.to_oklab())
            .for_each(|sample| {
                let delta = self.delta_E_ab(sample);
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
    /// Finds the sRGB value that is farthest away to the given Oklab.
    pub fn to_srgb_contrast(self) -> rgb::sRGB {
        let saved_delta = Mutex::new(f64::MIN);
        let saved_color = Mutex::new(rgb::sRGB { r: 0, g: 0, b: 0 });

        // All of these colors are known to be the 1-bit values
        itertools::iproduct!([0, 255], [0, 255], [0, 255])
            .map(|(r, g, b)| rgb::sRGB { r, g, b })
            .par_bridge()
            .map(|thing| thing.to_oklab())
            .for_each(|sample| {
                let delta = self.delta_E_Hyab(sample);
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
            l: l_.mul_add(0.2104542553, m_.mul_add(0.793617785, -0.0040720468 * s_)),
            a: l_.mul_add(1.9779984951, m_.mul_add(-2.428592205, 0.4505937099 * s_)),
            b: l_.mul_add(0.0259040371, m_.mul_add(0.7827717662, -0.808675766 * s_)),
            d65_reference_l: false,
        }
    }
    pub fn to_oklch(self) -> Oklch {
        self.to_oklab().to_oklch()
    }
}

#[cfg(test)]
mod tests {
    use crate::oklab;
    use crate::rgb;

    const DIFFERENCE: f64 = 1e-6;

    #[test]
    fn lrgb_to_oklab() {
        let test = (rgb::lRGB {
            r: 1.0,
            g: 0.5,
            b: 0.25,
        })
        .to_oklab()
        .to_lrgb();
        assert!((test.r - 1.0).abs() < DIFFERENCE);
        assert!((test.g - 0.5).abs() < DIFFERENCE);
        assert!((test.b - 0.25).abs() < DIFFERENCE);
    }

    #[test]
    fn oklab_to_oklch() {
        let test = (oklab::Oklab {
            l: 0.5,
            a: 0.25,
            b: 0.125,
            d65_reference_l: false,
        })
        .to_oklch()
        .to_oklab();
        assert!((test.l - 0.5).abs() < DIFFERENCE);
        assert!((test.a - 0.25).abs() < DIFFERENCE);
        assert!((test.b - 0.125).abs() < DIFFERENCE);
        assert_eq!(test.d65_reference_l, false);
    }

    #[test]
    fn unreferenced_white_to_d65_white() {
        let test = (oklab::Oklab {
            l: 0.5,
            a: 0.25,
            b: 0.125,
            d65_reference_l: false,
        })
        .to_d65_white()
        .to_unreferenced_white();
        assert!((test.l - 0.5).abs() < DIFFERENCE);
        assert!((test.a - 0.25).abs() < DIFFERENCE);
        assert!((test.b - 0.125).abs() < DIFFERENCE);
        assert_eq!(test.d65_reference_l, false);
    }
}
