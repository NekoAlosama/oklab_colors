mod oklab;
mod rgb;

use oklab::*;
use rayon::prelude::*;
use rgb::*;

const COMP: f64 = 1.0 / 9.0;
fn main() {
    // Time for benchmarking purposes
    let start_time = std::time::SystemTime::now();
    println!("COMP: {:?}", COMP);

    // List of all colors to store, including a starting color (might be your background)
    let starting_color = SRgb { r: 0, g: 0, b: 0 };
    let mut saved_srgb: Vec<SRgb> = vec![starting_color];

    println!("saved_srgb: {saved_srgb:?}");

    // Calculate HyAB median for starting color
    let mut starting_color_deltas: Vec<f64> = AllSRgb::default()
        .par_bridge()
        .map(|color| {
            color
                .srgb_to_oklab()
                .delta_hyab(starting_color.srgb_to_oklab())
        })
        .collect();
    
    starting_color_deltas.par_sort_by(|a, b| a.total_cmp(b));

    // The length of starting_color_deltas should be 2^24 (multiple of 2)
    let starting_color_median = (starting_color_deltas[starting_color_deltas.len() / 2] + starting_color_deltas[starting_color_deltas.len() / 2 - 1]) / 2.0;

    println!("starting_color_median: {starting_color_median:?}");

    // Create an iterator that pre-filters out colors based on starting_color_mean
    let filtered_values = AllSRgb::default()
        .par_bridge()
        .filter(|color| {
            color
                .srgb_to_oklab()
                .delta_hyab(starting_color.srgb_to_oklab())
                > starting_color_median
        });

    // Find 9 new colors
    // This produces a list of 8 well-contrasting colors and one excess color
    for i in 1..=9 {
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

        let twice_filtered_colors: Vec<Rgb<u8>> = filtered_values
            .clone()
            .filter_map(|color| {
                let sample_oklab = color.srgb_to_oklab();

                // Filter out colors that are too similar in relative hue to previous saved colors
                for saved_color in &saved_oklab {
                    if saved_color.chroma() < 1e-5 {
                        continue;
                    }
                    // Haven't found a good way to automate _COMP calculation
                    if saved_color.delta_h(sample_oklab) < COMP {
                        return None;
                    }
                }
                Some(color)
            })
            .collect();

        for color in &twice_filtered_colors {
            let sample_oklab = color.srgb_to_oklab();
            let minimum = saved_oklab
                .iter()
                .map(|color| sample_oklab.delta_hyab(*color))
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
            std::process::exit(i - 1);
        }
        // Print colors
        println!(
            "{i}: S{:?} // 0.6: S{:?} // DE: {:?} // l: {:?}",
            next_color,
            Oklab {
                l: 0.6 * next_color.srgb_to_oklab().l,
                a: next_color.srgb_to_oklab().a,
                b: next_color.srgb_to_oklab().b,
            }
            .oklab_to_srgb_closest(),
            max_delta_e,
            next_color.srgb_to_oklab().l
        );
        saved_srgb.push(next_color);
    }

    println!(
        "Total time: {:?}",
        start_time.elapsed().expect("Time went backwards")
    );
}

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

COMP: 0.1111111111111111
saved_srgb: [Rgb { r: 0, g: 0, b: 0 }]
starting_color_median: 0.8037688504132963
1: SRgb { r: 255, g: 255, b: 0 } // 0.6: SRgb { r: 111, g: 134, b: 0 } // DE: 1.178988628052311 // l: 0.9679827203267873
2: SRgb { r: 174, g: 0, b: 255 } // 0.6: SRgb { r: 80, g: 0, b: 136 } // DE: 0.8856062180812202 // l: 0.5864670698235025
3: SRgb { r: 0, g: 199, b: 253 } // 0.6: SRgb { r: 0, g: 95, b: 143 } // DE: 0.5014721255353223 // l: 0.7729993400576811
4: SRgb { r: 255, g: 85, b: 0 } // 0.6: SRgb { r: 142, g: 0, b: 0 } // DE: 0.4614032257831019 // l: 0.6758924968256405
5: SRgb { r: 0, g: 156, b: 0 } // 0.6: SRgb { r: 0, g: 76, b: 0 } // DE: 0.4073844489987827 // l: 0.6002254608447216
6: SRgb { r: 255, g: 113, b: 231 } // 0.6: SRgb { r: 146, g: 0, b: 133 } // DE: 0.31319765907349356 // l: 0.7577105401372394
7: SRgb { r: 255, g: 178, b: 52 } // 0.6: SRgb { r: 137, g: 82, b: 0 } // DE: 0.2694327778730158 // l: 0.8176745450309861
8: SRgb { r: 78, g: 130, b: 255 } // 0.6: SRgb { r: 11, g: 47, b: 168 } // DE: 0.2569878778127984 // l: 0.6363794758253163
*/
