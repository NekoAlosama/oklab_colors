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

    let color_center = (SRgb::all_colors()
        .par_bridge()
        .filter(|sample_color| *sample_color != saved_colors[0])
        .map(|sample_color| {
            saved_colors[0]
                .srgb_to_oklab()
                .delta_e(sample_color.srgb_to_oklab())
        })
        .map(|delta| delta.ln_1p())
        .sum::<f64>()
        / (2.0_f64.powi(24) - 1.0))
        .exp_m1();

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
            /*
            // Hue filter, don't know if this is best
            .filter(|test_srgb| {
                let test_color = test_srgb.srgb_to_oklab();
                let hue_differences = starting_colors
                    .clone()
                    // Check if the chorma of the starting color is greater than 0.0
                    // In .delta_h_relative(), not checking would result in a divide-by-zero
                    .filter(|color| (color.a.powi(2) + color.b.powi(2)) > 0.0)
                    .map(|color| color.delta_h_relative(test_color));
                let minimum = hue_differences.fold(f64::MAX, |a, b| if a < b { a } else { b });
                if minimum < (1.0 / 3.0_f64) {
                    false
                } else {
                    true
                }
            })
            */
            .for_each(|test_srgb| {
                let test_colors = starting_colors
                    .clone()
                    .chain(std::iter::once(test_srgb.srgb_to_oklab()));

                let all_deltas = test_colors
                    .permutations(2)
                    .map(|vector| vector[0].delta_e(vector[1]));

                let all_deltas_count = all_deltas.clone().count() as f64;
                let delta = (all_deltas.map(|delta| delta.ln_1p()).sum::<f64>() / all_deltas_count)
                    .exp_m1();

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
                "{saved_color}, {}, {b:.5}",
                Oklab {
                    l: (saved_color.srgb_to_oklab().l * 2.0 / 3.0),
                    a: saved_color.srgb_to_oklab().a / 3.0,
                    b: saved_color.srgb_to_oklab().b / 3.0
                }
                .oklab_to_srgb_closest(),
                b = saved_delta.into_inner()
            );
            saved_colors.push(saved_color)
        }
    }

    println!(
        "Total time: {:?}",
        start_time.elapsed().expect("Time went backwards")
    );
}

/*
delta_e_eok
geometric mean and no hue filter
color_center: 0.65097
(255, 255, 255), (148, 148, 148), 1.00000
(255, 0, 255), (122, 66, 120), 0.72148
(0, 255, 0), (86, 136, 82), 0.66345
(168, 0, 255), (81, 51, 109), 0.58931
(255, 255, 0), (144, 146, 95), 0.55638
(242, 0, 0), (110, 53, 45), 0.52851
(0, 162, 0), (51, 84, 49), 0.50720
(0, 129, 255), (48, 76, 112), 0.48904
Total time: 241.7516222s
*/
