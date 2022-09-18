use oklab::Oklab;
use rgb::SRgb;

mod oklab;
mod rgb;

fn main() {
    let start_time = std::time::SystemTime::now();

    let mut global_colors: Vec<SRgb> = vec![SRgb { r: 0, g: 0, b: 0 }];

    let mut counter = 8; // amount of NEW colors after Black

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

                    if !(sample_color.delta_eok(Oklab { l: 0.0, a: 0.0, b: 0.0 }) > 2.0/3.0) {
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

/*
// Start with black, no modifiers
Rgb { r: 0, g: 0, b: 0 }, 
Rgb { r: 255, g: 255, b: 255 }, 
Rgb { r: 98, g: 0, b: 255 }, 
Rgb { r: 0, g: 147, b: 0 }, 
Rgb { r: 255, g: 0, b: 91 }, 
Rgb { r: 96, g: 26, b: 3 }, 
Rgb { r: 70, g: 255, b: 0 }, 
Rgb { r: 53, g: 178, b: 255 }, 
SRgb { r: 0, g: 0, b: 122 }, // 0.27413753677679326

// Start with black, delta_eok(black) > 0.5 (aka has to be far away from black)
Rgb { r: 0, g: 0, b: 0 }, 
Rgb { r: 255, g: 255, b: 255 }, 
Rgb { r: 98, g: 0, b: 255 }, 
Rgb { r: 0, g: 147, b: 0 }, 
Rgb { r: 255, g: 0, b: 91 }, 
Rgb { r: 70, g: 255, b: 0 }, 
Rgb { r: 53, g: 178, b: 255 }, 
Rgb { r: 239, g: 167, b: 0 }, 
Rgb { r: 116, g: 88, b: 113 }, // 0.25907948384412693

// Start with black, delta_eok(white) < 0.5 (aka has to be close to white)
Rgb { r: 0, g: 0, b: 0 }, 
Rgb { r: 255, g: 255, b: 255 }, 
Rgb { r: 119, g: 102, b: 1 }, 
Rgb { r: 254, g: 0, b: 255 }, 
Rgb { r: 0, g: 102, b: 255 }, 
Rgb { r: 0, g: 224, b: 0 }, 
Rgb { r: 255, g: 136, b: 121 }, 
Rgb { r: 0, g: 201, b: 238 }, 
Rgb { r: 190, g: 39, b: 113 }, // 0.23907425533987767


[Rgb { r: 0, g: 0, b: 0 }, 
Rgb { r: 255, g: 255, b: 255 }, 
Rgb { r: 181, g: 0, b: 255 }, 
Rgb { r: 2, g: 167, b: 0 }, 
Rgb { r: 255, g: 60, b: 0 }, 
Rgb { r: 77, g: 176, b: 255 }, 
Rgb { r: 149, g: 255, b: 0 }, 
Rgb { r: 255, g: 132, b: 227 }, 
Rgb { r: 209, g: 181, b: 115 }]

*/