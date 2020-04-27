use crate::vec::Vec3;
use crate::ray::Ray;
use crate::rand::Rng;

pub struct Camera {
    origin : Vec3,
    lower_left_corner : Vec3,
    horizontal : Vec3,
    vertical : Vec3,
    u : Vec3,
    v : Vec3,
    w : Vec3,
    lens_radius : f64,
}

impl Camera {
    pub fn new(fov : f64, aspect : f64,
               aperature : f64, focus_dist : f64,
               look_at : Vec3, look_from : Vec3, vup : Vec3) -> Self {        
        let theta = fov.to_radians();
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).to_unit();
        let u = Vec3::cross(vup, w).to_unit();
        let v = Vec3::cross(w, u);

        Self {
            origin: look_from,
            lower_left_corner: look_from - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            u: u,
            v: v,
            w: w,
            lens_radius: aperature / 2.0, 
        }
    }

    pub fn get_ray<T : Rng>(&self, s : f64, t : f64, rng : &mut T) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset)
    }
}