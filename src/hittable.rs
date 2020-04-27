use crate::solids::Sphere;
use crate::materials::{MaterialType, HitRecord};
use crate::ray::Ray;
use crate::vec::Vec3;

pub trait Hittable {
    fn hit(&self, mat : MaterialType, ray : &Ray, t_min : f64, t_max : f64) -> Option<HitRecord>;
}

impl Hittable for Sphere {
    fn hit(&self, mat : MaterialType, ray : &Ray, t_min : f64, t_max : f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center();
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius()*self.radius();
        let discriminant = half_b*half_b - a*c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t_hit = (-half_b - root) / a;
            if t_hit < t_max && t_hit > t_min {
                let p = ray.at(t_hit);
                let outward_normal = (p - self.center()) / self.radius();
                return Some(HitRecord::new(t_hit, ray.at(t_hit), outward_normal, mat, ray))
            }
            let t_hit = (-half_b + root) / a;
            if t_hit < t_max && t_hit > t_min {
                let p = ray.at(t_hit);
                let outward_normal = (p - self.center()) / self.radius();
                return Some(HitRecord::new(t_hit, ray.at(t_hit), outward_normal, mat, ray))
            }
            return None;
        } else {
            return None;
        }
    }
}

pub struct HittableList {
    // TODO: change this to use Hittables not Spheres
    objects : Vec<(Box<Sphere>, MaterialType)>
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

    pub fn add(&mut self, object : Sphere, mat : MaterialType) {
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
}