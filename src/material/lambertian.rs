use crate::{ray::Ray, shape::HitRecord, vector::Vector};

use super::Material;

pub struct Lambertian {
    albedo: Vector,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, _: bool) -> Option<Ray> {
        let mut scatter_direction = rec.normal + Vector::random().unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.point, scatter_direction);
        Some(scattered)
    }

    fn get_attenuation(&self) -> Vector {
        self.albedo
    }
}

impl Lambertian {
    pub fn new(albedo: Vector) -> Self {
        Self { albedo }
    }
}
