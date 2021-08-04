use crate::{Color, HitRecord, Material, Ray, ScatterResult};

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if record.front_face() {
            self.index_of_refraction.recip()
        } else {
            self.index_of_refraction
        };

        let direction = ray.direction().to_unit();
        let cos_theta = (-direction).dot(record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > fastrand::f64() {
                direction.reflect(record.normal())
            } else {
                direction.refract(record.normal(), refraction_ratio)
            };

        ScatterResult {
            attenuation: Some(attenuation),
            scattered: Some(Ray::new(*record.point(), direction)),
            happened: true,
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
