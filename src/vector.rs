use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
};

use crate::{clamp, random_f64};

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul<Self> for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.0} {:.0} {:.0}", self.x, self.y, self.z)
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn random() -> Self {
        Self::new(random_f64(), random_f64(), random_f64())
    }
    pub fn random_in_unit_sphere() -> Self {
        let mut result = None;
        while result.is_none() {
            let p = Self::random() * 2.0 - Self::new(1., 1., 1.);
            if p.length_squared() >= 1.0 {
                continue;
            };
            result = Some(p);
        }
        result.unwrap()
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn get_string(&self, samples_per_pixel: i32) -> String {
        let mut gamma_color = Self {
            x: (self.x / samples_per_pixel as f64).sqrt(),
            y: (self.y / samples_per_pixel as f64).sqrt(),
            z: (self.z / samples_per_pixel as f64).sqrt(),
        };

        gamma_color.clamp(0.0, 1.0);
        format!("{}", gamma_color * 256.0)
    }
    pub fn clamp(&mut self, min: f64, max: f64) {
        *self = Self {
            x: clamp(self.x, min, max),
            y: clamp(self.y, min, max),
            z: clamp(self.z, min, max),
        }
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
    pub fn reflect(&self, n: Self) -> Self {
        *self - 2.0 * self.dot(&n) * n
    }
    pub fn refract(&self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = n.dot(&-*self).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }
    pub fn random_in_unit_disk() -> Self {
        let mut p = None;
        while p.is_none() {
            let q = Vector::new(random_f64() * 2.0 - 1.0, random_f64() * 2.0 - 1.0, 0.0);
            if q.length_squared() < 1.0 {
                p = Some(q)
            }
        }
        p.unwrap()
    }
}
