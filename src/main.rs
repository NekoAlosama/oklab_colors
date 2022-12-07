mod oklab;
mod rgb;

use oklab::*;
use rayon::prelude::*;
use rgb::*;

const COMP: f64 = 0.55;
fn main() {
    // Time for benchmarking purposes
    let start_time = std::time::SystemTime::now();
    println!("COMP: {:?}", COMP);

    // List of all colors to store, including a starting color (might be your background)
    let starting_color = SRgb { r: 0, g: 0, b: 0 };
    let mut saved_srgb: Vec<SRgb> = vec![starting_color, SRgb { r: 255, g: 255, b: 0 }];

    println!("saved_srgb: {saved_srgb:?}");
    println!("1: {:?} // 0.6: {:?}", saved_srgb[1], Oklab {
        l: 0.6 * saved_srgb[1].srgb_to_oklab().l,
        ..saved_srgb[1].srgb_to_oklab()
    }.oklab_to_srgb_closest());

    // Calculate HyAB mean for starting color
    let starting_color_deltas: Vec<f64> = AllSRgb::default()
        .par_bridge()
        .map(|color| {
            color
                .srgb_to_oklab()
                .delta_hyab(starting_color.srgb_to_oklab())
        })
        .collect();
    let starting_color_mean =
        starting_color_deltas.par_iter().sum::<f64>() / starting_color_deltas.len() as f64;
    
    println!("starting_color_mean: {starting_color_mean:?}");

    // Create an iterator that pre-filters out colors based on starting_color_mean
    let filtered_values = AllSRgb::default().par_bridge().filter(|color| {
        color
            .srgb_to_oklab()
            .delta_hyab(starting_color.srgb_to_oklab())
            > starting_color_mean
    });

    // Find 9 new colors
    // This produces a list of 8 well-contrasting colors and one excess color
    for i in 2..=9 {
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
                    // Haven't found a good way to automate _COMP calculation
                    if saved_color.relative_delta_h(sample_oklab) < COMP {
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

COMP: 0.55
saved_srgb: [Rgb { r: 0, g: 0, b: 0 }, Rgb { r: 255, g: 255, b: 0 }]
starting_color_mean: 0.7879053739478907
2: SRgb { r: 174, g: 0, b: 255 } // 0.6: SRgb { r: 80, g: 0, b: 136 } // DE: 0.8856062180812202 // l: 0.5864670698235025
3: SRgb { r: 0, g: 199, b: 253 } // 0.6: SRgb { r: 0, g: 95, b: 143 } // DE: 0.5014721255353223 // l: 0.7729993400576811
4: SRgb { r: 255, g: 85, b: 0 } // 0.6: SRgb { r: 142, g: 0, b: 0 } // DE: 0.4614032257831019 // l: 0.6758924968256405
5: SRgb { r: 0, g: 152, b: 0 } // 0.6: SRgb { r: 0, g: 74, b: 0 } // DE: 0.4157467762816877 // l: 0.5889019305640453
6: SRgb { r: 255, g: 208, b: 237 } // 0.6: SRgb { r: 138, g: 97, b: 123 } // DE: 0.317962646542098 // l: 0.9064095933177545
7: SRgb { r: 0, g: 77, b: 255 } // 0.6: SRgb { r: 0, g: 0, b: 156 } // DE: 0.2852811203526471 // l: 0.5201799787357348
8: SRgb { r: 249, g: 178, b: 42 } // 0.6: SRgb { r: 131, g: 84, b: 0 } // DE: 0.2714124119654985 // l: 0.8105051200012297

COMP: 0.55
saved_srgb: [Rgb { r: 0, g: 0, b: 0 }, Rgb { r: 255, g: 255, b: 255 }]
starting_color_mean: 0.7879053739478898
2: SRgb { r: 98, g: 0, b: 255 } // 0.6: SRgb { r: 32, g: 0, b: 137 } // DE: 0.7939227742807733 // l: 0.5001367631674302
3: SRgb { r: 0, g: 162, b: 0 } // 0.6: SRgb { r: 0, g: 79, b: 0 } // DE: 0.5928757036749703 // l: 0.6171115421336347
4: SRgb { r: 255, g: 91, b: 175 } // 0.6: SRgb { r: 142, g: 0, b: 93 } // DE: 0.5025946985016285 // l: 0.7109459244275075
5: SRgb { r: 0, g: 197, b: 255 } // 0.6: SRgb { r: 0, g: 94, b: 144 } // DE: 0.3810114118348807 // l: 0.7692645746107217
6: SRgb { r: 255, g: 175, b: 113 } // 0.6: SRgb { r: 145, g: 76, b: 0 } // DE: 0.3038395393252375 // l: 0.8183737595630627
7: SRgb { r: 219, g: 0, b: 32 } // 0.6: SRgb { r: 110, g: 0, b: 0 } // DE: 0.27742733304017564 // l: 0.5612704708488127
8: SRgb { r: 198, g: 0, b: 249 } // 0.6: SRgb { r: 93, g: 0, b: 134 } // DE: 0.2764380413357115 // l: 0.6145173008284384
*/
