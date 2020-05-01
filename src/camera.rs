use crate::ray::Ray;
use crate::vec::Vec3;
use crate::{pixmap::PixMap, rand::Rng};

#[derive(Clone, Copy)]
pub struct CameraOptions {
    fov: f64,
    aspect: f64,
    aperature: f64,
    focus_dist: f64,
    look_at: Vec3,
    look_from: Vec3,
    vup: Vec3,
}

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(opts: CameraOptions) -> Self {
        let theta = opts.fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = opts.aspect * half_height;

        let w = (opts.look_from - opts.look_at).to_unit();
        let u = Vec3::cross(opts.vup, w).to_unit();
        let v = Vec3::cross(w, u);

        Self {
            origin: opts.look_from,
            lower_left_corner: opts.look_from
                - half_width * opts.focus_dist * u
                - half_height * opts.focus_dist * v
                - opts.focus_dist * w,
            horizontal: 2.0 * half_width * opts.focus_dist * u,
            vertical: 2.0 * half_height * opts.focus_dist * v,
            u: u,
            v: v,
            w: w,
            lens_radius: opts.aperature / 2.0,
        }
    }

    pub fn get_ray<T: Rng>(&self, s: f64, t: f64, rng: &mut T) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

impl CameraOptions {
    pub fn default(pm: &PixMap) -> CameraOptions {
        let look_at = Vec3::new(0.0, 0.0, -1.0);
        let look_from = Vec3::new(0.0, 0.0, 0.0);

        CameraOptions {
            fov: 90.0,
            aspect: pm.aspect(),
            aperature: 0.0,
            focus_dist: (look_from - look_at).length(),
            look_at: look_at,
            look_from: look_from,
            vup: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn cool1(pm: &PixMap) -> CameraOptions {
        let look_at = Vec3::new(0.5, -0.5, -1.0);
        let look_from = Vec3::new(0.0, 1.0, 0.0);

        CameraOptions {
            fov: 30.0,
            aspect: pm.aspect(),
            aperature: 0.0,
            focus_dist: (look_from - look_at).length(),
            look_at: look_at,
            look_from: look_from,
            vup: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn cool2(pm: &PixMap) -> CameraOptions {
        let look_at = Vec3::new(0.0, 0.0, -1.0);
        let look_from = Vec3::new(-2.0, 1.0, -0.2);

        CameraOptions {
            fov: 25.0,
            aspect: pm.aspect(),
            aperature: 0.0,
            focus_dist: (look_from - look_at).length(),
            look_at: look_at,
            look_from: look_from,
            vup: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}
