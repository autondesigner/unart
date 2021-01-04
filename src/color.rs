use palette::rgb::Rgb;
use palette::Hsv;
use rand::prelude::*;

pub struct Color {
    pub rgb: [u8; 3],
    pub hsv: [f32; 3],
}

impl Color {
    pub fn new(h: f32, s: f32, v: f32) -> Color {
        let hsv: [f32; 3] = [h, s, v];
        let mut rgb: [u8; 3] = [0, 0, 0];
        let palette_hsv = Hsv::new(h, s, v);
        let palette_rgb: Rgb = Rgb::from(palette_hsv);
        let rgb_tuple = palette_rgb.into_components();
        rgb[0] = (rgb_tuple.0 * 255.0) as u8;
        rgb[1] = (rgb_tuple.1 * 255.0) as u8;
        rgb[2] = (rgb_tuple.2 * 255.0) as u8;
        Color { hsv, rgb }
    }
}

pub fn build_colors(rng: &mut StdRng, mut colors_count: usize) -> Vec<Color> {
    colors_count -= 1;
    let mut colors = Vec::with_capacity(colors_count);
    let adder: f32 = 360f32 * (1.0 / 1.0) / colors_count as f32;
    let mut hue = rng.gen_range(0..360) as f32;
    //let mut hue = 180.0;
    colors.push(Color::new(0.0, 0.0, 1.0));
    for _i in 0..colors_count {
        hue += adder;
        hue %= 360.0;
        colors.push(Color::new(hue, 7.0 / 8.0, 1.0))
    }
    colors
}
