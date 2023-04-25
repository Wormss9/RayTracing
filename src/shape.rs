pub mod sphere;

pub use sphere::Sphere;
use std::sync::Arc;

use crate::{material::Material, ray::Ray, vector::Vector};

pub struct HitRecord {
    pub point: Vector,
    pub normal: Vector,
    pub material: Arc<dyn Material>,
    t: f64,
}

impl HitRecord {
    pub fn front_face(ray: &Ray, outward_normal: &Vector) -> bool {
        ray.direction.dot(outward_normal) < 0.
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector) {
        self.normal = if Self::front_face(ray, &outward_normal) {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Ray>;
}

#[derive(Clone)]
pub struct HittableList(pub Vec<Arc<dyn Hittable + Send + Sync>>);

impl HittableList {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn add(&mut self, other: Arc<dyn Hittable + Send + Sync>) {
        self.0.push(other)
    }
    pub fn hit(
        &self,
        ray: &Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<(Arc<dyn Hittable + Send + Sync>, HitRecord)> {
        let mut closest_so_far = t_max;
        let mut closest_rec = None;

        for object in self.0.iter() {
            if let Some(current) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = current.t;
                closest_rec = Some((object.clone(), current));
            }
        }

        closest_rec
    }
}
