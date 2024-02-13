use crate::oklab::*;
use crate::rgb::*;
use itertools::Itertools;
use parking_lot::Mutex;
use rayon::prelude::*;

pub fn main() {
    let start_time = std::time::SystemTime::now();

    let mut saved_colors = vec![sRGB { r: 0, g: 0, b: 0 }];

    for _ in 1..=8 {
        let saved_delta = Mutex::new(f64::NEG_INFINITY);
        let saved_color = Mutex::new(sRGB {
            r: 99,
            g: 99,
            b: 99,
        });
        let starting_colors = saved_colors.iter().map(|color| color.to_oklab());

        sRGB::all_colors().par_bridge().for_each(|test_srgb| {
            let all_combos = starting_colors
                .clone()
                .chain(std::iter::once(test_srgb.to_oklab()))
                .permutations(2);

            // TODO: find a good averaging method
            let delta = all_combos
                .map(|vector| vector[0].delta_E_Hyab(vector[1]))
                .fold(f64::INFINITY, |a: f64, b: f64| a.min(b));

            let mut locked_saved_delta = saved_delta.lock();
            let mut locked_saved_color = saved_color.lock();

            if (delta > *locked_saved_delta) && (!saved_colors.contains(&test_srgb)) {
                *locked_saved_delta = delta;
                *locked_saved_color = test_srgb;
            }
        });

        let saved_color = saved_color.into_inner();

        if saved_color
            == (sRGB {
                r: 99,
                g: 99,
                b: 99,
            })
        {
            println!("Finished with {:?}", saved_colors.len());
            std::process::exit(99);
        } else {
            println!(
                "{saved_color}, {}, {:.5?}",
                Oklab {
                    l: (saved_color.to_oklab().l * 2.0 / 3.0),
                    a: saved_color.to_oklab().a / 3.0,
                    b: saved_color.to_oklab().b / 3.0,
                    ..Default::default()
                }
                .to_srgb_closest(),
                saved_delta.into_inner()
            );
            saved_colors.push(saved_color)
        }
    }

    println!(
        "Total time: {:.3?}",
        start_time.elapsed().expect("Time went backwards")
    );
}

/*
Hyab, test
sRGB(255, 255, 0), sRGB(144, 146, 95), 1.17899
sRGB(174, 0, 255), sRGB(84, 52, 110), 0.88561
sRGB(0, 102, 0), sRGB(29, 51, 27), 0.59114
sRGB(0, 199, 253), sRGB(72, 110, 126), 0.50147
sRGB(0, 0, 131), sRGB(4, 16, 46), 0.46510
sRGB(255, 85, 0), sRGB(120, 71, 55), 0.46140
sRGB(129, 0, 101), sRGB(57, 26, 47), 0.35797
sRGB(0, 191, 0), sRGB(62, 101, 59), 0.34149
Total time: 124.345s
*/
