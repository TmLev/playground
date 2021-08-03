use crate::{Color, HitRecord, Ray};

mod lambertian;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult;
}

pub struct ScatterResult {
    pub attenuation: Option<Color>,
    pub scattered: Option<Ray>,
    pub happened: bool,
}

impl Default for ScatterResult {
    fn default() -> Self {
        Self {
            attenuation: None,
            scattered: None,
            happened: false,
        }
    }
}
