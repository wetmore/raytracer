use crate::{ray::Ray, vec::Vec3};

#[derive(Clone, Copy)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let t0 = f64::min(
                (self.min.comp(a) - r.origin().comp(a)) / r.direction().comp(a),
                (self.max.comp(a) - r.origin().comp(a)) / r.direction().comp(a),
            );
            let t1 = f64::max(
                (self.min.comp(a) - r.origin().comp(a)) / r.direction().comp(a),
                (self.max.comp(a) - r.origin().comp(a)) / r.direction().comp(a),
            );
            let tmin = f64::max(t0, t_min);
            let tmax = f64::min(t1, t_max);
            if tmax <= tmin {
                return false;
            }
        }
        return true;
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Vec3::new(
            f64::min(box0.min().x(), box1.min().x()),
            f64::min(box0.min().y(), box1.min().y()),
            f64::min(box0.min().z(), box1.min().z()),
        );
        let big = Vec3::new(
            f64::max(box0.max().x(), box1.max().x()),
            f64::max(box0.max().y(), box1.max().y()),
            f64::max(box0.max().z(), box1.max().z()),
        );
        return AABB::new(small, big);
    }
}
