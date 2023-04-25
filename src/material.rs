mod dielectric;
mod lambertian;
mod metal;

pub use self::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use crate::{ray::Ray, shape::HitRecord, vector::Vector};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, outward: bool) -> Option<Ray>;
    fn get_attenuation(&self) -> Vector;
}
