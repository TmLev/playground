use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::Result;

use ray_traycer::{Point3, Ray, Vec3};

const OUTPUT_PATH: &str = "image.ppm";

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

type Color = Vec3;

fn main() -> Result<()> {
    env_logger::init();

    let file = File::create(OUTPUT_PATH)?;
    let mut output = BufWriter::new(file);

    writeln!(&mut output, "P3")?;
    writeln!(&mut output, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(&mut output, "255")?;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::default();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for row in (0..IMAGE_HEIGHT).rev() {
        log::info!("Lines remaining: {}/{}", row + 1, IMAGE_HEIGHT);

        for column in 0..IMAGE_WIDTH {
            let u = (column as f64) / (IMAGE_WIDTH as f64 - 1.0);
            let v = (row as f64) / (IMAGE_HEIGHT as f64 - 1.0);
            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            write_color(&mut output, ray_color(&ray));
        }
    }

    log::info!("Done");

    Ok(())
}

fn write_color(output: &mut impl Write, color: Color) {
    writeln!(
        output,
        "{} {} {}",
        (256.0 * color.x()) as u8,
        (256.0 * color.y()) as u8,
        (256.0 * color.z()) as u8,
    )
    .unwrap();
}

fn ray_color(ray: &Ray) -> Color {
    let direction = ray.direction().to_unit();
    let t = 0.5 * (direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
