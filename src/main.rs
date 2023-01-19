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
    let filtered_values = AllSRgb::default()
        .par_bridge()
        .filter(|color| {
            color
                .srgb_to_oklab()
                .delta_hyab(starting_color.srgb_to_oklab())
                > starting_color_mean
        });
    
    let mut total_delta_e = 0.0;

    // Find 9 new colors
    // This produces a list of 8 well-contrasting colors and one excess color
    for i in saved_srgb.len()..=11 {
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
                let hue_delta = saved_color.delta_h(sample_color) * (sample_color.chroma() /  saved_color.chroma());
                if hue_delta < saved_color.chroma() * HUE_LIMIT {
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
            println!("total_delta_e: {total_delta_e:?}");
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
                l: (2.0 / 3.0) * next_color.srgb_to_oklab().l,
                a: (1.0 / 3.0) * next_color.srgb_to_oklab().a,
                b: (1.0 / 3.0) * next_color.srgb_to_oklab().b,
            }
            .oklab_to_srgb_closest(),
            max_delta_e,
            starting_color
                .srgb_to_oklab()
                .delta_hyab(next_color.srgb_to_oklab())
        );
        saved_srgb.push(next_color);
        total_delta_e += max_delta_e;
    }

    println!("total_delta_e: {total_delta_e:?}");
    println!(
        "Total time: {:?}",
        start_time.elapsed().expect("Time went backwards")
    );
}
const HUE_LIMIT: f64 = 0.0;
/*
// relative
HUE_LIMIT: 0.526
starting_color_mean: 0.7879053739478803
1: SRgb { r: 255, g: 255, b: 0 } // 0.6: SRgb { r: 144, g: 146, b: 95 } // Max: 1.178988628052311 // HyAB: 1.178988628052311
2: SRgb { r: 174, g: 0, b: 255 } // 0.6: SRgb { r: 84, g: 52, b: 110 } // Max: 0.8856062180812202 // HyAB: 0.8859335093126509
3: SRgb { r: 255, g: 105, b: 3 } // 0.6: SRgb { r: 121, g: 77, b: 58 } // Max: 0.4916477399434682 // HyAB: 0.9014127903350057
4: SRgb { r: 0, g: 154, b: 0 } // 0.6: SRgb { r: 48, g: 80, b: 46 } // Max: 0.41053314739230745 // HyAB: 0.7968875477963855
5: SRgb { r: 0, g: 224, b: 228 } // 0.6: SRgb { r: 80, g: 122, b: 122 } // Max: 0.3933413117118847 // HyAB: 0.9622739500224883
6: SRgb { r: 0, g: 77, b: 255 } // 0.6: SRgb { r: 32, g: 55, b: 104 } // Max: 0.2852811203526471 // HyAB: 0.7881235684518256
7: SRgb { r: 214, g: 0, b: 99 } // 0.6: SRgb { r: 98, g: 47, b: 60 } // Max: 0.2814983957342849 // HyAB: 0.7895693640603396
8: SRgb { r: 255, g: 0, b: 207 } // 0.6: SRgb { r: 120, g: 62, b: 103 } // Max: 0.2455556318824372 // HyAB: 0.9701216452318219
Enough for only 8 colors
total_delta_e: 4.17245219315056
Total time: 45.7564243s

// not relative
HUE_LIMIT: 0.117
starting_color_mean: 0.7879053739478856
1: SRgb { r: 255, g: 255, b: 0 } // 0.6: SRgb { r: 144, g: 146, b: 95 } // Max: 1.178988628052311 // HyAB: 1.178988628052311
2: SRgb { r: 174, g: 0, b: 255 } // 0.6: SRgb { r: 84, g: 52, b: 110 } // Max: 0.8856062180812202 // HyAB: 0.8859335093126509
3: SRgb { r: 0, g: 199, b: 253 } // 0.6: SRgb { r: 72, g: 110, b: 126 } // Max: 0.5014721255353223 // HyAB: 0.9211634598867131
4: SRgb { r: 255, g: 85, b: 0 } // 0.6: SRgb { r: 120, g: 71, b: 55 } // Max: 0.4614032257831019 // HyAB: 0.8933626557356829
5: SRgb { r: 0, g: 161, b: 0 } // 0.6: SRgb { r: 51, g: 84, b: 48 } // Max: 0.39701189147732086 // HyAB: 0.823337670606284
6: SRgb { r: 0, g: 77, b: 255 } // 0.6: SRgb { r: 32, g: 55, b: 104 } // Max: 0.2852811203526471 // HyAB: 0.7881235684518256
7: SRgb { r: 255, g: 80, b: 217 } // 0.6: SRgb { r: 123, g: 74, b: 110 } // Max: 0.27932244449804555 // HyAB: 0.9680050003186139
8: SRgb { r: 218, g: 0, b: 89 } // 0.6: SRgb { r: 99, g: 47, b: 57 } // Max: 0.22076114704714983 // HyAB: 0.7954040187929728
Enough for only 8 colors
total_delta_e: 4.209846800827118
Total time: 49.2169669s

// no limit
HUE_LIMIT: 0.0
starting_color_mean: 0.7879053739478927
1: SRgb { r: 255, g: 255, b: 0 } // 0.6: SRgb { r: 144, g: 146, b: 95 } // Max: 1.178988628052311 // HyAB: 1.178988628052311
2: SRgb { r: 174, g: 0, b: 255 } // 0.6: SRgb { r: 84, g: 52, b: 110 } // Max: 0.8856062180812202 // HyAB: 0.8859335093126509
3: SRgb { r: 0, g: 199, b: 253 } // 0.6: SRgb { r: 72, g: 110, b: 126 } // Max: 0.5014721255353223 // HyAB: 0.9211634598867131
4: SRgb { r: 255, g: 85, b: 0 } // 0.6: SRgb { r: 120, g: 71, b: 55 } // Max: 0.4614032257831019 // HyAB: 0.8933626557356829
5: SRgb { r: 0, g: 152, b: 0 } // 0.6: SRgb { r: 48, g: 79, b: 45 } // Max: 0.4157467762816877 // HyAB: 0.7892901961808592
6: SRgb { r: 255, g: 212, b: 255 } // 0.6: SRgb { r: 140, g: 127, b: 140 } // Max: 0.32231909649246143 // HyAB: 0.9929480035741907
7: SRgb { r: 255, g: 111, b: 230 } // 0.6: SRgb { r: 126, g: 84, b: 117 } // Max: 0.31069024658775146 // HyAB: 0.9744149857653833
8: SRgb { r: 0, g: 236, b: 0 } // 0.6: SRgb { r: 79, g: 126, b: 75 } // Max: 0.3029078909830686 // HyAB: 1.0951778865013337
9: SRgb { r: 0, g: 77, b: 255 } // 0.6: SRgb { r: 32, g: 55, b: 104 } // Max: 0.2852811203526471 // HyAB: 0.7881235684518256
10: SRgb { r: 255, g: 181, b: 78 } // 0.6: SRgb { r: 131, g: 109, b: 82 } // Max: 0.2735572790142253 // HyAB: 0.9696272058253093
11: SRgb { r: 211, g: 0, b: 112 } // 0.6: SRgb { r: 97, g: 46, b: 63 } // Max: 0.2631370670688121 // HyAB: 0.7894673640476573
total_delta_e: 5.20110967423261
Total time: 106.1059602s
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
