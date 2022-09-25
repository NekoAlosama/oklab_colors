#[allow(unused_imports)]
use oklab::Oklab;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use rgb::SRgb;

mod oklab;
mod rgb;

const COLOR_COUNT: i32 = 8;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = std::time::SystemTime::now();

    let mut global_colors: Vec<SRgb> = vec![SRgb { r: 0, g: 0, b: 0 }];

    for _ in 0..COLOR_COUNT {
        let colors: Vec<Oklab> = global_colors.iter().map(|x| (x).srgb_to_oklab()).collect();
        let mut lowest = (
            SRgb {
                r: 99,
                g: 99,
                b: 99,
            },
            0.0,
        );
        for r in 0..=255 {
            for g in 0..=255 {
                for b in 0..=255 {
                    let sample_color = SRgb { r, g, b }.srgb_to_oklab();

                    // Look for colors above the median
                    if sample_color.delta_eok(SRgb { r: 0, g: 0, b: 0 }.srgb_to_oklab())
                        < 0.6665780758984647
                    {
                        continue;
                    }

                    let minimum = colors
                        .iter()
                        .map(|x| x.delta_eok(sample_color))
                        .fold(f64::INFINITY, |a, b| a.min(b));

                    if minimum > lowest.1 {
                        lowest = (SRgb { r, g, b }, minimum);
                    }
                }
            }
        }

        println!("S{:?}, // {:?}", lowest.0, lowest.1,);
        global_colors.push(lowest.0);

        //println!("{:?}", start_time.elapsed());
    }
    println!("{:?}", global_colors);

    // Find median value
    let mut values: Vec<f64> = Vec::with_capacity(16777216);

    for r in 0..=255 {
        for g in 0..=255 {
            for b in 0..=255 {
                let sample_color = SRgb { r, g, b }.srgb_to_oklab();

                values.push(sample_color.delta_eok(SRgb { r: 0, g: 0, b: 0 }.srgb_to_oklab()))
            }
        }
    }

    let _a = values.len();
    println!("{:?}", _a);

    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median = values[8388608 - 1] + (1.0 / 2.0) * (values[8388608] - values[8388608 - 1]);

    println!("Median: {median:?}");

    let two_thirds = values[11184811 - 1] + (1.0 / 3.0) * (values[11184811] - values[11184811 - 1]);

    println!("2nd 3-fractile: {two_thirds:?}");

    let root = BitMapBackend::new("./test.png", (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf64, RangedCoordf64>::new(
        0.0_f64..16777216.0_f64,
        -1.0..0.0_f64,
        (0..1000, 0..1000),
    ));

    let dot = |x: f64, y: f64| {
        EmptyElement::at((x, y)) + Circle::new((0, 0), 1, ShapeStyle::from(&BLACK).filled())
    };

    for (i, value) in values.iter().enumerate().take(16777216) {
        root.draw(&dot(i as f64, -*value))?;
    }

    root.present()?;

    println!("{:?}", start_time.elapsed());
    Ok(())
}

/*
// Start with black, delta_eok(black) > median
Rgb { r: 255, g: 255, b: 255 },
Rgb { r: 181, g: 0, b: 255 },
Rgb { r: 0, g: 167, b: 0 },
Rgb { r: 255, g: 61, b: 1 },
Rgb { r: 76, g: 176, b: 255 },
Rgb { r: 148, g: 255, b: 0 },
Rgb { r: 255, g: 132, b: 228 },
Rgb { r: 208, g: 181, b: 116 } // 0.23383803751027174

// Start with black, delta_eok(black) > 2nd 3-fractile
Rgb { r: 255, g: 255, b: 255 },
Rgb { r: 234, g: 0, b: 254 },
Rgb { r: 0, g: 193, b: 0 },
Rgb { r: 255, g: 116, b: 61 },
Rgb { r: 78, g: 172, b: 255 },
Rgb { r: 202, g: 255, b: 3 },
Rgb { r: 255, g: 160, b: 248 },
Rgb { r: 10, g: 239, b: 214 } // 0.21081440674696594
*/
