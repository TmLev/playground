use crate::{Color, HitRecord, Ray};

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> ScatterResult;
}

// TODO(TmLev): Maybe `Option`-s can be removed.
// Revisit after finishing the book.
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
