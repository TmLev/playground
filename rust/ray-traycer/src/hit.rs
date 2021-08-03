use std::rc::Rc;

use crate::{Point3, Ray, Vec3};

pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, t: f64) -> Self {
        Self {
            point,
            normal,
            t,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(hittable: Rc<dyn Hittable>) -> Self {
        let mut list = Self::default();
        list.add(hittable);
        list
    }

    pub fn add(&mut self, hittable: Rc<dyn Hittable>) {
        self.objects.push(hittable);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut record: Option<HitRecord> = None;
        let mut closest = t_max;

        for object in self.objects.iter() {
            match object.hit(ray, t_min, closest) {
                None => continue,
                Some(hit) => {
                    closest = hit.t;
                    record = Some(hit);
                }
            }
        }

        record
    }
}
