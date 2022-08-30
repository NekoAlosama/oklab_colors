use oklab::Oklab;
use rgb::SRgb;

mod oklab;
mod rgb;

fn main() {
    let start_time = std::time::SystemTime::now();

    let mut global_colors: Vec<SRgb> = vec![SRgb { r: 0, g: 0, b: 0 }];

    let mut counter = 63; // amount of NEW colors after Black

    while counter > 0 {
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

        println!(
            "S{:?}, // {:?}",
            lowest.0,
            lowest.1,
        );
        global_colors.push(lowest.0);

        println!("{:?}", start_time.elapsed());
        counter -= 1;
    }
    println!("{:?}", global_colors);
}

/* [Rgb { r: 0, g: 0, b: 0 }, Rgb { r: 255, g: 255, b: 255 }, Rgb { r: 98, g: 0, b: 255 }, Rgb { r: 0, g: 147, b: 0 }, Rgb { r: 255, g: 0, b: 91 }, Rgb { r: 96, g: 26, b: 3 }, Rgb { r: 70, g: 255, b: 0 }, Rgb { r: 53, g: 178, b: 255 }, Rgb { r: 0, g: 0, b: 122 }, Rgb { r: 239, g: 167, b: 0 }, Rgb { r: 227, g: 0, b: 255 }, Rgb { r: 128, g: 99, b: 138 }, Rgb { r: 253, g: 156, b: 226 }, Rgb { r: 2, g: 244, b: 223 }, Rgb { r: 103, g: 0, b: 127 }, Rgb { r: 0, g: 84, b: 0 }, Rgb { r: 0, g: 26, b: 0 }, Rgb { r: 255, g: 244, b: 73 }, Rgb { r: 192, g: 103, b: 1 }, Rgb { r: 117, g: 180, b: 136 }, Rgb { r: 146, g: 112, b: 254 }, Rgb { r: 0, g: 76, b: 168 }, Rgb { r: 172, g: 0, b: 58 }, Rgb { r: 47, g: 0, b: 50 }, Rgb { r: 177, g: 0, b: 163 }, Rgb { r: 16, g: 200, b: 0 }, Rgb { r: 210, g: 204, b: 189 }, Rgb { r: 22, g: 56, b: 73 }, Rgb { r: 208, g: 102, b: 178 }, Rgb { r: 0, g: 139, b: 137 }, Rgb { r: 119, g: 82, b: 24 }, Rgb { r: 0, g: 124, b: 219 }, Rgb { r: 255, g: 125, b: 115 }, Rgb { r: 167, g: 217, b: 94 }, Rgb { r: 0, g: 0, b: 34 }, Rgb { r: 172, g: 158, b: 192 }, Rgb { r: 43, g: 95, b: 101 }, Rgb { r: 0, g: 0, b: 230 }, Rgb { r: 161, g: 17, b: 242 }, Rgb { r: 113, g: 63, b: 183 }, Rgb { r: 105, g: 60, b: 89 }, Rgb { r: 146, g: 144, b: 0 }, Rgb { r: 255, g: 0, b: 185 }, Rgb { r: 59, 
g: 0, b: 166 }, Rgb { r: 254, g: 105, b: 255 }, Rgb { r: 191, g: 255, b: 193 }, Rgb { r: 130, g: 213, b: 255 }, Rgb { r: 143, g: 135, b: 121 }, Rgb { r: 189, g: 81, b: 102 }, Rgb { r: 24, g: 0, b: 1 }, Rgb { r: 255, 
g: 207, b: 255 }, Rgb { r: 195, g: 144, b: 255 }, Rgb { r: 64, g: 114, b: 48 }, Rgb { r: 0, g: 83, b: 253 }, Rgb { r: 55, g: 27, b: 101 }, Rgb { r: 93, g: 0, b: 65 }, Rgb { r: 200, g: 154, b: 116 }, Rgb { r: 255, g: 
207, b: 113 }, Rgb { r: 125, g: 131, b: 197 }, Rgb { r: 28, g: 52, b: 0 }, Rgb { r: 217, g: 2, b: 0 }, Rgb { r: 0, g: 217, b: 150 }, Rgb { r: 53, g: 18, b: 2 }, Rgb { r: 248, g: 77, b: 0 }] */