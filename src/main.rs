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

    // Create an iterator that pre-filters out colors based on starting_color_mean and l_mean
    let filtered_values: Vec<Rgb<u8>> = AllSRgb::default()
        .par_bridge()
        .filter(|color| {
            color
                .srgb_to_oklab()
                .delta_hyab(starting_color.srgb_to_oklab())
                > starting_color_mean
        })
        .collect();

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

        for color in &filtered_values {
            let sample_color = color.srgb_to_oklab();

            let mut bad_hue = false;
            for saved_color in &saved_oklab {
                if saved_color.chroma() < 0.001 {
                    continue;
                }
                let hue_delta = sample_color.modified_delta_h(*saved_color, HUE_LIMIT);
                if hue_delta < HUE_LIMIT {
                    bad_hue = true;
                }
            }
            if bad_hue {
                continue;
            }

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
                a: next_color.srgb_to_oklab().a,
                b: next_color.srgb_to_oklab().b,
            }
            .oklab_to_srgb_closest(),
            max_delta_e,
            starting_color.srgb_to_oklab().delta_hyab(next_color.srgb_to_oklab())
        );
        saved_srgb.push(next_color);
    }

    println!(
        "Total time: {:?}",
        start_time.elapsed().expect("Time went backwards")
    );
}
const HUE_LIMIT: f64 = 0.116;
/*
// Bordering on 8 and 9, generating 8
// Interpretation: 7 good colors, 8th can be thrown away and is probably bad
HUE_LIMIT: 0.118
starting_color_mean: 0.7879053739478883
1: SRgb { r: 255, g: 255, b: 0 } // 0.6: SRgb { r: 111, g: 134, b: 0 } // Max: 1.178988628052311 // HyAB: 1.178988628052311
2: SRgb { r: 174, g: 0, b: 255 } // 0.6: SRgb { r: 80, g: 0, b: 136 } // Max: 0.8856062180812202 // HyAB: 0.8859335093126509
3: SRgb { r: 0, g: 199, b: 253 } // 0.6: SRgb { r: 0, g: 95, b: 143 } // Max: 0.5014721255353223 // HyAB: 0.9211634598867131
4: SRgb { r: 255, g: 85, b: 0 } // 0.6: SRgb { r: 142, g: 0, b: 0 } // Max: 0.4614032257831019 // HyAB: 0.8933626557356829
5: SRgb { r: 0, g: 154, b: 24 } // 0.6: SRgb { r: 0, g: 75, b: 0 } // Max: 0.4077417242727185 // HyAB: 0.791179843028626
6: SRgb { r: 255, g: 114, b: 229 } // 0.6: SRgb { r: 147, g: 0, b: 130 } // Max: 0.30987847887930103 // HyAB: 0.9724467496544608
7: SRgb { r: 0, g: 77, b: 255 } // 0.6: SRgb { r: 0, g: 0, b: 156 } // Max: 0.2852811203526471 // HyAB: 0.7881235684518256
8: SRgb { r: 214, g: 0, b: 96 } // 0.6: SRgb { r: 106, g: 0, b: 45 } // Max: 0.23773035817250346 // HyAB: 0.7881277825144696

// Trying to border on 9 and 10, generating 10+
// Interpretation: 8 good colors, 9th+ can be thrown away and is probably bad
HUE_LIMIT: 0.116
starting_color_mean: 0.7879053739478904
1: SRgb { r: 255, g: 255, b: 0 } // 0.6: SRgb { r: 111, g: 134, b: 0 } // Max: 1.178988628052311 // HyAB: 1.178988628052311
2: SRgb { r: 174, g: 0, b: 255 } // 0.6: SRgb { r: 80, g: 0, b: 136 } // Max: 0.8856062180812202 // HyAB: 0.8859335093126509
3: SRgb { r: 0, g: 199, b: 253 } // 0.6: SRgb { r: 0, g: 95, b: 143 } // Max: 0.5014721255353223 // HyAB: 0.9211634598867131
4: SRgb { r: 255, g: 85, b: 0 } // 0.6: SRgb { r: 142, g: 0, b: 0 } // Max: 0.4614032257831019 // HyAB: 0.8933626557356829
5: SRgb { r: 0, g: 152, b: 3 } // 0.6: SRgb { r: 0, g: 74, b: 0 } // Max: 0.41538630926588854 // HyAB: 0.7886925924459887
6: SRgb { r: 255, g: 113, b: 231 } // 0.6: SRgb { r: 146, g: 0, b: 133 } // Max: 0.31319765907349356 // HyAB: 0.9750673383555444
7: SRgb { r: 0, g: 77, b: 255 } // 0.6: SRgb { r: 1, g: 0, b: 155 } // Max: 0.2852811203526471 // HyAB: 0.7881235684518256
8: SRgb { r: 255, g: 172, b: 10 } // 0.6: SRgb { r: 137, g: 78, b: 0 } // Max: 0.25508502658009513 // HyAB: 0.9745717763341601
// excess
9: SRgb { r: 23, g: 255, b: 226 } // 0.6: SRgb { r: 0, g: 130, b: 96 } // Max: 0.24480186166388457 // HyAB: 1.0575240560137602
10: SRgb { r: 214, g: 0, b: 100 } // 0.6: SRgb { r: 106, g: 0, b: 49 } // Max: 0.2423818670471716 // HyAB: 0.7900694037994571

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
