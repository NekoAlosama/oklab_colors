#![allow(unused_imports)]
use oklab::Oklab;
use rgb::SRgb;

mod oklab;
mod rgb;

// If 9 colors, add next term; if 8 colors, subtract next term
/*
const COMP: f64 = 0.0
    + 1.0/1.0
    - 1.0/2.0
    + 1.0/4.0
    - 1.0/8.0
    - 1.0/16.0
    - 1.0/32.0
    + 1.0/64.0
    + 1.0/128.0
    - 1.0/256.0
    - 1.0/512.0
    + 1.0/1024.0
    + 1.0/2048.0
    + 1.0/4096.0
    - 1.0/8192.0
    - 1.0/16384.0
    + 1.0/32768.0
    + 0.0/65536.0
    - 0.0/131072.0
    - 0.0/262144.0
    - 0.0/524288.0;
*/

const STEP: usize = 255;

fn main() {
    // Time for benchmarking purposes
    let start_time = std::time::SystemTime::now();

    /*

    // List of all colors to store, starting with black
    let mut saved_srgb: Vec<SRgb> = vec![
        SRgb { r: 0, g: 0, b: 0 },
    ];

    // Find 16 new colors
    for i in 1..=16 {
        // Remake saved_srgb into a list of Oklab colors
        let saved_oklab: Vec<Oklab> = saved_srgb
            .iter()
            .map(|color| color.srgb_to_oklab())
            .collect();

        let mut next_color = SRgb {
            r: 99,
            g: 99,
            b: 99,
        }; // 50% lightness
        let mut max_delta_e = f64::MIN;

        // Iterate over all values
        for r in 0..=255 {
            for g in 0..=255 {
                for b in 0..=255 {
                    let sample_oklab = SRgb { r, g, b }.srgb_to_oklab();

                    // Reject colors darker than the median black
                    if sample_oklab.delta_eok(SRgb { r: 0, g: 0, b: 0 }.srgb_to_oklab()) < 0.6665780758984647 {
                        continue;
                    }

                    let mut bad_color = false;
                    for color in &saved_oklab {
                        if sample_oklab.delta_h(*color) / color.a.hypot(color.b) < COMP {
                            bad_color = true;
                        }
                    }
                    if bad_color {
                        continue;
                    }

                    // Find the minimum difference
                    let minimum = saved_oklab
                        .iter()
                        .map(|color| sample_oklab.delta_eok(*color))
                        .fold(f64::INFINITY, |a, b| a.min(b));

                    // If the minimum is still high, save it
                    if minimum > max_delta_e {
                        next_color = SRgb { r, g, b };
                        max_delta_e = minimum;
                    }
                }
            }
        }

        if next_color
            == (SRgb {
                r: 99,
                g: 99,
                b: 99,
            })
        {
            println!("Enough for only {:?} colors", i - 1);
            println!(
                "Total time: {:?}",
                start_time.elapsed().expect("Time went backwards")
            );
            println!("{COMP:?}");
            std::process::exit(999);
        }
        // Print colors
        println!(
            "{i}: S{:?}, // {:?} // S{:?}",
            next_color,
            max_delta_e,
            Oklab {
                l: 0.6 * next_color.srgb_to_oklab().l,
                a: next_color.srgb_to_oklab().a,
                b: next_color.srgb_to_oklab().b,
            }
            .oklab_to_srgb()
        );
        saved_srgb.push(next_color);
    }

    println!(
        "Total time: {:?}",
        start_time.elapsed().expect("Time went backwards")
    );
    println!("{COMP:?}");
    */

    /*
    // Find median difference of all colors and black
    let mut deltas: Vec<f64> = Vec::with_capacity(16777216);
    for r in 0..=255 {
        for g in 0..=255 {
            for b in 0..=255 {
                let sample_color = SRgb { r, g, b }.srgb_to_oklab();

                deltas.push(sample_color.delta_eok(SRgb { r: 0, g: 0, b: 0 }.srgb_to_oklab()));
            }
        }
    }

    // Sort deltas
    deltas.sort_by(|a, b| {
        a.partial_cmp(b)
            .expect("No idea how partial_cmp fails in this context")
    });

    let median = match deltas.len() % 2 {
        1 => deltas[deltas.len() / 2],
        0 => (deltas[deltas.len() / 2 - 1] + deltas[deltas.len() / 2]) / 2.0,
        _ => panic!("Modulo 2 should not have a different number"),
    };

    println!("{:?}", median);
    */

    // Just the two best colors
    let background = SRgb { r: 0, g: 0, b: 0 }.srgb_to_oklab();
    let mut best_color_1 = SRgb { r: 0, g: 0, b: 0 }.srgb_to_oklab();
    let mut best_color_2 = SRgb { r: 0, g: 0, b: 0 }.srgb_to_oklab();
    let mut lowest_delta = -1.0;
    let mut lowest_total = -1.0; // Adjust this one for optimization

    for r1 in (0..=255).step_by(STEP) {
        for g1 in (0..=255).step_by(STEP) {
            for b1 in (0..=255).step_by(STEP) {
                let sample_color_1 = SRgb {
                    r: r1,
                    g: g1,
                    b: b1,
                }
                .srgb_to_oklab();

                // Think of a triangle of deltas in the 3d color space
                let delta_1 = background.delta_eok(sample_color_1);
                if delta_1 < lowest_delta {
                    continue;
                }

                for r2 in 0..=255 {
                    for g2 in 0..=255 {
                        for b2 in 0..=255 {
                            let sample_color_2 = SRgb {
                                r: r2,
                                g: g2,
                                b: b2,
                            }
                            .srgb_to_oklab();

                            let delta_2 = background.delta_eok(sample_color_2);
                            if delta_2 < lowest_delta {
                                continue;
                            }

                            let delta_3 = sample_color_1.delta_eok(sample_color_2);
                            if delta_3 < lowest_delta {
                                continue;
                            }

                            // Check for longer triangle perimeter
                            if delta_1 + delta_2 + delta_3 > lowest_total {
                                // if the deltas have passed this point, they are all greater than lowest_delta
                                lowest_delta = delta_1.min(delta_2).min(delta_3);
                                lowest_total = delta_1 + delta_2 + delta_3;
                                best_color_1 = sample_color_1;
                                best_color_2 = sample_color_2;

                                // Print after a full run per sample_color_1
                                println!(
                                    "sample_color_1: {:?}",
                                    SRgb {
                                        r: r1,
                                        g: g1,
                                        b: b1
                                    }
                                );
                                println!(
                                    "best_colors: {:?}, {:?}",
                                    best_color_1.oklab_to_srgb(),
                                    best_color_2.oklab_to_srgb()
                                );
                                println!("lowest_delta, lowest_total: {lowest_delta:?}, {lowest_total:?}");
                                println!(
                                    "Time: {:?}",
                                    start_time.elapsed().expect("Time went backwards")
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Finished!");
    println!("{:?}", best_color_1.oklab_to_srgb());
    println!("{:?}", best_color_2.oklab_to_srgb());
    println!("{lowest_delta:?}, {lowest_total:?}");
    println!(
        "Time: {:?}",
        start_time.elapsed().expect("Time went backwards")
    );
}

/*
// Notes
No restrictions: bad, later colors blend in with each other because of low L

Median L value: 0.6422860119555848
Median delta_eok(black): 0.6665780758984647

// Worst for delta_eok()
Rgb { r: 106, g: 99, b: 77 }, 0.5012295841851826


// Blue and Yellow
sample_color_1: Rgb { r: 0, g: 0, b: 255 }
best_colors: Rgb { r: 0, g: 0, b: 255 }, Rgb { r: 255, g: 255, b: 0 }
lowest_delta, lowest_total: 0.5499269444442552, 2.2672342921431183
// Green and sub-Magenta
sample_color_1: Rgb { r: 0, g: 255, b: 0 }
best_colors: Rgb { r: 0, g: 255, b: 0 }, Rgb { r: 210, g: 0, b: 255 }
lowest_delta, lowest_total: 0.6454100606091758, 2.2673209980606486
*/
