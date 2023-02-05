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
    let mut colors = vec![SRgb { r: 0, g: 0, b: 0 }];

    // Find the geometric mean of the D_E_HyAB of all colors in the SRgb color space (should be ~0.7638691045633106 for black)
    // Arithmetic mean (~0.7879053739478903) was seen to be too restrictive, and geometric mean kinda makes sense?
    let color_center = SRgb::all_colors()
        .par_bridge()
        .map(|sample_color| {
            colors[0]
                .srgb_to_oklab()
                .delta_e_hyab(sample_color.srgb_to_oklab())
        })
        .map(|delta| delta.powf(1.0 / 2.0_f64.powi(24)))
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
        let starting_colors = colors.iter().map(|color| color.srgb_to_oklab());

        SRgb::all_colors()
            .par_bridge()
            .filter(|test_srgb| {
                test_srgb
                    .srgb_to_oklab()
                    .delta_e_hyab(colors[0].srgb_to_oklab())
                    > color_center
            })
            .for_each(|test_srgb| {
                let test_colors = starting_colors
                    .clone()
                    .chain(std::iter::once(test_srgb.srgb_to_oklab()));

                let all_deltas = test_colors
                    .permutations(2)
                    .map(|vector| vector[0].delta_e_hyab(vector[1]));

                //let delta = all_deltas.fold(f64::MAX, |a, b| if a < b { a } else { b });
                let all_deltas_count = all_deltas.clone().count() as f64;
                let delta = all_deltas
                    .map(|delta| delta.powf(all_deltas_count.recip()))
                    .product();

                // Aquire locks so that there is a lower possibilty of results that are off by 1
                // I think this is how it works?
                // Ex: returning (0,1,255) or (0,0,254) instead of (0,0,255)
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
            println!("Finished with {:?}", colors.len());
            std::process::exit(99);
        } else {
            println!(
                "{saved_color:?}, {:?}, {saved_delta:?}",
                Oklab {
                    l: saved_color.srgb_to_oklab().l * 2.0 / 3.0,
                    a: saved_color.srgb_to_oklab().a / 3.0,
                    b: saved_color.srgb_to_oklab().b / 3.0
                }
                .oklab_to_srgb_closest()
            );
            colors.push(saved_color)
        }
    }

    println!(
        "Total time: {:?}",
        start_time.elapsed().expect("Time went backwards")
    );
}

/*
Product / Geometric mean
color_center: 0.7638691045633555
Rgb { r: 255, g: 255, b: 0 }, Rgb { r: 144, g: 146, b: 95 }, Mutex { data: 1.1789886280523112 }
Rgb { r: 232, g: 0, b: 255 }, Rgb { r: 111, g: 62, b: 117 }, Mutex { data: 0.9769937143974741 }
Rgb { r: 0, g: 0, b: 255 }, Rgb { r: 18, g: 41, b: 98 }, Mutex { data: 0.8531304239398674 }
Rgb { r: 0, g: 255, b: 0 }, Rgb { r: 86, g: 136, b: 82 }, Mutex { data: 0.7866553123091546 }
Rgb { r: 220, g: 0, b: 0 }, Rgb { r: 99, g: 47, b: 40 }, Mutex { data: 0.7307681535561 }
Rgb { r: 255, g: 255, b: 255 }, Rgb { r: 148, g: 148, b: 148 }, Mutex { data: 0.6847196266388391 }
Rgb { r: 111, g: 0, b: 255 }, Rgb { r: 56, g: 45, b: 103 }, Mutex { data: 0.6495625967729834 }
Rgb { r: 0, g: 195, b: 0 }, Rgb { r: 64, g: 103, b: 60 }, Mutex { data: 0.6230359587380241 }
Total time: 321.7228071s
*/
