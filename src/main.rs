use crate::rgb::SRgb;

mod oklab;
mod rgb;

fn main() {
    let start_time = std::time::SystemTime::now();

    let colors = [
        SRgb { r: 0, g: 0, b: 0 },
        SRgb { r: 255, g: 255, b: 255, },
        //
        SRgb { r: 98, g: 0, b: 255 }, // 0.5799433286232789
        SRgb { r: 0, g: 147, b: 0 }, // 0.4681274897859445
        SRgb { r: 255, g: 0, b: 91 }, // 0.4110261974778937
        SRgb { r: 96, g: 26, b: 3 }, // 0.34670575331839143
        SRgb { r: 70, g: 255, b: 0 }, // 0.31207694016546567
        SRgb { r: 53, g: 178, b: 255 }, // 0.3092397920299183
        SRgb { r: 0, g: 0, b: 122 }, // 0.27413753677679326
        SRgb { r: 239, g: 167, b: 0 }, // 0.2733065816661675
        SRgb { r: 227, g: 0, b: 255 }, // 0.25427909778983876
        SRgb { r: 128, g: 99, b: 138 }, // 0.24213254326823605
        SRgb { r: 253, g: 156, b: 226 }, // 0.23574886807659642
        SRgb { r: 2, g: 244, b: 223 }, // 0.2020212732606329
        SRgb { r: 103, g: 0, b: 127 }, // 0.20061010087749231
    ]
    .map(|x| x.srgb_to_oklab());
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
                let minimum = colors
                    .map(|x| x.oklab_difference(sample_color))
                    .iter()
                    .fold(f64::INFINITY, |a, &b| a.min(b));

                if minimum > lowest.1 {
                    lowest = (SRgb { r, g, b }, minimum);
                }
            }
        }
    }

    println!("S{:?}, // {:?}", lowest.0, lowest.1);

    println!("{:?}", start_time.elapsed());
}
