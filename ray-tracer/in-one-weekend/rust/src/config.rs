pub const OUTPUT_PATH: &str = "image.ppm";

pub const ASPECT_RATIO: f64 = 3.0 / 2.0;

pub const IMAGE_WIDTH: usize = 1200;
pub const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

pub const SAMPLES_PER_PIXEL: usize = 500;

pub const MAX_DEPTH: usize = 50;
