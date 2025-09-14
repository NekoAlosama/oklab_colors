use crate::rgb::*;
use itertools::Itertools;
use parking_lot::Mutex;
use rayon::prelude::*;

pub fn main() {
    let start_time = std::time::Instant::now();

    let saved_delta = Mutex::new(f64::NEG_INFINITY);
    let saved_color_rgb = Mutex::new(sRGB::default());

    let saved_colors_rgb = vec![
        sRGB { r: 0, g: 0, b: 0 },
    ];
    let saved_colors_oklab = saved_colors_rgb.iter().map(|color| color.to_oklab().to_d65_white());

    let lowest = saved_colors_oklab
        .clone()
        .combinations(2)
        .map(|vector| vector[0].delta_E_ab(vector[1]))
        .fold(f64::INFINITY, |a: f64, b: f64| a.min(b));

    sRGB::all_colors().par_bridge().for_each(|test_srgb| {
        let delta = saved_colors_oklab
            .clone()
            .map(|color| color.delta_E_ab(test_srgb.to_oklab().to_d65_white()))
            .fold(lowest, |a: f64, b: f64| a.min(b));

        let mut locked_saved_delta = saved_delta.lock();
        let mut locked_saved_color = saved_color_rgb.lock();

        if (delta > *locked_saved_delta) && (!saved_colors_rgb.contains(&test_srgb)) {
            *locked_saved_delta = delta;
            *locked_saved_color = test_srgb;
        }
    });

    let saved_color_rgb = saved_color_rgb.into_inner();

    println!(
        "{saved_color_rgb}, {}, {}",
        saved_color_rgb.to_oklch(),
        saved_delta.into_inner()
    );

    let end_time = start_time.elapsed();
    dbg!(end_time);
}

/*
unreferenced, black vs white, ab
sRGB(98, 0, 255), Oklch(0.5001367631674302, 0.29405950783892687, -1.3252001633498984), 0.5799433286232789
unreferenced, black vs white, Hyab
sRGB(98, 0, 255), Oklch(0.5001367631674302, 0.29405950783892687, -1.3252001633498984), 0.7939227742807731

unreferenced, black vs yellow, ab
sRGB(161, 0, 255), Oklch(0.5696927786944534, 0.29693861855652437, -0.971201982351409), 0.6423428778734613
unreferenced, black vs yellow, Hyab
sRGB(174, 0, 255), Oklch(0.5864670698235025, 0.2994664394891484, -0.9042730366005025), 0.8856062180812202


d65, black vs white, ab
sRGB(160, 2, 255), Oklch(0.5688374562362724, 0.29632129775818167, -0.9769775939895653), 0.5812103388060578
d65, black vs white, Hyab
sRGB(160, 2, 255), Oklch(0.5688374562362724, 0.29632129775818167, -0.9769775939895653), 0.7963204441874125

d65, black vs yellow, ab
sRGB(202, 2, 255), Oklch(0.6248744410520418, 0.30582574596512696, -0.7705868552043711), 0.6420464025428635
d65, black vs yellow, Hyab
sRGB(211, 0, 255), Oklch(0.637234344283238, 0.3087350326808623, -0.7301283624323093), 0.8875289083953797


unreferenced, ab
sRGB(255, 255, 255), Oklch(0.9999999934735462, 0.00000003727399549045184, 1.5686244886369696), 0.9999999934735468
unreferenced, Hyab
sRGB(255, 255, 0), Oklch(0.9679827203267873, 0.21100590772552363, 1.915834517121069), 1.178988628052311

d65, ab
sRGB(255, 255, 255), Oklch(0.9999999934735462, 0.00000003727399549045184, 1.5686244886369696), 0.9999999923961905
d65, Hyab
sRGB(255, 255, 0), Oklch(0.9679827203267873, 0.21100590772552363, 1.915834517121069), 1.1737103045344184


// blurple, mostly blue
unreferenced, black vs white, ab
sRGB(98, 0, 255), Oklch(0.5001367631674302, 0.29405950783892687, -1.3252001633498984), 0.5799433286232789
// seems like a sweet spot
unreferenced, black vs yellow, Hyab
sRGB(174, 0, 255), Oklch(0.5864670698235025, 0.2994664394891484, -0.9042730366005025), 0.8856062180812202

// close to sweet spot, but too blue
d65, black vs white, ab
sRGB(160, 2, 255), Oklch(0.5688374562362724, 0.29632129775818167, -0.9769775939895653), 0.5812103388060578
// too magenta/pink
d65, black vs yellow, Hyab
sRGB(211, 0, 255), Oklch(0.637234344283238, 0.3087350326808623, -0.7301283624323093), 0.8875289083953797
*/
