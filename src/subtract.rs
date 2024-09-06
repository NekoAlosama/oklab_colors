use crate::oklab::*;
use crate::rgb::*;
use itertools::Itertools;
use parking_lot::Mutex;
use rayon::prelude::*;

pub fn main() {
    let start_time = std::time::SystemTime::now();

    let mut saved_colors = vec![
        sRGB { r: 0, g: 0, b: 0 },
        sRGB {
            r: 255,
            g: 255,
            b: 255,
        },
    ];

    for _ in 1..=8 {
        let saved_delta = Mutex::new(f32::NEG_INFINITY);
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
                .fold(f32::INFINITY, |a: f32, b: f32| a.min(b));

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
                l: saved_color.to_oklab().l
                    * 0.5
                    * ((saved_color.to_oklab().l + 0.3 * saved_color.to_oklab().chroma())
                        / (0.7 * saved_color.to_oklab().l)),
                a: saved_color.to_oklab().a * 0.5,
                b: saved_color.to_oklab().b * 0.5,
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
SubtractByZero colors, d65 white
1: Oklch { l: 0.5659293, c: 0.25691673, h: 0.51022726, d65_reference_l: true }
1.5: Oklch { l: 0.36701253, c: 0.08853466, h: 0.36954263, d65_reference_l: true }
2: Oklch { l: 0.42238456, c: 0.28034958, h: -1.6786362, d65_reference_l: true }
2.5: Oklch { l: 0.46813166, c: 0.25438973, h: -1.5612254, d65_reference_l: true }
3: Oklch { l: 0.8445289, c: 0.2948271, h: 2.4870124, d65_reference_l: true }
3.5: Oklch { l: 0.46812937, c: 0.11657242, h: 2.5089962, d65_reference_l: true }
4: Oklch { l: 0.96270436, c: 0.21100593, h: 1.9158345, d65_reference_l: true }
4.5: Oklch { l: 0.5189428, c: 0.086309135, h: 1.8988158, d65_reference_l: true }
5: Oklch { l: 0.6149031, c: 0.2856644, h: -0.28258458, d65_reference_l: true }
5.5: Oklch { l: 0.38586926, c: 0.10842354, h: -0.39326328, d65_reference_l: true }
6: Oklch { l: 0.75678724, c: 0.17117621, h: 1.2243929, d65_reference_l: true }
6.5: Oklch { l: 0.45930898, c: 0.0646044, h: 1.4156178, d65_reference_l: true }
7: Oklch { l: 0.88984823, c: 0.15454963, h: -2.883825, d65_reference_l: true }
7.5: Oklch { l: 0.48737255, c: 0.06570041, h: -2.8722253, d65_reference_l: true }
8: Oklch { l: 0.68795955, c: 5.9604645e-8, h: 3.1415927, d65_reference_l: true }
8.5: Oklch { l: 0.2762164, c: 4.4703484e-8, h: 3.1415927, d65_reference_l: true }
1: (0.37963274, 0.2909576)
1.5: (0.1920533, 0.16366722)
2: (0.4882576, 0.35876042)
2.5: (0.42547604, 0.3197996)
3: (0.32213548, 0.25388408)
3.5: (0.21061362, 0.17725688)
4: (0.21298371, 0.17897196)
4.5: (0.1458813, 0.12850901)
5: (0.3929537, 0.29939055)
5.5: (0.22522679, 0.18776204)
6: (0.21152738, 0.17791866)
6.5: (0.12024429, 0.10803731)
7: (0.16826396, 0.14580858)
7.5: (0.11696678, 0.10536573)
8: (8.147219e-8, 8.147218e-8)
8.5: (1.2033402e-7, 1.2033401e-7)

70% saturation

delta_E_Hyab
sRGB(98, 0, 255), sRGB(71, 59, 153), 0.79392
sRGB(0, 162, 0), sRGB(56, 110, 52), 0.59288
sRGB(119, 0, 0), sRGB(72, 25, 20), 0.50399
sRGB(255, 93, 184), sRGB(162, 88, 126), 0.49814
sRGB(0, 0, 105), sRGB(4, 18, 59), 0.39720
sRGB(0, 199, 255), sRGB(71, 133, 158), 0.37576
sRGB(255, 179, 0), sRGB(163, 128, 72), 0.35261
sRGB(0, 102, 107), sRGB(31, 64, 66), 0.33159
Total time: 63.985s
*/
