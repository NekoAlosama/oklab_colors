use crate::oklab::*;
use crate::rgb::*;
use itertools::Itertools;
use parking_lot::Mutex;
use rayon::prelude::*;

pub fn main() {
    let start_time = std::time::SystemTime::now();

    let mut saved_colors = vec![sRGB { r: 0, g: 0, b: 0 }];

    for _ in 1..=8 {
        let saved_delta = Mutex::new(f64::MIN);
        let saved_color = Mutex::new(sRGB {
            r: 99,
            g: 99,
            b: 99,
        });
        let starting_colors = saved_colors
            .iter()
            .map(|color| color.to_oklab().to_d65_reference());

        sRGB::all_colors()
            .par_bridge()
            .filter(|srgb| {
                srgb.to_oklab().to_d65_reference().delta_E_ab(Oklab {
                    l: 0.0,
                    a: 0.0,
                    b: 0.0,
                    ..Default::default()
                }) > 0.5_f64
            })
            .for_each(|test_srgb| {
                let test_colors = starting_colors
                    .clone()
                    .chain(std::iter::once(test_srgb.to_oklab().to_d65_reference()));

                let all_deltas = test_colors
                    .permutations(2)
                    .map(|vector| vector[0].delta_E_ab(vector[1]));

                // TODO: find a good averaging method
                let delta = all_deltas.map(|delta: f64| delta.ln()).sum::<f64>();

                let mut locked_saved_delta = saved_delta.lock();
                let mut locked_saved_color = saved_color.lock();

                if delta > *locked_saved_delta {
                    *locked_saved_delta = delta;
                    *locked_saved_color = test_srgb;
                }
            });

        let saved_color = saved_color.into_inner();

        if saved_color
            == (sRGB {
                r: 99,
                g: 99,
                b: 99,
            })
        {
            println!("Finished with {:?}", saved_colors.len());
            std::process::exit(99);
        } else {
            println!(
                "{saved_color}, {}, {:.5?}",
                Oklab {
                    l: (saved_color.to_oklab().l * 2.0 / 3.0),
                    a: saved_color.to_oklab().a / 3.0,
                    b: saved_color.to_oklab().b / 3.0,
                    ..Default::default()
                }
                .to_srgb_closest(),
                -saved_delta.into_inner().recip()
            );
            saved_colors.push(saved_color)
        }
    }

    println!(
        "Total time: {:.3?}",
        start_time.elapsed().expect("Time went backwards")
    );
}

/*
start with black, uses ref_l(), no filter
geometric mean for delta (just logarithmic sum)
(255, 255, 255), (148, 148, 148), -0.00000
(255, 0, 255), (122, 66, 120), -2.12858
(0, 255, 0), (86, 136, 82), -5.42440
(0, 0, 255), (18, 41, 98), -9.87676
(106, 0, 0), (44, 17, 14), -16.72142
(255, 255, 0), (144, 146, 95), -25.08952
(0, 0, 82), (2, 6, 26), -35.10770
(255, 0, 0), (116, 57, 48), -46.70339
Total time: 49.898s


start with black, uses ref_l(), filter colors <0.5 delta_e() from black
geometric mean for delta (just logarithmic sum)
(255, 255, 255), (148, 148, 148), -0.00000
(255, 0, 255), (122, 66, 120), -2.12858
(0, 255, 0), (86, 136, 82), -5.42440
(6, 54, 255), (27, 49, 101), -10.24664
(202, 0, 0), (91, 43, 36), -17.45836
(255, 255, 0), (144, 146, 95), -26.51181
(0, 135, 0), (41, 69, 39), -37.41459
(137, 0, 255), (67, 48, 106), -50.81634


start with black, uses ref_l(), filter colors <0.618... delta_e() from black
geometric mean for delta (just logarithmic sum)


haven't read the code after refactor to see what the following is supposed to be
(255, 255, 255), (148, 148, 148), 65756511.31290
(255, 0, 255), (122, 66, 120), 0.46980
(0, 255, 0), (86, 136, 82), 0.18435
(6, 54, 255), (27, 49, 101), 0.09759
(202, 0, 0), (91, 43, 36), 0.05728
(255, 255, 0), (144, 146, 95), 0.03772
(0, 135, 0), (41, 69, 39), 0.02673
(137, 0, 255), (67, 48, 106), 0.01968
Total time: 51.928s
*/
