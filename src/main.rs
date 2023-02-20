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

    // Find the geometric mean of the delta_e_hyab of all SRgb colors (should be ~0.6961545907868586 for black)
    // Geometric mean is used here and later
    let color_center = SRgb::all_colors()
        .par_bridge()
        .map(|sample_color| {
            saved_colors[0]
                .srgb_to_oklab()
                .ref_l()
                .delta_e_hyab(sample_color.srgb_to_oklab().ref_l())
        })
        .map(|delta| delta.powf(1.0 / 2.0_f64.powi(24)))
        // Ignore zeroes for the geometric mean, can't find a better way to implement it
        .filter(|delta_pow| *delta_pow > 0.0)
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
                    .delta_e_hyab(saved_colors[0].srgb_to_oklab().ref_l())
                    > color_center
            })
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
            .for_each(|test_srgb| {
                let test_colors = starting_colors
                    .clone()
                    .chain(std::iter::once(test_srgb.srgb_to_oklab().ref_l()));

                // Calculate the delta_e_hyab of all pairs of colors
                let all_deltas = test_colors
                    .permutations(2)
                    .map(|vector| vector[0].delta_e_hyab(vector[1]));

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
delta_e_hyab
color_center: 0.7638691045635004
(255, 255, 0), (144, 146, 95), Mutex { data: 1.1789886280523112 }
(232, 0, 255), (111, 62, 117), Mutex { data: 0.9769937143974741 }
(0, 0, 255), (18, 41, 98), Mutex { data: 0.8531304239398674 }
(0, 255, 0), (86, 136, 82), Mutex { data: 0.7866553123091546 }
(220, 0, 0), (99, 47, 40), Mutex { data: 0.7307681535561 }
(255, 255, 255), (148, 148, 148), Mutex { data: 0.6847196266388391 }
(111, 0, 255), (56, 45, 103), Mutex { data: 0.6495625967729834 }
(0, 195, 0), (64, 103, 60), Mutex { data: 0.6230359587380241 }
Total time: 299.8869481s

delta_e_hyab with ref_l
color_center: 0.6961545907868586
(255, 255, 0), (158, 160, 109), Mutex { data: 1.1737103045344182 }
(255, 0, 255), (136, 79, 133), Mutex { data: 0.9770036181683445 }
(3, 53, 255), (36, 60, 113), Mutex { data: 0.8539989061926019 }
(0, 255, 0), (99, 150, 95), Mutex { data: 0.7905701301505752 }
(232, 0, 0), (119, 62, 54), Mutex { data: 0.73696951351631 }
(255, 255, 255), (162, 162, 162), Mutex { data: 0.6956202765066236 }
(129, 0, 255), (75, 58, 118), Mutex { data: 0.6635703097445015 }
(0, 175, 0), (68, 104, 65), Mutex { data: 0.6379754887424087 }
Total time: 326.7186512s

delta_e_hyab with ref_l and hue filter (1.0 / 3.0)
color_center: 0.6961545907868837
(255, 255, 0), (158, 160, 109), Mutex { data: 1.1737103045344182 }
(255, 0, 255), (136, 79, 133), Mutex { data: 0.9770036181683445 }
(3, 53, 255), (36, 60, 113), Mutex { data: 0.8539989061926019 }
(0, 255, 0), (99, 150, 95), Mutex { data: 0.7905701301505752 }
(232, 0, 0), (119, 62, 54), Mutex { data: 0.73696951351631 }
(0, 255, 255), (105, 153, 153), Mutex { data: 0.687495415107223 }
(131, 0, 255), (76, 59, 118), Mutex { data: 0.6563199610411524 }
(255, 154, 0), (141, 110, 80), Mutex { data: 0.6251395055898334 }
Total time: 75.3583256s

delta_e_eok
color_center: 0.6363311198384808
(255, 255, 255), (148, 148, 148), Mutex { data: 0.9999999934735468 }
(255, 0, 255), (122, 66, 120), Mutex { data: 0.6974433524326189 }
(0, 255, 0), (86, 136, 82), Mutex { data: 0.6326735344919341 }
(0, 119, 255), (45, 71, 110), Mutex { data: 0.564564503672074 }
(255, 0, 0), (116, 57, 48), Mutex { data: 0.5288553467330599 }
(0, 157, 0), (49, 82, 47), Mutex { data: 0.49706666363890095 }
(255, 255, 0), (144, 146, 95), Mutex { data: 0.47629312002773977 }
(156, 0, 255), (76, 50, 108), Mutex { data: 0.46515053689536395 }
Total time: 275.4107714s
*/
