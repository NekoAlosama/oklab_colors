use crate::rgb::*;
use parking_lot::Mutex;
use rand::prelude::*;
use rayon::prelude::*;

pub fn main() {
    // Time for benchmarking purposes
    let start_time = std::time::SystemTime::now();
    let mut rng = rand::thread_rng();

    let saved_color = Mutex::new(sRGB::default());
    let saved_delta = Mutex::new(f64::NEG_INFINITY);

    //let mut limits = (0..=255).map(|x| sRGB { r: x, g: x, b: x }).collect::<Vec<_>>();
    let mut limits = sRGB::all_colors().collect::<Vec<_>>();
    limits.shuffle(&mut rng);

    let oklab_colors = limits.par_iter().map(|srgb_color| srgb_color.to_oklab());

    oklab_colors.clone().for_each(|original| {
        /*
        let max_delta = oklab_colors
            .clone()
            .map(|sample| original.delta_E_Hyab(sample))
            .reduce(|| f64::NEG_INFINITY, |a, b| a.max(b));
        */
        let max_delta = original.saturation();

        let mut locked_saved_color = saved_color.lock();
        let mut locked_saved_delta = saved_delta.lock();

        if max_delta > *locked_saved_delta {
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
New best: sRGB(118, 121, 108) / Oklch(0.5695902238581334, 0.01980537412749601, 2.0523442950768525), 0.5897962567929554
error: process didn't exit successfully: `target\release\contrasting_colors.exe` (exit code: 0xc000013a, STATUS_CONTROL_C_EXIT)
^C

most saturated color (both .chroma()/.l and .saturation())
unreferenced white:
New best: sRGB(0, 0, 172) / Oklch(0.33649185331733306, 0.23316567643904332, -1.6746081505015074), 0.5695563289431641
saved_color: Mutex { data: sRGB { r: 0, g: 0, b: 172 } }
saved_delta: Mutex { data: 0.5695563289431641 }
Total time: 2.6673963

referenced white returns black

most colorful color (.chroma())
color reference doesn't matter
New best: sRGB(255, 0, 255) / Oklch(0.7016738558717924, 0.32249096477516426, -0.5521625213131971), 0.32249096477516426
saved_color: Mutex { data: sRGB { r: 255, g: 0, b: 255 } }
saved_delta: Mutex { data: 0.32249096477516426 }
Total time: 3.1147876
*/
