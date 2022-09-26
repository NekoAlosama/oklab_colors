#![allow(unused_imports)]
use oklab::Oklab;
use rgb::SRgb;

mod oklab;
mod rgb;

fn main() {
    // Time for benchmarking purposes
    let start_time = std::time::SystemTime::now();

    // List of all colors to store, starting with black
    let mut global_colors: Vec<(SRgb, f64)> = vec![(SRgb { r: 0, g: 0, b: 0 }, 0.0)];

    // Find 8 new colors
    for i in 0..8 {
        // Find median difference of the last color to find the next color
        // Underscore because we are only using it for median
        let mut _values: Vec<f64> = Vec::with_capacity(16777216);

        for r in 0..=255 {
            for g in 0..=255 {
                for b in 0..=255 {
                    let sample_color = SRgb { r, g, b }.srgb_to_oklab();

                    _values.push(
                        sample_color.delta_hyab(
                            global_colors
                                .last()
                                .expect("No black somehow?")
                                .0
                                .srgb_to_oklab(),
                        ),
                    );
                }
            }
        }

        // Sort numberically
        _values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Find median
        let median = (_values[_values.len() / 2 - 1] + _values[_values.len() / 2]) / 2.0;

        println!(
            "Median of {:?}: {median:?}",
            global_colors.last().expect("None somehow?")
        );

        global_colors[i].1 = median;

        // Remake global_colors into a list of Oklab colors
        let local_colors: Vec<Oklab> = global_colors
            .iter()
            .map(|x| (x).0.srgb_to_oklab())
            .collect();
        let mut local_low = (
            SRgb {
                r: 99,
                g: 99,
                b: 99,
            },
            f64::MIN,
        );

        // Iterate over all values
        for r in 0..=255 {
            for g in 0..=255 {
                for b in 0..=255 {
                    let sample_color = SRgb { r, g, b }.srgb_to_oklab();

                    if sample_color.l < 0.6422860119555848 {
                        continue;
                    }
                    // Look for colors above the median
                    for color in &global_colors {
                        if sample_color.delta_hyab(color.0.srgb_to_oklab()) < color.1 {
                            continue;
                        }
                    }

                    // Find the minimum difference between the sample_color and all the global_colors
                    let minimum = local_colors
                        .iter()
                        .map(|x| x.delta_hyab(sample_color))
                        .fold(f64::INFINITY, |a, b| a.min(b));

                    // If the minimum is still high, save it
                    if minimum > local_low.1 {
                        local_low = (SRgb { r, g, b }, minimum);
                    }
                }
            }
        }

        // Print colors
        println!(
            "New contrasting color: S{:?}, // {:?}",
            local_low.0, local_low.1,
        );
        global_colors.push(local_low);

        //println!("{:?}", start_time.elapsed());
    }

    for color in &global_colors[1..] {
        println!("S{:?}", color.0);
    }

    // Print colors with 0.6 * L for Etterna
    for color in &global_colors[1..] {
        let sample_color = color.0.srgb_to_oklab();
        println!(
            "S{:?}",
            Oklab {
                l: 0.6 * sample_color.l,
                ..sample_color
            }
            .oklab_to_srgb()
        );
    }

    // Find
    let mut global_min = (
        SRgb {
            r: 255,
            g: 255,
            b: 255,
        },
        f64::MAX,
    );

    for r in 0..=255 {
        for g in 0..=255 {
            for b in 0..=255 {
                let sample_color = SRgb { r, g, b }.srgb_to_oklab();
                let mut local_max = f64::MIN;

                for r2 in &[0, 1] {
                    for g2 in &[0, 1] {
                        for b2 in &[0, 1] {
                            let opposite_color = SRgb {
                                r: r2 * 255,
                                g: g2 * 255,
                                b: b2 * 255,
                            }
                            .srgb_to_oklab();

                            let delta = sample_color.delta_hyab(opposite_color);

                            if delta > local_max {
                                local_max = delta;
                            }
                        }
                    }
                }

                if local_max < global_min.1 {
                    global_min = (SRgb { r, g, b }, local_max);
                }
            }
        }
    }

    println!("Worst contrasting color: {:?}", global_min);

    println!("{:?}", start_time.elapsed());

    let mut _values: Vec<f64> = Vec::with_capacity(16777216);

    for r in 0..=255 {
        for g in 0..=255 {
            for b in 0..=255 {
                let sample_color = SRgb { r, g, b }.srgb_to_oklab();

                _values.push(sample_color.l);
            }
        }
    }

    // Sort numberically
    _values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Find median
    let median = (_values[_values.len() / 2 - 1] + _values[_values.len() / 2]) / 2.0;

    println!("{median:?}");
}

/*


New contrasting color: SRgb { r: 255, g: 255, b: 0 }, // 1.178988628052311
New contrasting color: SRgb { r: 174, g: 0, b: 255 }, // 0.8856062180812202
New contrasting color: SRgb { r: 0, g: 102, b: 0 }, // 0.591135857406696
New contrasting color: SRgb { r: 0, g: 199, b: 253 }, // 0.5014721255353223
New contrasting color: SRgb { r: 0, g: 0, b: 131 }, // 0.46510218724386543
New contrasting color: SRgb { r: 255, g: 85, b: 0 }, // 0.4614032257831019
New contrasting color: SRgb { r: 129, g: 0, b: 101 }, // 0.35797263523043626
New contrasting color: SRgb { r: 0, g: 191, b: 0 }, // 0.3414937399727721

 SRgb { r: 255, g: 255, b: 0 }
SRgb { r: 215, g: 0, b: 255 }
SRgb { r: 0, g: 202, b: 253 }
SRgb { r: 255, g: 109, b: 0 }
SRgb { r: 1, g: 171, b: 0 }
SRgb { r: 255, g: 213, b: 255 }
SRgb { r: 0, g: 249, b: 0 }
SRgb { r: 255, g: 125, b: 217 } 0.2727474309412291

SRgb { r: 134, g: 129, b: 0 }
SRgb { r: 129, g: 0, b: 166 }
SRgb { r: 0, g: 104, b: 151 }
SRgb { r: 155, g: 0, b: 0 }
SRgb { r: 0, g: 91, b: 0 }
SRgb { r: 137, g: 100, b: 137 }
SRgb { r: 0, g: 135, b: 0 }
SRgb { r: 150, g: 18, b: 120 }


// Similar color implies agreement

// This makes less sense?
// Worst for delta_eok()
Rgb { r: 106, g: 99, b: 77 }, 0.5012295841851826

// This makes more sense?
// Worst for delta_hyab()
Rgb { r: 109, g: 110, b: 66 }, 0.5895154256053338
*/
