use crate::{oklab, rgb::*};
use parking_lot::Mutex;
use rayon::prelude::*;

pub fn main() {
    // Time for benchmarking purposes
    let start_time = std::time::SystemTime::now();

    let saved_color = Mutex::new(sRGB::default());
    let saved_delta = Mutex::new(f64::INFINITY);

    //let limits = (0..=255).map(|x| sRGB { r: x, g: x, b: x });
    let limits = sRGB::all_colors();

    let oklab_colors = limits
        .par_bridge()
        .map(|srgb_color| srgb_color.to_oklab().to_d65_white());

    oklab_colors
        .clone()
        .filter(|color| {
            color.delta_E_ab(crate::Oklab::BLACK) <= 0.5006
                && color.delta_E_ab(crate::Oklab::WHITE) <= 0.5006
        })
        .for_each(|original| {
            let max_delta = oklab_colors
                .clone()
                .map(|sample| original.delta_E_ab(sample))
                .reduce(|| 0.0_f64, |a, b| a.max(b));

            let mut locked_saved_color = saved_color.lock();
            let mut locked_saved_delta = saved_delta.lock();

            if max_delta < *locked_saved_delta {
                *locked_saved_delta = max_delta;
                *locked_saved_color = original.to_unreferenced_white().to_srgb();
                println!(
                    "New best: {} / {}, {max_delta}",
                    original.to_unreferenced_white().to_srgb(),
                    original.to_oklch()
                );
            }
        });

    println!("saved_color: {saved_color:?}");
    println!("saved_delta: {saved_delta:?}");

    println!(
        "Total time: {:?}",
        start_time
            .elapsed()
            .expect("Time went backwards")
            .as_secs_f64()
    );
}

/*
least maximum contrast

grayscale, uses delta_E_Hyab, unreferenced white:
New best: sRGB(99, 99, 99) / Oklch(0.4996955681708059, 0.000000018625650484542155, 1.5686244887742418), 0.5003044439283907
saved_color: Mutex { data: sRGB { r: 99, g: 99, b: 99 } }
saved_delta: Mutex { data: 0.5003044439283907 }
Total time: 0.00527

grayscale, uses delta_E_Hyab, D65 white:
New best: sRGB(119, 119, 119) / Oklch(0.5004875299658732, 0.00000002121868428119319, 1.5686244958289373), 0.5004875511845575
saved_color: Mutex { data: sRGB { r: 119, g: 119, b: 119 } }
saved_delta: Mutex { data: 0.5004875511845575 }
Total time: 0.0041162

any color, uses delta_E_Hyab, unreferenced white:
fitered to 0.51 below BLACK and WHITE
New best: sRGB(99, 99, 99) / Oklch(0.4996955681708059, 0.000000018625650484542155, 1.5686244887742418), 0.5003044439283907
saved_color: Mutex { data: sRGB { r: 99, g: 99, b: 99 } }
saved_delta: Mutex { data: 0.5003044439283907 }
Total time: 24.1357114

any color, uses delta_E_Hyab, D65 white:
fitered to 0.51 below BLACK and WHITE
New best: sRGB(119, 119, 119) / Oklch(0.5004875299658732, 0.00000002121868428119319, 1.5686244958289373), 0.5004875511845575
saved_color: Mutex { data: sRGB { r: 119, g: 119, b: 119 } }
saved_delta: Mutex { data: 0.5004875511845575 }
Total time: 174.7414537

any color, uses delta_E_ab, unreferenced white:
filtered to 0.5013 below BLACK and WHITE
New best: sRGB(106, 99, 77) / Oklch(0.4999524894905842, 0.034403377521218005, 1.6144679183027821), 0.5012295841851826
saved_color: Mutex { data: sRGB { r: 106, g: 99, b: 77 } }
saved_delta: Mutex { data: 0.5012295841851826 }
Total time: 424.2191319

any color, uses delta_E_ab, D65 white:
filtered to 0.5006 below BLACK and WHITE
New best: sRGB(121, 120, 104) / Oklch(0.5000038657116537, 0.023873913221130267, 1.8186434009827999), 0.5005735005561995
saved_color: Mutex { data: sRGB { r: 121, g: 120, b: 104 } }
saved_delta: Mutex { data: 0.5005735005561995 }
Total time: 92.8795022
 */
