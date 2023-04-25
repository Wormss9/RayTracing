use crate::vector::Vector;

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, time: f64) -> Vector {
        self.origin + self.direction * time
    }
}
