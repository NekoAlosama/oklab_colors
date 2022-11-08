#![allow(unused_imports)]
use itertools::Itertools;
use oklab::Oklab;
use rayon::prelude::*;
use rgb::SRgb;

mod oklab;
mod rgb;

const N: usize = 8; // factors of 256: 1, 2, 4, 8, 16, 32, 64, 128, 256.

fn main() {
    let start_time = std::time::SystemTime::now();

    let background = SRgb { r: 0, g: 0, b: 0 }.srgb_to_oklab();

    let mut srgb_colors: Vec<Oklab> = Vec::with_capacity(2_usize.pow(8).pow(3));
    for r in (0..=256_usize).step_by(N) {
        for g in (0..=256_usize).step_by(N) {
            for b in (0..=256_usize).step_by(N) {
                srgb_colors.push(
                    SRgb {
                        r: r.saturating_sub(1) as u8,
                        g: g.saturating_sub(1) as u8,
                        b: b.saturating_sub(1) as u8,
                    }
                    .srgb_to_oklab(),
                );
            }
        }
    }

    let saved = std::sync::Mutex::new({
        (
            f64::MIN,
            f64::MIN,
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
        )
    });

    srgb_colors
        .clone()
        .par_iter()
        .filter(|&color1| color1.delta_eok(background) > saved.lock().unwrap().0)
        .for_each(|color1| {
            srgb_colors
                .clone()
                .par_iter()
                .filter(|&color2| color2.delta_eok(background) > saved.lock().unwrap().0)
                .filter(|&color2| color2.delta_eok(*color1) > saved.lock().unwrap().0)
                .map(|color2| compute_job(background, *color1, *color2))
                .for_each(|item| {
                    if item.0 > saved.lock().unwrap().0 {
                        *saved.lock().unwrap() = item;
                    }
                })
        });

    let saved = saved.lock().unwrap();
    println!("{N:?}");
    println!("{saved:?}");
    println!("{:?}, {:?}", saved.0, saved.1);
    println!(
        "{:?}, {:?}",
        saved.2.oklab_to_srgb(),
        saved.3.oklab_to_srgb(),
    );

    println!(
        "Time: {:?}",
        start_time.elapsed().expect("Time went backwards")
    );
}

// c1, c2, lowest difference with each other, sum of differences
fn compute_job(bg: Oklab, c1: Oklab, c2: Oklab) -> (f64, f64, Oklab, Oklab) {
    let delta1 = c1.delta_eok(bg);

    let delta2 = c2.delta_eok(bg);
    let delta3 = c2.delta_eok(c1);
    (
        delta1.min(delta2).min(delta3),
        delta1 + delta2 + delta3,
        c1,
        c2,
    )
}