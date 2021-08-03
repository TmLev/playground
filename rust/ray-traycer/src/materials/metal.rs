use crate::{Color, HitRecord, Material, Ray, ScatterResult};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult {
        let reflected = ray.direction().to_unit().reflect(record.normal());
        let scattered = Ray::new(*record.point(), reflected);
        ScatterResult {
            attenuation: Some(self.albedo),
            scattered: Some(scattered),
            happened: scattered.direction().dot(record.normal()) > 0.0,
        }
    }
}
