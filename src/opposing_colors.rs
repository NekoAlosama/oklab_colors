use crate::oklab::*;
use crate::rgb::*;
use itertools::Itertools;
use parking_lot::Mutex;
use rayon::prelude::*;

pub fn main() {
    let start_time = std::time::SystemTime::now();

    let mut saved_colors = vec![sRGB { r: 0, g: 0, b: 0 }];

    for _ in 1..=9 {
        let saved_delta = Mutex::new(f64::NEG_INFINITY);
        let saved_color = Mutex::new(sRGB::default());
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

        println!(
            "{saved_color}, {}, {:.5?}",
            Oklab {
                l: (saved_color.to_oklab().l * 2.0 / 3.0),
                a: saved_color.to_oklab().a / 3.0,
                b: saved_color.to_oklab().b / 3.0,
                d65_reference_l: false
            }
            .to_srgb_closest(),
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
Hyab min, unreferenced white
sRGB(255, 255, 0), sRGB(144, 146, 95), 1.17899
sRGB(174, 0, 255), sRGB(84, 52, 110), 0.88561
sRGB(0, 102, 0), sRGB(29, 51, 27), 0.59114
sRGB(0, 199, 253), sRGB(72, 110, 126), 0.50147
sRGB(0, 0, 131), sRGB(4, 16, 46), 0.46510
sRGB(255, 85, 0), sRGB(120, 71, 55), 0.46140
sRGB(129, 0, 101), sRGB(57, 26, 47), 0.35797
sRGB(0, 191, 0), sRGB(62, 101, 59), 0.34149
sRGB(255, 212, 255), sRGB(140, 127, 140), 0.32232
Total time: 104.289s

Hyab min, D65 white
sRGB(255, 255, 0), sRGB(158, 160, 109), 1.17371

sRGB(211, 0, 255), sRGB(114, 70, 127), 0.88753
sRGB(0, 128, 0), sRGB(50, 77, 47), 0.62036

sRGB(2, 0, 201), sRGB(21, 40, 86), 0.54520
sRGB(0, 203, 255), sRGB(87, 125, 141), 0.51915

sRGB(255, 109, 0), sRGB(135, 91, 71), 0.43797
sRGB(158, 0, 82), sRGB(82, 42, 55), 0.41752
sRGB(68, 117, 204), sRGB(63, 79, 105), 0.33563
sRGB(255, 210, 255), sRGB(154, 140, 154), 0.33405
Total time: 89.833s
*/
