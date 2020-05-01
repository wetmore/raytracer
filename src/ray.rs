use crate::vec::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    vec: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray {
            origin: origin,
            vec: dir,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.vec
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.vec * t
    }
}
