use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::Result;

const OUTPUT_PATH: &str = "image.ppm";

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() -> Result<()> {
    env_logger::init();

    let file = File::create(OUTPUT_PATH)?;
    let mut output = BufWriter::new(file);

    writeln!(&mut output, "P3")?;
    writeln!(&mut output, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(&mut output, "255")?;

    for row in (0..IMAGE_HEIGHT).rev() {
        log::info!("Lines remaining: {}/{}", row + 1, IMAGE_HEIGHT);

        for column in 0..IMAGE_WIDTH {
            let r = (column as f32) / (IMAGE_WIDTH as f32 - 1f32);
            let g = (row as f32) / (IMAGE_HEIGHT as f32 - 1f32);
            let b = 0.25f32;

            let r = (256f32 * r) as u8;
            let g = (256f32 * g) as u8;
            let b = (256f32 * b) as u8;

            writeln!(&mut output, "{} {} {}", r, g, b)?;
        }
    }

    log::info!("Done");

    Ok(())
}
