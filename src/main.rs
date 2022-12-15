// Brute-force color

mod oklab;
mod rgb;

use oklab::*;
use rayon::prelude::*;
use rgb::*;

fn main() {
    // Time for benchmarking purposes
    let start_time = std::time::SystemTime::now();
    println!("HUE_LIMIT: {:?}", HUE_LIMIT);

    // List of all colors to store, including a starting color (might be your background)
    let starting_color = SRgb { r: 0, g: 0, b: 0 };
    let mut saved_srgb: Vec<SRgb> = vec![starting_color];

    // Calculate HyAB mean for starting color
    let starting_color_deltas = AllSRgb::default().par_bridge().map(|color| {
        color
            .srgb_to_oklab()
            .delta_hyab(starting_color.srgb_to_oklab())
    });

    // Since the range of delta_hyab should be from 0.0 -> ~1.17, there should be no outliers, though the distribution is skewed
    // The difference between the mean and median is negligible
    let starting_color_mean =
        starting_color_deltas.clone().sum::<f64>() / starting_color_deltas.count() as f64;

    println!("starting_color_mean: {starting_color_mean:?}");

    // Calculate HyAB mean for starting color
    let l_deltas = AllSRgb::default().par_bridge().map(|color| {
        color
            .srgb_to_oklab()
            .delta_l(starting_color.srgb_to_oklab())
            .abs()
    });

    // Since the range of delta_hyab should be from 0.0 -> ~1.17, there should be no outliers, though the distribution is skewed
    // The difference between the mean and median is negligible
    let l_mean =
    l_deltas.clone().sum::<f64>() / l_deltas.count() as f64;

    println!("l_mean: {l_mean:?}");

    // Create an iterator that pre-filters out colors based on starting_color_mean and l_mean
    let filtered_values = AllSRgb::default()
        .par_bridge()
        .filter(|color| {
            color
                .srgb_to_oklab()
                .delta_l(starting_color.srgb_to_oklab())
                .abs()
                > l_mean
        })
        .filter(|color| {
            color
                .srgb_to_oklab()
                .delta_hyab(starting_color.srgb_to_oklab())
                > starting_color_mean
        });

    // Find 9 new colors
    // This produces a list of 8 well-contrasting colors and one excess color
    for i in saved_srgb.len()..=10 {
        // Remake saved_srgb into an updated Vec of Oklab colors
        let saved_oklab: Vec<Oklab> = saved_srgb
            .iter()
            .map(|color| color.srgb_to_oklab())
            .collect();

        // Placeholder color, marks when to end
        let mut next_color = SRgb {
            r: 99,
            g: 99,
            b: 99,
        };
        let mut max_delta_e = f64::MIN;

        let hue_filtered_values: Vec<Rgb<u8>> = filtered_values.clone().filter_map(|color| {
            let sample_color = color.srgb_to_oklab();

            for saved_color in &saved_oklab {
                if saved_color.chroma() < f64::MIN_POSITIVE
                    || sample_color.chroma() < f64::MIN_POSITIVE
                {
                    continue;
                }
                let hue_delta = saved_color.delta_h(sample_color) / saved_color.chroma();
                if hue_delta < HUE_LIMIT {
                    return None
                }
            }
            Some(color)
        }).collect();
        
        for color in &hue_filtered_values {
            let sample_color = color.srgb_to_oklab();

            let minimum = saved_oklab
                .iter()
                .map(|color| color.delta_hyab(sample_color))
                .fold(f64::INFINITY, |a, b| a.min(b));

            // If the minimum is greater than max_delta_e, save it
            if minimum > max_delta_e {
                next_color = *color;
                max_delta_e = minimum;
            }
        }

        // Check if the above was not able to find a new color
        if next_color
            == (SRgb {
                r: 99,
                g: 99,
                b: 99,
            })
        {
            // Early exit
            println!("Enough for only {:?} colors", i - 1);
            println!(
                "Total time: {:?}",
                start_time.elapsed().expect("Time went backwards")
            );
            std::process::exit((i - 1).try_into().unwrap());
        }
        // Print colors
        println!(
            "{i}: S{:?} // 0.6: S{:?} // Max: {:?} // HyAB: {:?}",
            next_color,
            Oklab {
                l: 0.6 * next_color.srgb_to_oklab().l,
                a: 0.6 * next_color.srgb_to_oklab().a,
                b: 0.6 * next_color.srgb_to_oklab().b,
            }
            .oklab_to_srgb_closest(),
            max_delta_e,
            starting_color
                .srgb_to_oklab()
                .delta_hyab(next_color.srgb_to_oklab())
        );
        saved_srgb.push(next_color);
    }

    println!(
        "Total time: {:?}",
        start_time.elapsed().expect("Time went backwards")
    );
}
const HUE_LIMIT: f64 = 0.5;
/*
// Interpretation for generated results: last color is bad
HUE_LIMIT: 0.5
starting_color_mean: 0.7879053739478877
l_mean: 0.6374367230074139
1: SRgb { r: 255, g: 255, b: 0 } // 0.6: SRgb { r: 128, g: 128, b: 0 } // Max: 1.178988628052311 // HyAB: 1.178988628052311
2: SRgb { r: 211, g: 2, b: 255 } // 0.6: SRgb { r: 105, g: 0, b: 128 } // Max: 0.8344607206863967 // HyAB: 0.9458796016391453
3: SRgb { r: 0, g: 202, b: 254 } // 0.6: SRgb { r: 0, g: 100, b: 128 } // Max: 0.49213886263453943 // HyAB: 0.9282755427060038
4: SRgb { r: 255, g: 108, b: 0 } // 0.6: SRgb { r: 128, g: 50, b: 0 } // Max: 0.424408051787352 // HyAB: 0.9032380086238806
5: SRgb { r: 0, g: 170, b: 0 } // 0.6: SRgb { r: 0, g: 83, b: 0 } // Max: 0.3770062120030873 // HyAB: 0.8570366186357381
6: SRgb { r: 66, g: 133, b: 255 } // 0.6: SRgb { r: 28, g: 64, b: 128 } // Max: 0.25783561185514864 // HyAB: 0.8319534016532011
7: SRgb { r: 150, g: 255, b: 243 } // 0.6: SRgb { r: 73, g: 128, b: 122 } // Max: 0.24624658475971292 // HyAB: 1.0327056222041975
8: SRgb { r: 255, g: 159, b: 179 } // 0.6: SRgb { r: 128, g: 77, b: 88 } // Max: 0.23268599642171187 // HyAB: 0.920881337067342
9: SRgb { r: 255, g: 178, b: 0 } // 0.6: SRgb { r: 128, g: 87, b: 0 } // Max: 0.18338069345643662 // HyAB: 0.9865340979011643
Enough for only 9 colors
Total time: 74.3195219s
*/

/*
Notes:
    RGB clamping sucks
    Chroma clamping: common, maintains L and h, limit on chroma
    HyAB: maintains L, c and h can change
    EOk: none maintained, all change
    Helmholtz–Kohlrausch effect?, unknown if Oklab accounts for this (EOK if yes?, HyAB if no?)


RGB is clearly not Gaussian distribuited
Right-tailed / more dark colors compared to bright

Stats of L:
    Min: 0.0
    Median: 0.6422860119555848
    Max: 0.9999999934735462
    Midpoint: 0.4999999967367731
    Mean: 0.6374367230074196
    Standard Deviation: 0.16545165484909216

Stats of delta_hyab with black:
    Min: 0.0
    Median: 0.8037688504132963
    Max: 1.178988628052311
    Midpoint: 0.5894943140261555
    Mean: 0.7879053739478903
    Standard Deviation: 0.18009524803650195


Minimum maximum color differnce:
    Color: SRgb { r: 109, g: 110, b: 66 }
    as Oklch: Oklch { l: 0.5262062467440681, c: 0.06330917886126561, h: 1.9113332386717883 }
    Mean: 0.32338769038481646
    Maximum color difference: 0.5895154256053338


varies around grey
Minimum mean color difference (estimate):
    Color: SRgb { r: 142, g: 144, b: 140 }
    as Oklch: Oklch { l: 0.6504540480868615, c: 0.0062790531395881845, h: 2.2439414355482805 }
    Mean: 0.3296362354811247
    Maximum color differnece: 0.6567331012264497

Median doesn't seem good since it might return as 0.0 for many colors
*/
