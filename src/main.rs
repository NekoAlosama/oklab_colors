mod oklab;
mod rgb;
use crate::oklab::Oklch;
use crate::rgb::sRGB;
use itertools::Itertools;
use parking_lot::Mutex;
use rayon::prelude::*;

fn main() {
    let start_time = std::time::SystemTime::now();

    /*
    let mut saved_colors = vec![
        sRGB::new(255, 0, 0),
        sRGB::new(128, 64, 64),
        sRGB::new(0, 64, 255),
        sRGB::new(64, 79, 128),
        sRGB::new(0, 255, 0),
        sRGB::new(64, 128, 64),
        sRGB::new(255, 255, 0),
        sRGB::new(128, 128, 64),
        sRGB::new(255, 0, 191),
        sRGB::new(128, 64, 112),
        sRGB::new(255, 164, 0),
        sRGB::new(128, 105, 64),
        sRGB::new(0, 255, 255),
        sRGB::new(64, 128, 128),
        sRGB::new(168, 168, 168),
        sRGB::new(64, 64, 64),
    ];*/

    /*
    for color in &saved_colors {
        let oklch_version = color.to_oklch();
        println!("{oklch_version}");
    }
    */

    let mut saved_colors = vec![sRGB::new(0, 0, 0), sRGB::new(255, 255, 255)];

    for _ in 1..=8 {
        let saved_delta = Mutex::new(f64::NEG_INFINITY);
        let saved_color = Mutex::new(sRGB::default());
        let starting_colors = saved_colors.iter().map(|color| color.to_oklab());

        sRGB::all_colors().par_bridge().for_each(|test_srgb| {
            if test_srgb.to_oklab().l < 0.5 {
                return;
            }
            let all_combos = starting_colors
                .clone()
                .chain(std::iter::once(test_srgb.to_oklab()))
                .permutations(2);

            // TODO: find a good averaging method
            let delta = all_combos
                .map(|vector| vector[0].delta_E_Hyab(vector[1]))
                .fold(f64::INFINITY, |a: f64, b: f64| a.min(b));

            let mut locked_saved_delta = saved_delta.lock();
            let mut locked_saved_color = saved_color.lock();

            if (delta > *locked_saved_delta) && (!saved_colors.contains(&test_srgb)) {
                *locked_saved_delta = delta;
                drop(locked_saved_delta);
                *locked_saved_color = test_srgb;
                drop(locked_saved_color);
            }
        });

        let saved_color = saved_color.into_inner();

        println!(
            "{saved_color}, {}, {:.5?}",
            Oklch {
                l: 0.5,
                c: 0.1,
                h: saved_color.to_oklch().h,
                d65_reference_l: false
            }
            .to_srgb_closest(),
            saved_delta.into_inner()
        );
        saved_colors.push(saved_color);
    }

    println!(
        "Total time: {:.3?}",
        start_time.elapsed().expect("Time went backwards")
    );
}

/*
subtract by zero colors
commented lines have noticable hue shifts
//Oklch(0.6279553606145516, 0.25768330773615666, 0.510227549756395)
//Oklch(0.4524084677079231, 0.08853463578288609, 0.3695431138720326)

//Oklch(0.501089920528117, 0.2803495786018907, -1.6786361062387636)
//Oklch(0.43822066892689765, 0.0832384724669101, -1.5736803291746473)

Oklch(0.8664396115356694, 0.2948272403370164, 2.4870128323373977)
Oklch(0.5410743407133864, 0.11657250255737486, 2.508996882542441)

Oklch(0.9679827203267873, 0.21100590772552363, 1.915834517121069)
Oklch(0.5853101665649572, 0.08630918901587094, 1.8988173717342338)

//Oklch(0.6684886057526132, 0.28566448101957015, -0.2825842243019164)
//Oklch(0.469028313197212, 0.10842348691193887, -0.3932634377277638)

//Oklch(0.7909275304925968, 0.1711762561648887, 1.2243938983650822)
//Oklch(0.5333779116201199, 0.06460437927284683, 1.4156197752849016)

Oklch(0.9053992300557675, 0.15455001106436875, -2.883825885121417)
Oklch(0.557845887650596, 0.06570032927950503, -2.8722246272059704)

Oklch(0.7315949801397452, 0.00000002726946816141894, 1.568624492457541)
Oklch(0.3714949436051885, 0.000000013847100919398513, 1.5686244890183265)

let mut saved_colors = vec![
        sRGB::new(255, 0, 0),
        sRGB::new(128, 64, 64),

        sRGB::new(0, 64, 255),
        sRGB::new(64, 79, 128),

        sRGB::new(0, 255, 0),
        sRGB::new(64, 128, 64),

        sRGB::new(255, 255, 0),
        sRGB::new(128, 128, 64),

        sRGB::new(255, 0, 191),
        sRGB::new(128, 64, 112),

        sRGB::new(255, 164, 0),
        sRGB::new(128, 105, 64),

        sRGB::new(0, 255, 255),
        sRGB::new(64, 128, 128),

        sRGB::new(168, 168, 168),
        sRGB::new(64, 64, 64),
    ];
*/

/*
sRGB(98, 0, 255), sRGB(93, 90, 154), 0.79392
sRGB(0, 162, 0), sRGB(63, 113, 59), 0.59288
sRGB(255, 91, 175), sRGB(142, 73, 105), 0.50259
sRGB(162, 66, 0), sRGB(145, 79, 47), 0.38137
sRGB(0, 197, 255), sRGB(0, 109, 144), 0.38101
sRGB(255, 177, 0), sRGB(131, 90, 13), 0.35599
sRGB(0, 255, 0), sRGB(63, 113, 59), 0.32501
sRGB(126, 119, 181), sRGB(97, 89, 153), 0.30045
Total time: 45.812s
*/
