use std::ops::{Add, Sub, Mul, Div, Neg};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn to_unit(&self) -> Vec3 {
        let l = self.length();
        Vec3::new(self.0 / l, self.1 / l, self.2 / l)
    }

    pub fn dot(v1 : Vec3, v2 : Vec3) -> f64 {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }

    pub fn random<T : Rng>(rng : &mut T) -> Vec3 {
        Vec3::new(rng.gen(),rng.gen(),rng.gen())
    }

    pub fn random_interval<T : Rng>(rng : &mut T, min : f64, max : f64) -> Vec3 {
        Vec3::new(rng.gen_range(min,max),rng.gen_range(min,max),rng.gen_range(min,max))
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self::new(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new(self * other.0, self * other.1, self * other.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        Vec3::new(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.0, -self.1, -self.2)
    }
}