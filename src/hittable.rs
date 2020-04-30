use crate::solids::Sphere;
use crate::materials::{MaterialType, HitRecord};
use crate::ray::Ray;
use crate::{aabb::AABB, vec::Vec3};
use std::f64::{consts};


pub enum HittableType {
    Sphere(Sphere),
    BVHNode(BVHNode),
}

impl HittableType {
    pub fn sphere(center : Vec3, radius : f64) -> Self {
        HittableType::Sphere(Sphere::new(center, radius))
    }
}
pub trait Hittable {
    fn hit(&self, mat : MaterialType, ray : &Ray, t_min : f64, t_max : f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0 : f64, t1 : f64) -> Option<AABB>; 
}

impl Hittable for HittableType {
    fn hit(&self, mat : MaterialType, ray : &Ray, t_min : f64, t_max : f64) -> Option<HitRecord> {
        match *self {
            HittableType::Sphere(s) => { s.hit(mat, ray, t_min, t_max) }
            HittableType::BVHNode(_) => {
                None
            }
        }
    }

    fn bounding_box(&self, t0 : f64, t1 : f64) -> Option<AABB> {
        match *self {
            HittableType::Sphere(s) => {
                let r = s.radius();
                let r_vec = Vec3::new(r,r,r);
                Some(AABB::new(s.center() - r_vec, s.center() - r_vec))
            }
            HittableType::BVHNode(_) => {
                None
            }
        }
    }
}

pub struct HittableList {
    objects : Vec<(Box<HittableType>, MaterialType)>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object : HittableType, mat : MaterialType) {
        self.objects.push((Box::new(object), mat))
    }

    pub fn hit(&self, ray : &Ray, t_min : f64, t_max : f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec = None;

        for (object, mat) in &self.objects {
            match object.hit(*mat, ray, t_min, closest_so_far) {
                None => (),
                Some(temp_rec) => {
                    closest_so_far = temp_rec.t;
                    rec = Some(temp_rec);
                }
            }
        }

        return rec;
    }

    pub fn bounding_box(&self, t0 : f64, t1 : f64) -> Option<AABB> {
        if self.objects.len() == 0 {
            return None;
        }

        let bbs : Vec<AABB>= self.objects.iter().flat_map(|(hittable,_)| {
            match hittable.bounding_box(t0,t1) {
                None => { vec![] }
                Some(bb) => { vec![bb]}
            }
        }).collect();

        if bbs.len() < self.objects.len() {
            return None;
        }

        let mut output = bbs[0];
        for bb in bbs {
            output = AABB::surrounding_box(output, bb)
        }
        return Some(output);
    }
}

struct BVHNode {
    left : Box<HittableType>,
    right : Box<HittableType>,
}