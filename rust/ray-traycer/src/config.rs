pub const OUTPUT_PATH: &str = "image.ppm";

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub const IMAGE_WIDTH: usize = 400;
pub const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize
