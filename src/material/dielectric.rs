use crate::{ray::Ray, shape::HitRecord, vector::Vector, random_f64};

use super::Material;

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, outward: bool) -> Option<Ray> {
        let refraction_ratio = if outward {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = r_in.direction.unit_vector();

        let cos_theta = rec.normal.dot(&-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;



        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_f64() {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        Some(Ray::new(rec.point, direction))
    }

    fn get_attenuation(&self) -> Vector {
        Vector::new(1.0, 1.0, 1.0)
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
