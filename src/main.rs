mod oklab;
mod rgb;

use itertools::Itertools;
use oklab::*;
use parking_lot::Mutex;
use rayon::prelude::*;
use rgb::*;

fn main() {
    let start_time = std::time::SystemTime::now();

    let mut saved_colors = vec![SRgb { r: 0, g: 0, b: 0 }];

    let color_center = SRgb::all_colors()
        .par_bridge()
        .map(|sample_color| {
            saved_colors[0]
                .srgb_to_oklab()
                .delta_e(sample_color.srgb_to_oklab())
        })
        .sum::<f64>()
        / 2.0_f64.powi(24);

    println!("color_center: {color_center:.5}");

    for _ in 1..=8 {
        let saved_delta = Mutex::new(f64::MIN);
        let saved_color = Mutex::new(SRgb {
            r: 99,
            g: 99,
            b: 99,
        });
        let starting_colors = saved_colors.iter().map(|color| color.srgb_to_oklab());

        SRgb::all_colors()
            .par_bridge()
            .filter(|test_srgb| {
                test_srgb
                    .srgb_to_oklab()
                    .delta_e(saved_colors[0].srgb_to_oklab())
                    > color_center
            })
            .for_each(|test_srgb| {
                let test_colors = starting_colors
                    .clone()
                    .chain(std::iter::once(test_srgb.srgb_to_oklab()));

                let all_deltas = test_colors
                    .permutations(2)
                    .map(|vector| vector[0].delta_e(vector[1]));

                let all_deltas_count = all_deltas.clone().count() as f64;
                let delta =
                    (all_deltas.map(|delta| delta.ln()).sum::<f64>() / all_deltas_count).exp();

                let mut locked_saved_delta = saved_delta.lock();
                let mut locked_saved_color = saved_color.lock();

                if delta > *locked_saved_delta {
                    *locked_saved_delta = delta;
                    *locked_saved_color = test_srgb;
                }
            });

        let saved_color = saved_color.into_inner();

        if saved_color
            == (SRgb {
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
                    l: (saved_color.srgb_to_oklab().l * 2.0 / 3.0),
                    a: saved_color.srgb_to_oklab().a / 3.0,
                    b: saved_color.srgb_to_oklab().b / 3.0
                }
                .oklab_to_srgb_closest(),
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
arithmetic mean for color_center
color_center: 0.65910
(255, 255, 255), (148, 148, 148), 1.00000
(255, 0, 255), (122, 66, 120), 0.69744
(0, 255, 0), (86, 136, 82), 0.63267
(246, 0, 0), (112, 54, 46), 0.56269
(0, 134, 255), (50, 78, 113), 0.52354
(255, 255, 0), (144, 146, 95), 0.49229
(175, 0, 255), (85, 52, 110), 0.47025
(0, 165, 0), (52, 86, 50), 0.45863
Total time: 225.157s
*/
