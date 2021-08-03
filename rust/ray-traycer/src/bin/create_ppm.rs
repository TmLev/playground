use std::{
    fs::File,
    io::{BufWriter, Write},
    rc::Rc,
};

use anyhow::Result;

use ray_traycer::{config, Camera, Hittable, HittableList, Point3, Ray, Sphere, Vec3};

type Color = Vec3;

fn main() -> Result<()> {
    // Logging

    env_logger::init();

    // Output

    let file = File::create(config::OUTPUT_PATH)?;
    let mut output = BufWriter::new(file);

    writeln!(&mut output, "P3")?;
    writeln!(
        &mut output,
        "{} {}",
        config::IMAGE_WIDTH,
        config::IMAGE_HEIGHT
    )?;
    writeln!(&mut output, "255")?;

    // World

    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render

    for row in (0..config::IMAGE_HEIGHT).rev() {
        log::info!("Lines remaining: {}/{}", row + 1, config::IMAGE_HEIGHT);

        for column in 0..config::IMAGE_WIDTH {
            let mut pixel_color = Color::default();

            for sample in 0..config::SAMPLES_PER_PIXEL {
                let u = (column as f64 + fastrand::f64()) / (config::IMAGE_WIDTH as f64 - 1.0);
                let v = (row as f64 + fastrand::f64()) / (config::IMAGE_HEIGHT as f64 - 1.0);
                let ray = camera.ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }

            write_color(&mut output, &pixel_color);
        }
    }

    log::info!("Done");

    Ok(())
}

fn write_color(output: &mut impl Write, color: &Color) {
    let scale = 1.0 / config::SAMPLES_PER_PIXEL as f64;

    let r = scale * color.x();
    let g = scale * color.y();
    let b = scale * color.z();

    writeln!(
        output,
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as u8,
        (256.0 * g.clamp(0.0, 0.999)) as u8,
        (256.0 * b.clamp(0.0, 0.999)) as u8,
    )
    .unwrap();
}

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    match world.hit(ray, 0.0, f64::INFINITY) {
        Some(record) => (*record.normal() + Color::new(1.0, 1.0, 1.0)) * 0.5,
        None => {
            let direction = ray.direction().to_unit();
            let t = 0.5 * (direction.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}
