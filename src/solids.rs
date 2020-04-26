use crate::vec::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vec3, radius : f64) -> Self {
        Self {
            center : center,
            radius: radius
        }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}