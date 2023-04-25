use crate::{ray::Ray, shape::HitRecord, vector::Vector};

use super::Material;

pub struct Metal {
    albedo: Vector,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, _: bool) -> Option<Ray> {
        let reflected = r_in.direction.unit_vector().reflect(rec.normal);
        let scattered = Ray::new(
            rec.point,
            reflected + self.fuzz * Vector::random_in_unit_sphere().unit_vector(),
        );
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some(scattered)
        } else {
            None
        }
    }

    fn get_attenuation(&self) -> Vector {
        self.albedo
    }
}

impl Metal {
    pub fn new(albedo: Vector, fuzz: Option<f64>) -> Self {
        let fuzz = if let Some(fuzz) = fuzz { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}
