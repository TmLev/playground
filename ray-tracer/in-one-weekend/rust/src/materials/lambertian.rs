use crate::{Color, HitRecord, Material, Ray, ScatterResult, Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> ScatterResult {
        let mut direction = *record.normal() + Vec3::random_unit();
        if direction.near_zero() {
            direction = *record.normal();
        }

        ScatterResult {
            scattered: Some(Ray::new(*record.point(), direction)),
            attenuation: Some(self.albedo),
            happened: true,
        }
    }
}
