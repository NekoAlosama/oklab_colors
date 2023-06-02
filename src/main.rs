mod oklab;
mod rgb;

use itertools::Itertools;
use oklab::*;
use parking_lot::Mutex;
use rayon::prelude::*;
use rgb::*;

fn main() {
    // Time for benchmarking purposes
    let start_time = std::time::SystemTime::now();

    // Save generated colors into this Vec for future colors
    let mut saved_colors = vec![SRgb { r: 0, g: 0, b: 0 }];

    // Find the geometric mean of the delta_e_eok of all SRgb colors
    // Geometric mean is used here and later
    let color_center = SRgb::all_colors()
        .par_bridge()
        .map(|sample_color| {
            saved_colors[0]
                .srgb_to_oklab()
                .ref_l()
                .delta_e_eok(sample_color.srgb_to_oklab().ref_l())
        })
        .map(|delta| delta.powf(1.0 / 2.0_f64.powi(24)))
        // Ignore zeroes for the geometric mean, can't find a better way to implement it
        .filter(|delta_pow| *delta_pow > 1e-10)
        .product::<f64>();

    println!("color_center: {color_center:?}");

    for _ in 1..=8 {
        let saved_delta = Mutex::new(f64::MIN);
        let saved_color = Mutex::new(SRgb {
            r: 99,
            g: 99,
            b: 99,
        });
        let starting_colors = saved_colors
            .iter()
            .map(|color| color.srgb_to_oklab().ref_l());

        SRgb::all_colors()
            .par_bridge()
            // Use color_center to remove colors below the mean
            .filter(|test_srgb| {
                test_srgb
                    .srgb_to_oklab()
                    .ref_l()
                    .delta_e_eok(saved_colors[0].srgb_to_oklab().ref_l())
                    > color_center
            })
            /*
            // Hue filter, don't know if this is best
            .filter(|test_srgb| {
                let test_color = test_srgb.srgb_to_oklab().ref_l();
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
                    .chain(std::iter::once(test_srgb.srgb_to_oklab().ref_l()));

                // Calculate the delta_e_eok of all pairs of colors
                let all_deltas = test_colors
                    .permutations(2)
                    .map(|vector| vector[0].delta_e_eok(vector[1]));

                // Calculate the geometric mean of the all_deltas
                // Allows for 0.0, since that means that test_srgb is the same as one of the colors in saved_colors
                let all_deltas_count = all_deltas.clone().count() as f64;
                let delta = all_deltas
                    .map(|delta| delta.powf(all_deltas_count.recip()))
                    .product();

                // Acquire locks so that there is a lower possibilty of results that are off by 1
                // Ex: returning (0, 0, 254) instead of (0, 0, 255)
                // I think this is how it works?
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
                "{saved_color}, {}, {saved_delta:?}",
                Oklab {
                    l: (saved_color.srgb_to_oklab().ref_l().l * 2.0 / 3.0),
                    a: saved_color.srgb_to_oklab().a / 3.0,
                    b: saved_color.srgb_to_oklab().b / 3.0
                }
                .unref_l()
                .oklab_to_srgb_closest()
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
color_center: 0.5711175110266362
(255, 255, 255), (162, 162, 162), Mutex { data: 0.9999999923961904 }
(255, 0, 255), (136, 79, 133), Mutex { data: 0.7013396866690387 }
(0, 255, 0), (99, 150, 95), Mutex { data: 0.6363327019523347 }
(1, 116, 255), (56, 82, 123), Mutex { data: 0.5723935204881084 }
(233, 0, 0), (119, 63, 55), Mutex { data: 0.5364216704163772 }
(255, 255, 0), (158, 160, 109), Mutex { data: 0.5096122753917641 }
(0, 156, 0), (61, 93, 58), Mutex { data: 0.4902856311835443 }
(153, 0, 255), (86, 61, 120), Mutex { data: 0.4777737583278232 }
Total time: 330.9302167s
*/
