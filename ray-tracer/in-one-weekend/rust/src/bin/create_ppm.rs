use std::{
    fs::File,
    io::{BufWriter, Write},
    rc::Rc,
};

use anyhow::Result;

use ray_tracer::{
    config, materials, Camera, Color, Hittable, HittableList, Point3, Ray, Sphere, Vec3,
};

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

    let world = random_scene();

    // Camera

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        20.0,
        config::ASPECT_RATIO,
        aperture,
        focus_dist,
    );

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

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Rc::new(materials::Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * fastrand::f64(),
                0.2,
                b as f64 + 0.9 * fastrand::f64(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            let choose_material = fastrand::f64();
            let material: Rc<dyn materials::Material> = if choose_material < 0.8 {
                // Diffuse
                let albedo = Color::random() * Color::random();
                Rc::new(materials::Lambertian::new(albedo))
            } else if choose_material < 0.95 {
                // Metal
                let albedo = Color::random_rng(0.5, 1.0);
                let fuzz = fastrand::f64() * 0.5;
                Rc::new(materials::Metal::new(albedo, fuzz))
            } else {
                // Glass
                Rc::new(materials::Dielectric::new(1.5))
            };

            world.add(Rc::new(Sphere::new(center, 0.2, material)));
        }
    }

    let material = Rc::new(materials::Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Rc::new(materials::Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Rc::new(materials::Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    world
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
