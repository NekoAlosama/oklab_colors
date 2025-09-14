use std::f64::consts::PI;

mod oklab;
mod rgb;
use crate::oklab::*;
use crate::rgb::*;
use itertools::Itertools;
use parking_lot::Mutex;
use rayon::prelude::*;
fn main() {
    let start_time = std::time::SystemTime::now();

    let mut saved_colors: Vec<sRGB> = vec![
    ];

    for _ in 1..=8 {
        let saved_delta = Mutex::new(f64::NEG_INFINITY);
        let saved_color = Mutex::new(sRGB::default());
        let starting_colors = saved_colors.iter().map(|color: &sRGB| color.to_oklch());

        sRGB::all_colors().par_bridge().for_each(|test_color| {
			if test_color.to_oklab().delta_E_ab(Oklab::BLACK) < 2.0/3.0 {
				return;
			}
			for start_color in starting_colors.clone() {
				let mut hue_diff = (test_color.to_oklch().h - start_color.h).abs(); 
				hue_diff = hue_diff.min(2.0*PI - hue_diff);

				if hue_diff < PI / (saved_colors.len() + 1) as f64 {
					return;
				}
			}

			let delta = test_color.to_oklab().saturation().1;

            let mut locked_saved_delta = saved_delta.lock();
            let mut locked_saved_color: parking_lot::lock_api::MutexGuard<'_, parking_lot::RawMutex, sRGB> = saved_color.lock();

            if (delta > *locked_saved_delta) && (!saved_colors.contains(&test_color)) {
                *locked_saved_delta = delta;
                *locked_saved_color = test_color;
            }
        });

        let saved_color = saved_color.into_inner();

        println!(
            "{saved_color}, sens: {:.5?}, diff: {:.5?}",
			saved_color.to_oklab().delta_E_ab(Oklab::BLACK),
            saved_delta.into_inner()
        );
        saved_colors.push(saved_color)
    }

    println!(
        "Total time: {:.3?}",
        start_time.elapsed().expect("Time went backwards")
    );
}

/*
	saturation().0
sRGB(181, 0, 255), sens: 0.66748, diff: 0.50526
sRGB(0, 178, 0), sens: 0.69885, diff: 0.34027
sRGB(251, 0, 0), sens: 0.67068, diff: 0.41035
sRGB(0, 138, 255), sens: 0.66689, diff: 0.31362
sRGB(234, 0, 167), sens: 0.67635, diff: 0.42321
sRGB(217, 120, 0), sens: 0.68575, diff: 0.23631
sRGB(128, 113, 255), sens: 0.66701, diff: 0.32014
sRGB(206, 242, 0), sens: 0.92819, diff: 0.23740

	saturation().1
sRGB(181, 0, 255), sens: 0.66748, diff: 0.45096
sRGB(0, 178, 0), sens: 0.69885, diff: 0.32214
sRGB(251, 0, 0), sens: 0.67068, diff: 0.37963
sRGB(0, 138, 255), sens: 0.66689, diff: 0.29925
sRGB(234, 0, 167), sens: 0.67635, diff: 0.38974
sRGB(217, 120, 0), sens: 0.68575, diff: 0.22997
sRGB(128, 113, 255), sens: 0.66701, diff: 0.30489
sRGB(206, 242, 0), sens: 0.92819, diff: 0.23098
*/