use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::Result;

use ray_traycer::Vec3;

const OUTPUT_PATH: &str = "image.ppm";

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

type Color = Vec3;

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
            let r = (column as f64) / (IMAGE_WIDTH as f64 - 1f64);
            let g = (row as f64) / (IMAGE_HEIGHT as f64 - 1f64);
            let b = 0.25f64;
            write_color(&mut output, Color::new(r, g, b));
        }
    }

    log::info!("Done");

    Ok(())
}

fn write_color(output: &mut impl Write, color: Color) {
    writeln!(
        output,
        "{} {} {}",
        (256f64 * color.x()) as u8,
        (256f64 * color.y()) as u8,
        (256f64 * color.z()) as u8,
    )
    .unwrap();
}
