use std::{
    fs::File,
    io::{BufWriter, Write},
    rc::Rc,
};

use anyhow::Result;

use ray_traycer::{config, materials, Camera, Color, Hittable, HittableList, Point3, Ray, Sphere};

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

    let material_ground = Rc::new(materials::Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(materials::Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(materials::Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(materials::Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let camera = Camera::new();

    // Render

    for row in (0..config::IMAGE_HEIGHT).rev() {
        log::info!("Lines remaining: {}/{}", row + 1, config::IMAGE_HEIGHT);

        for column in 0..config::IMAGE_WIDTH {
            let mut pixel_color = Color::default();

            for _ in 0..config::SAMPLES_PER_PIXEL {
                let u = (column as f64 + fastrand::f64()) / (config::IMAGE_WIDTH as f64 - 1.0);
                let v = (row as f64 + fastrand::f64()) / (config::IMAGE_HEIGHT as f64 - 1.0);
                let ray = camera.ray(u, v);
                pixel_color += ray_color(&ray, &world, config::MAX_DEPTH);
            }

            write_color(&mut output, &pixel_color);
        }
    }

    log::info!("Done");

    Ok(())
}

fn write_color(output: &mut impl Write, color: &Color) {
    let scale = 1.0 / config::SAMPLES_PER_PIXEL as f64;

    let r = (scale * color.x()).sqrt();
    let g = (scale * color.y()).sqrt();
    let b = (scale * color.z()).sqrt();

    writeln!(
        output,
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as u8,
        (256.0 * g.clamp(0.0, 0.999)) as u8,
        (256.0 * b.clamp(0.0, 0.999)) as u8,
    )
    .unwrap();
}

fn ray_color(ray: &Ray, world: &impl Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color::default();
    }

    match world.hit(ray, 0.001, f64::INFINITY) {
        Some(record) => {
            let scatter = record.material().scatter(ray, &record);
            match scatter.happened {
                false => Color::default(),
                true => {
                    scatter.attenuation.unwrap()
                        * ray_color(&scatter.scattered.unwrap(), world, depth - 1)
                }
            }
        }
        None => {
            let direction = ray.direction().to_unit();
            let t = 0.5 * (direction.y() + 1.0);
            (1.0 - t) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}
