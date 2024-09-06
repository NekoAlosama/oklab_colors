mod oklab;
mod rgb;
use parking_lot::Mutex;
use rayon::prelude::*;

const HALF_MARGIN: f64 = 0.0227501319482;
const LIMIT: f64 = 0.5;

fn main() {
    let start_time = std::time::Instant::now();

    let saved_saturation = Mutex::new(f64::NEG_INFINITY);
    let saved_color = Mutex::new(rgb::sRGB { r: 0, g: 0, b: 0 });

    rgb::sRGB::all_colors().par_bridge().for_each(|color| {
        let oklch_color = color.to_oklch();

        if oklch_color.l < LIMIT.mul_add(-HALF_MARGIN, LIMIT)
            || oklch_color.l > LIMIT.mul_add(HALF_MARGIN, LIMIT)
        {
            return;
        }

        let saturation = oklch_color.c / (oklch_color.c + oklch_color.l);

        let mut locked_saved_saturation = saved_saturation.lock();
        let mut locked_saved_color = saved_color.lock();

        if saturation > *locked_saved_saturation {
            *locked_saved_saturation = saturation;
            *locked_saved_color = color;
        }
    });

    let saved_saturation = saved_saturation.into_inner();
    let saved_color = saved_color.into_inner();

    dbg!(saved_saturation);
    dbg!(saved_color);
    dbg!(saved_color.to_oklch());

    let end_time = start_time.elapsed();
    dbg!(end_time);
}

/*
usual saturation
[src/main.rs:39:5] saved_color = sRGB {
    r: 85,
    g: 0,
    b: 255,
}
[src/main.rs:40:5] saved_color.to_oklch() = Oklch {
    l: 0.48896292,
    c: 0.29595116,
    h: -1.39736,
    d65_reference_l: false,
}
*/
