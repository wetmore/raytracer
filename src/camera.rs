use crate::vec::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin : Vec3,
    lower_left_corner : Vec3,
    horizontal : Vec3,
    vertical : Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
        }
    }
}

impl Camera {
    pub fn new(fov : f64, aspect : f64, look_at : Vec3, look_from : Vec3, vup : Vec3) -> Self {
        let theta = fov.to_radians();
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).to_unit();
        let u = Vec3::cross(vup, w).to_unit();
        let v = Vec3::cross(w, u);

        Self {
            origin: look_from,
            lower_left_corner: look_from - half_width*u - half_height*v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }

    pub fn get_ray(&self, u : f64, v : f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}