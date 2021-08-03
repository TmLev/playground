use crate::{Color, HitRecord, Material, Ray, ScatterResult, Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult {
        let reflected = ray.direction().to_unit().reflect(record.normal());
        let scattered = Ray::new(
            *record.point(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        ScatterResult {
            attenuation: Some(self.albedo),
            scattered: Some(scattered),
            happened: scattered.direction().dot(record.normal()) > 0.0,
        }
    }
}
