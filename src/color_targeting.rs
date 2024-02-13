use crate::{oklab::Oklab, rgb::*};
use parking_lot::Mutex;
use rayon::prelude::*;
use rand::prelude::*;

pub fn main() {
    // Time for benchmarking purposes
    let start_time = std::time::SystemTime::now();
    let mut rng = rand::thread_rng();

    let saved_color = Mutex::new(sRGB::default());
    let saved_delta = Mutex::new(f64::INFINITY);

    //let limits = (0..=255).map(|x| sRGB { r: x, g: x, b: x });
    let mut limits = sRGB::all_colors().collect::<Vec<_>>();
    limits.shuffle(&mut rng);

    let oklab_colors = limits.iter().par_bridge().map(|srgb_color| srgb_color.to_oklab());

    oklab_colors
        .clone()
        .for_each(|original| {
            let max_delta = oklab_colors
                .clone()
                .map(|sample| original.delta_E_Hyab(sample))
                .reduce(|| f64::NEG_INFINITY, |a, b| a.max(b));

            let mut locked_saved_color = saved_color.lock();
            let mut locked_saved_delta = saved_delta.lock();

            if max_delta < *locked_saved_delta {
                *locked_saved_delta = max_delta;
                *locked_saved_color = original.to_srgb();
                println!(
                    "New best: {} / {}, {max_delta}",
                    original.to_srgb(),
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

grayscale:
New best: sRGB(99, 99, 99) / Oklch(0.4996955681708059, 0.000000018625650484542155, 1.5686244887742418), 0.5003044439510853
saved_color: Mutex { data: sRGB { r: 99, g: 99, b: 99 } }
saved_delta: Mutex { data: 0.5003044439510853 }
Total time: 0.0036186

all colors, randomized:
New best: sRGB(119, 128, 126) / Oklch(0.5917254454633886, 0.011096808439379767, -3.1212383372041934), 0.6028222539027683
error: process didn't exit successfully: `target\release\contrasting_colors.exe` (exit code: 0xc000013a, STATUS_CONTROL_C_EXIT)
^C
*/
