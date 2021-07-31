use std::{fs::File, io::Write};

use anyhow::Result;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

const OUTPUT_PATH: &str = "image.ppm";

fn main() -> Result<()> {
    let mut ppm = Vec::with_capacity(
        1 // Format
            + 1 // Sizes
            + 1 // Max color
            + IMAGE_HEIGHT * IMAGE_WIDTH,
    );

    ppm.push("P3".to_string());
    ppm.push(format!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT));
    ppm.push("255".to_string());

    for row in (0..IMAGE_HEIGHT).rev() {
        for column in 0..IMAGE_WIDTH {
            let r = (column as f32) / (IMAGE_WIDTH as f32 - 1f32);
            let g = (row as f32) / (IMAGE_HEIGHT as f32 - 1f32);
            let b = 0.25f32;

            let r = (256f32 * r) as u8;
            let g = (256f32 * g) as u8;
            let b = (256f32 * b) as u8;

            ppm.push(format!("{} {} {}", r, g, b));
        }
    }

    let mut output = File::create(OUTPUT_PATH)?;
    writeln!(&mut output, "{}", ppm.join("\n")).unwrap();

    Ok(())
}
