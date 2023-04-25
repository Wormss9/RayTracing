use std::sync::Arc;

use crate::{material::Material, vector::Vector};

use super::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Vector,
    pub radius: f64,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;

        let mut hit_record = HitRecord {
            t,
            point,
            normal,
            material: self.material.clone(),
        };

        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }

    fn scatter(&self, r_in: &crate::ray::Ray, rec: &HitRecord) -> Option<crate::ray::Ray> {
        let outward_normal = (rec.point - self.center) / self.radius;
        let normal = HitRecord::front_face(r_in, &outward_normal);
        self.material.scatter(r_in, rec, normal)
    }
}
