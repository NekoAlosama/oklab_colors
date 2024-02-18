mod color_targeting;
mod oklab;
mod opposing_colors;
mod rgb;

fn main() {
    let mut color = (rgb::sRGB {
        r: 255,
        g: 0,
        b: 0,
    }).to_oklab();
    println!("{}", color);
    println!("{}", color.to_lrgb());
    println!("{}", color.to_srgb_closest());
}
