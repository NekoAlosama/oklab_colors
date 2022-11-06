#![allow(unused_imports)]
use itertools::Itertools;
use oklab::Oklab;
use rayon::prelude::*;
use rgb::SRgb;

mod oklab;
mod rgb;

fn main() {
    let mut start_time = std::time::SystemTime::now();

    let background = SRgb { r: 0, g: 0, b: 0 }.srgb_to_oklab();

    let mut srgb_colors: Vec<Oklab> = Vec::with_capacity(2_usize.pow(8).pow(3));
    for r in 0..=255 {
        for g in 0..=255 {
            for b in 0..=255 {
                srgb_colors.push(SRgb { r, g, b }.srgb_to_oklab());
            }
        }
    }

    let mut lowest = f64::MIN;
    let mut saved: (Oklab, Oklab, f64, f64) = (
        Oklab {
            l: f64::NAN,
            a: f64::NAN,
            b: f64::NAN,
        },
        Oklab {
            l: f64::NAN,
            a: f64::NAN,
            b: f64::NAN,
        },
        f64::MIN,
        f64::MIN,
    );

    let mut best_colors = srgb_colors.clone().into_par_iter().map(|color1| {
        srgb_colors
            .clone()
            .into_par_iter()
            .map(|color2| compute_job(background, color1, color2))
            .for_each_with((lowest, saved), |ini, item| {
                if item.2 > ini.0 {
                    ini.0 = item.2;
                    ini.1 = item;
                }
            })
    });

    println!("{saved:?}");
    println!("{lowest:?}");

    println!(
        "Time: {:?}",
        start_time.elapsed().expect("Time went backwards")
    );
}

// c1, c2, lowest difference with each other, sum of differences
fn compute_job(bg: Oklab, c1: Oklab, c2: Oklab) -> (Oklab, Oklab, f64, f64) {
    let mut delta1 = c1.delta_eok(bg);
    let mut delta2 = c2.delta_eok(bg);
    let mut delta3 = c1.delta_eok(c2);
    (
        c1,
        c2,
        delta1.min(delta2).min(delta3),
        delta1 + delta2 + delta3,
    )
}

/*
Notes:
Plan:

Black + 1:
SRgb { r: 255, g: 255, b: 255}
0.9999999934735468
*/
