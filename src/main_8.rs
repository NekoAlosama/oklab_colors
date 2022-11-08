#![allow(unused_imports)]
use itertools::Itertools;
use oklab::Oklab;
use rayon::prelude::*;
use rgb::SRgb;

mod oklab;
mod rgb;

const N: usize = 64; // factors of 256: 1, 2, 4, 8, 16, 32, 64, 128, 256.

fn main() {
    let start_time = std::time::SystemTime::now();

    let background = SRgb { r: 0, g: 0, b: 0 }.srgb_to_oklab();

    let mut srgb_colors: Vec<Oklab> = Vec::with_capacity(2_usize.pow(8).pow(3));
    for r in (0..=256_usize).step_by(N) {
        for g in (0..=256_usize).step_by(N) {
            for b in (0..=256_usize).step_by(N) {
                srgb_colors.push(
                    SRgb {
                        r: r.saturating_sub(1) as u8,
                        g: g.saturating_sub(1) as u8,
                        b: b.saturating_sub(1) as u8,
                    }
                    .srgb_to_oklab(),
                );
            }
        }
    }

    let saved = std::sync::Mutex::new({
        (
            f64::MIN,
            f64::MIN,
            Oklab {
                l: f64::NAN,
                a: f64::NAN,
                b: f64::NAN,
            },
            Oklab {
                l: f64::NAN,
                a: f64::NAN,
                b: f64::NAN,
            },
            Oklab {
                l: f64::NAN,
                a: f64::NAN,
                b: f64::NAN,
            },
            Oklab {
                l: f64::NAN,
                a: f64::NAN,
                b: f64::NAN,
            },
            Oklab {
                l: f64::NAN,
                a: f64::NAN,
                b: f64::NAN,
            },
            Oklab {
                l: f64::NAN,
                a: f64::NAN,
                b: f64::NAN,
            },
            Oklab {
                l: f64::NAN,
                a: f64::NAN,
                b: f64::NAN,
            },
            Oklab {
                l: f64::NAN,
                a: f64::NAN,
                b: f64::NAN,
            },
        )
    });

    srgb_colors
        .clone()
        .par_iter()
        .filter(|&color1| color1.delta_eok(background) > saved.lock().unwrap().0)
        .for_each(|color1| {
            srgb_colors
                .clone()
                .par_iter()
                .filter(|&color2| color2.delta_eok(background) > saved.lock().unwrap().0)
                .filter(|&color2| color2.delta_eok(*color1) > saved.lock().unwrap().0)
                .for_each(|color2| {
                    srgb_colors
                        .clone()
                        .par_iter()
                        .filter(|&color3| color3.delta_eok(background) > saved.lock().unwrap().0)
                        .filter(|&color3| color3.delta_eok(*color1) > saved.lock().unwrap().0)
                        .filter(|&color3| color3.delta_eok(*color2) > saved.lock().unwrap().0)
                        .for_each(|color3| {
                            srgb_colors
                                .clone()
                                .par_iter()
                                .filter(|&color4| {
                                    color4.delta_eok(background) > saved.lock().unwrap().0
                                })
                                .filter(|&color4| {
                                    color4.delta_eok(*color1) > saved.lock().unwrap().0
                                })
                                .filter(|&color4| {
                                    color4.delta_eok(*color2) > saved.lock().unwrap().0
                                })
                                .filter(|&color4| {
                                    color4.delta_eok(*color3) > saved.lock().unwrap().0
                                })
                                .for_each(|color4| {
                                    srgb_colors
                                        .clone()
                                        .par_iter()
                                        .filter(|&color5| {
                                            color5.delta_eok(background) > saved.lock().unwrap().0
                                        })
                                        .filter(|&color5| {
                                            color5.delta_eok(*color1) > saved.lock().unwrap().0
                                        })
                                        .filter(|&color5| {
                                            color5.delta_eok(*color2) > saved.lock().unwrap().0
                                        })
                                        .filter(|&color5| {
                                            color5.delta_eok(*color3) > saved.lock().unwrap().0
                                        })
                                        .filter(|&color5| {
                                            color5.delta_eok(*color4) > saved.lock().unwrap().0
                                        })
                                        .for_each(|color5| {
                                            srgb_colors
                                                .clone()
                                                .par_iter()
                                                .filter(|&color6| {
                                                    color6.delta_eok(background)
                                                        > saved.lock().unwrap().0
                                                })
                                                .filter(|&color6| {
                                                    color6.delta_eok(*color1)
                                                        > saved.lock().unwrap().0
                                                })
                                                .filter(|&color6| {
                                                    color6.delta_eok(*color2)
                                                        > saved.lock().unwrap().0
                                                })
                                                .filter(|&color6| {
                                                    color6.delta_eok(*color3)
                                                        > saved.lock().unwrap().0
                                                })
                                                .filter(|&color6| {
                                                    color6.delta_eok(*color4)
                                                        > saved.lock().unwrap().0
                                                })
                                                .filter(|&color6| {
                                                    color6.delta_eok(*color5)
                                                        > saved.lock().unwrap().0
                                                })
                                                .for_each(|color6| {
                                                    srgb_colors
                                                        .clone()
                                                        .par_iter()
                                                        .filter(|&color7| {
                                                            color7.delta_eok(background)
                                                                > saved.lock().unwrap().0
                                                        })
                                                        .filter(|&color7| {
                                                            color7.delta_eok(*color1)
                                                                > saved.lock().unwrap().0
                                                        })
                                                        .filter(|&color7| {
                                                            color7.delta_eok(*color2)
                                                                > saved.lock().unwrap().0
                                                        })
                                                        .filter(|&color7| {
                                                            color7.delta_eok(*color3)
                                                                > saved.lock().unwrap().0
                                                        })
                                                        .filter(|&color7| {
                                                            color7.delta_eok(*color4)
                                                                > saved.lock().unwrap().0
                                                        })
                                                        .filter(|&color7| {
                                                            color7.delta_eok(*color5)
                                                                > saved.lock().unwrap().0
                                                        })
                                                        .filter(|&color7| {
                                                            color7.delta_eok(*color6)
                                                                > saved.lock().unwrap().0
                                                        })
                                                        .for_each(|color7| {
                                                            srgb_colors
                                                                .clone()
                                                                .par_iter()
                                                                .filter(|&color8| {
                                                                    color8.delta_eok(background)
                                                                        > saved.lock().unwrap().0
                                                                })
                                                                .filter(|&color8| {
                                                                    color8.delta_eok(*color1)
                                                                        > saved.lock().unwrap().0
                                                                })
                                                                .filter(|&color8| {
                                                                    color8.delta_eok(*color2)
                                                                        > saved.lock().unwrap().0
                                                                })
                                                                .filter(|&color8| {
                                                                    color8.delta_eok(*color3)
                                                                        > saved.lock().unwrap().0
                                                                })
                                                                .filter(|&color8| {
                                                                    color8.delta_eok(*color4)
                                                                        > saved.lock().unwrap().0
                                                                })
                                                                .filter(|&color8| {
                                                                    color8.delta_eok(*color5)
                                                                        > saved.lock().unwrap().0
                                                                })
                                                                .filter(|&color8| {
                                                                    color8.delta_eok(*color6)
                                                                        > saved.lock().unwrap().0
                                                                })
                                                                .filter(|&color8| {
                                                                    color8.delta_eok(*color7)
                                                                        > saved.lock().unwrap().0
                                                                })
                                                                .map(|color8| {
                                                                    compute_job(
                                                                        background, *color1,
                                                                        *color2, *color3, *color4,
                                                                        *color5, *color6, *color7,
                                                                        *color8,
                                                                    )
                                                                })
                                                                .for_each(|item| {
                                                                    if item.0
                                                                        > saved.lock().unwrap().0
                                                                    {
                                                                        *saved.lock().unwrap() =
                                                                            item;
                                                                    }
                                                                })
                                                        })
                                                })
                                        })
                                })
                        })
                })
        });

    let saved = saved.lock().unwrap();
    println!("{N:?}");
    println!("{saved:?}");
    println!("{:?}, {:?}", saved.0, saved.1);
    println!(
        "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
        saved.2.oklab_to_srgb(),
        saved.3.oklab_to_srgb(),
        saved.4.oklab_to_srgb(),
        saved.5.oklab_to_srgb(),
        saved.6.oklab_to_srgb(),
        saved.7.oklab_to_srgb(),
        saved.8.oklab_to_srgb(),
        saved.9.oklab_to_srgb(),
    );

    println!(
        "Time: {:?}",
        start_time.elapsed().expect("Time went backwards")
    );
}

// c1, c2, lowest difference with each other, sum of differences
fn compute_job(
    bg: Oklab,
    c1: Oklab,
    c2: Oklab,
    c3: Oklab,
    c4: Oklab,
    c5: Oklab,
    c6: Oklab,
    c7: Oklab,
    c8: Oklab,
) -> (
    f64,
    f64,
    Oklab,
    Oklab,
    Oklab,
    Oklab,
    Oklab,
    Oklab,
    Oklab,
    Oklab,
) {
    let delta1 = c1.delta_eok(bg);

    let delta2 = c2.delta_eok(bg);
    let delta3 = c2.delta_eok(c1);

    let delta4 = c3.delta_eok(bg);
    let delta5 = c3.delta_eok(c1);
    let delta6 = c3.delta_eok(c2);

    let delta7 = c4.delta_eok(bg);
    let delta8 = c4.delta_eok(c1);
    let delta9 = c4.delta_eok(c2);
    let delta10 = c4.delta_eok(c3);

    let delta11 = c5.delta_eok(bg);
    let delta12 = c5.delta_eok(c1);
    let delta13 = c5.delta_eok(c2);
    let delta14 = c5.delta_eok(c3);
    let delta15 = c5.delta_eok(c4);

    let delta16 = c6.delta_eok(bg);
    let delta17 = c6.delta_eok(c1);
    let delta18 = c6.delta_eok(c2);
    let delta19 = c6.delta_eok(c3);
    let delta20 = c6.delta_eok(c4);
    let delta21 = c6.delta_eok(c5);

    let delta22 = c7.delta_eok(bg);
    let delta23 = c7.delta_eok(c1);
    let delta24 = c7.delta_eok(c2);
    let delta25 = c7.delta_eok(c3);
    let delta26 = c7.delta_eok(c4);
    let delta27 = c7.delta_eok(c5);
    let delta28 = c7.delta_eok(c6);

    let delta29 = c8.delta_eok(bg);
    let delta30 = c8.delta_eok(c1);
    let delta31 = c8.delta_eok(c2);
    let delta32 = c8.delta_eok(c3);
    let delta33 = c8.delta_eok(c4);
    let delta34 = c8.delta_eok(c5);
    let delta35 = c8.delta_eok(c6);
    let delta36 = c8.delta_eok(c7);

    let deltas = [
        delta1, delta2, delta3, delta4, delta5, delta6, delta7, delta8, delta9, delta10, delta11,
        delta12, delta13, delta14, delta15, delta16, delta17, delta18, delta19, delta20, delta21,
        delta22, delta23, delta24, delta25, delta26, delta27, delta28, delta29, delta30, delta31,
        delta32, delta33, delta34, delta35, delta36,
    ];
    (
        deltas.iter().fold(f64::INFINITY, |a, b| a.min(*b)),
        deltas.iter().sum(),
        c1,
        c2,
        c3,
        c4,
        c5,
        c6,
        c7,
        c8,
    )
}
