mod oklab;
mod rgb;
use parking_lot::Mutex;
//use peroxide::fuga::*;
use rayon::prelude::*;

const MARGIN: f64 = 0.01;
pub fn main() {
    let start_time = std::time::Instant::now();

    let saved_saturation = Mutex::new(f64::NEG_INFINITY);
    let saved_color = Mutex::new(rgb::sRGB { r: 0, g: 0, b: 0 });

    rgb::sRGB::all_colors().par_bridge().for_each(|color| {
        let oklch_color = color.to_oklch();

        let saturation = oklch_color.c / (oklch_color.c + oklch_color.l);

        let mut locked_saved_saturation = saved_saturation.lock();
        let mut locked_saved_color = saved_color.lock();

        if saturation >= *locked_saved_saturation
            && oklch_color.l <= f64::from_bits(0x3fe48d9b6500e14b) + MARGIN
            && oklch_color.l >= f64::from_bits(0x3fe48d9b6500e14b) - MARGIN
        {
            *locked_saved_saturation = saturation;
            *locked_saved_color = color;
        }
    });

    let saved_saturation = saved_saturation.into_inner();
    let saved_color = saved_color.into_inner();

    dbg!(saved_saturation);
    dbg!(saved_color);
    dbg!(saved_color.to_oklch());

    /*
    let all_l = rgb::sRGB::all_colors()
        .par_bridge()
        .map(|color| color.to_oklab().l)
        .collect::<Vec<f64>>();

    println!("{:#}", all_l.quantile(1.0 / 3.0, Type2));
    println!("{:#x}", all_l.quantile(1.0 / 3.0, Type2).to_bits());
    println!("{:#}", all_l.quantile(0.5, Type2));
    println!("{:#x}", all_l.quantile(0.5, Type2).to_bits());
    println!("{:#}", all_l.quantile(2.0 / 3.0, Type2));
    println!("{:#x}", all_l.quantile(2.0 / 3.0, Type2).to_bits());

    let all_saturations = rgb::sRGB::all_colors()
        .par_bridge()
        .map(|color| {
            let oklch_color = color.to_oklch();
            let saturation = oklch_color.c / (oklch_color.c + oklch_color.l);
            if saturation.is_finite() {
                saturation
            } else {
                return 0.0;
            }
        })
        .collect::<Vec<f64>>();

    println!("{:#}", all_saturations.quantile(1.0 / 2.0, Type2));
    */

    let end_time = start_time.elapsed();
    dbg!(end_time);
}

/*
1/3 l:
0.5649730861149279
f64::from_bits(0x3fe214426fff7b59)

median l:
0.6422860119555848
f64::from_bits(0x3fe48d9b6500e14b)

2/3 l:
0.7232864083465409
f64::from_bits(0x3fe7252989afab94)

median saturation:
0.1859637946051847
f64::from_bits(0x3fc7cda96008dfca)

[src/main.rs:33:5] saved_saturation = 0.34374442511149855
[src/main.rs:34:5] saved_color = sRGB {
    r: 158,
    g: 0,
    b: 255,
}
[src/main.rs:35:5] saved_color.to_oklch() = Oklch {
    l: 0.565916703868856,
    c: 0.29642523351583727,
    h: -0.9870617546183168,
    d65_reference_l: false,
}
[src/main.rs:67:5] end_time = 3.9917593s
*/
