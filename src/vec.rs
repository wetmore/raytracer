use std::ops::{Add, Sub, Mul, Div, Neg};
use rand::Rng;
use std::f64::{consts};


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

    pub fn cross(u : Vec3, v : Vec3) -> Vec3 {
        Vec3::new(u.y() * v.z() - u.z() * v.y(),
                u.z() * v.x() - u.x() * v.z(),
                u.x() * v.y() - u.y() * v.x())
    }

    pub fn reflect(v : Vec3, n : Vec3) -> Vec3 {
        v - 2.0*Vec3::dot(v,n)*n
    }

    pub fn refract(uv : Vec3, n : Vec3, etai_over_etat : f64) -> Vec3 {
        let cos_theta = Vec3::dot(-uv, n);
        let r_out_parallel = etai_over_etat * (uv + cos_theta*n);
        let r_out_perp = -(1.0 - r_out_parallel.length_squared()).sqrt() * n;
        r_out_parallel + r_out_perp
    }

    pub fn random<T : Rng>(rng : &mut T) -> Vec3 {
        Vec3::new(rng.gen(),rng.gen(),rng.gen())
    }

    pub fn random_interval<T : Rng>(rng : &mut T, min : f64, max : f64) -> Vec3 {
        Vec3::new(rng.gen_range(min,max),rng.gen_range(min,max),rng.gen_range(min,max))
    }

    pub fn random_in_unit_sphere<T : Rng>(rng : &mut T) -> Vec3 {
        loop {
            let p = Vec3::random_interval(rng, -1.0, 1.0);
            if p.length_squared() > 1.0 { continue };
            return p;
        }
    }

    pub fn random_unit_vector<T : Rng>(rng : &mut T) -> Vec3 {
        let a = rng.gen_range(0.0, 2.0 * consts::PI);
        let z : f64 = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z*z).sqrt();
        return Vec3::new(r * a.cos(), r*a.sin(), z);
    }

    pub fn random_in_hemisphere<T : Rng>(rng : &mut T, normal : &Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_in_unit_sphere(rng);
        // In the same hemisphere as the normal
        if Vec3::dot(in_unit_sphere, *normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
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