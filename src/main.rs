mod oklab;
mod rgb;

use oklab::*;
use rayon::prelude::*;
use rgb::*;

const COMP: f64 = std::f64::consts::PI / 6.0;
fn main() {
    // Time for benchmarking purposes
    let start_time = std::time::SystemTime::now();
    println!("COMP: {:?}", COMP);

    // List of all colors to store, including a starting color (might be your background)
    let starting_color = SRgb { r: 0, g: 0, b: 0 };
    let mut saved_srgb: Vec<SRgb> = vec![
        starting_color,
        SRgb {
            r: 255,
            g: 255,
            b: 0,
        },
    ];

    println!("saved_srgb: {saved_srgb:?}");

    // Calculate l mean
    let l_deltas: Vec<f64> = AllSRgb::default()
        .par_bridge()
        .map(|color| color.srgb_to_oklab().l)
        .collect();
    let l_mean = l_deltas.par_iter().sum::<f64>() / l_deltas.len() as f64;

    println!("l_mean: {l_mean:?}");

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

    // Print second color
    println!(
        "1: S{:?} // 0.6: S{:?} // DE: {:?} // l: {:?}",
        saved_srgb[1],
        Oklab {
            l: 0.6 * saved_srgb[1].srgb_to_oklab().l,
            ..saved_srgb[1].srgb_to_oklab()
        }
        .oklab_to_srgb_closest(),
        saved_srgb[1]
            .srgb_to_oklab()
            .delta_hyab(starting_color.srgb_to_oklab()),
        saved_srgb[1].srgb_to_oklab().l
    );

    // Create an iterator that pre-filters out colors based on starting_color_mean
    let filtered_values = AllSRgb::default()
        .par_bridge()
        .filter(|color| {
            color.srgb_to_oklab().l > starting_color.srgb_to_oklab().l.max(l_mean)
                || color.srgb_to_oklab().l < starting_color.srgb_to_oklab().l.min(l_mean)
        })
        .filter(|color| {
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

COMP: 0.5
saved_srgb: [Rgb { r: 0, g: 0, b: 0 }, Rgb { r: 255, g: 255, b: 0 }]
l_mean: 0.6374367230074206
starting_color_mean: 0.7879053739478898
1: SRgb { r: 255, g: 255, b: 0 } // 0.6: SRgb { r: 111, g: 134, b: 0 } // DE: 1.178988628052311 // l: 0.9679827203267873
2: SRgb { r: 211, g: 2, b: 255 } // 0.6: SRgb { r: 99, g: 0, b: 139 } // DE: 0.8344607206863967 // l: 0.6375615559609987
3: SRgb { r: 0, g: 202, b: 254 } // 0.6: SRgb { r: 0, g: 97, b: 142 } // DE: 0.49213886263453943 // l: 0.7804486680945886
4: SRgb { r: 255, g: 108, b: 0 } // 0.6: SRgb { r: 146, g: 18, b: 0 } // DE: 0.424408051787352 // l: 0.7035513718039714
5: SRgb { r: 0, g: 170, b: 0 } // 0.6: SRgb { r: 0, g: 83, b: 0 } // DE: 0.3770062120030873 // l: 0.6394486105627715
6: SRgb { r: 255, g: 208, b: 231 } // 0.6: SRgb { r: 138, g: 97, b: 118 } // DE: 0.31135197796903935 // l: 0.9045818012261558
7: SRgb { r: 66, g: 133, b: 255 } // 0.6: SRgb { r: 0, g: 50, b: 166 } // DE: 0.25783561185514864 // l: 0.6377244660630951
8: SRgb { r: 8, g: 253, b: 203 } // 0.6: SRgb { r: 0, g: 129, b: 79 } // DE: 0.2505776470783697 // l: 0.8854319415597356

*/
