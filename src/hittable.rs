use crate::materials::{HitRecord, MaterialType};
use crate::ray::Ray;
use crate::solids::Sphere;
use crate::{aabb::AABB, vec::Vec3};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::{cmp::Ordering, f64::consts, sync::Arc};

#[derive(Clone, Copy)]
pub enum HittableType {
    Sphere(Sphere),
}

impl HittableType {
    pub fn sphere(center: Vec3, radius: f64) -> Self {
        HittableType::Sphere(Sphere::new(center, radius))
    }

    pub fn compare<'a, 'b>(a: &'a HittableType, b: &'b HittableType, axis: u8) -> Ordering {
        let a_bb = a
            .bounding_box(0.0, 0.0)
            .expect("No bounding box in BHVNode constructor");
        let b_bb = b
            .bounding_box(0.0, 0.0)
            .expect("No bounding box in BHVNode constructor");

        a_bb.min()
            .comp(axis)
            .partial_cmp(&b_bb.min().comp(axis))
            .unwrap()
    }
}
pub trait Hittable {
    fn hit(&self, mat: MaterialType, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

impl Hittable for HittableType {
    fn hit(&self, mat: MaterialType, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            HittableType::Sphere(s) => s.hit(mat, ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        match *self {
            HittableType::Sphere(s) => {
                let r = s.radius();
                let r_vec = Vec3::new(r, r, r);
                Some(AABB::new(s.center() - r_vec, s.center() - r_vec))
            }
        }
    }
}

pub struct HittableList {
    objects: Vec<(Box<HittableType>, MaterialType)>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: HittableType, mat: MaterialType) {
        self.objects.push((Box::new(object), mat))
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

    pub fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.objects.len() == 0 {
            return None;
        }

        let bbs: Vec<AABB> = self
            .objects
            .iter()
            .flat_map(|(hittable, _)| match hittable.bounding_box(t0, t1) {
                None => vec![],
                Some(bb) => vec![bb],
            })
            .collect();

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

enum BVHNode {
    Branch(Box<BVHNode>, Box<BVHNode>, AABB),
    Leaf(HittableType),
}

impl BVHNode {
    pub fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        match *self {
            BVHNode::Branch(_, _, bb) => Some(bb),
            BVHNode::Leaf(hittable) => hittable.bounding_box(t0, t1),
        }
    }

    pub fn hit(&self, mat: MaterialType, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            BVHNode::Branch(left, right, bb) => {
                if !bb.hit(*ray, t_min, t_max) {
                    return None;
                }

                match left.hit(mat, ray, t_min, t_max) {
                    None => right.hit(mat, ray, t_min, t_max),
                    Some(left_rec) => {
                        match right.hit(mat, ray, t_min, left_rec.t) {
                            None => Some(left_rec),
                            // Choose the right_rec, because it's closer
                            Some(right_rec) => Some(right_rec),
                        }
                    }
                }
            }
            BVHNode::Leaf(hittable) => hittable.hit(mat, ray, t_min, t_max),
        }
    }

    fn new<T: Rng, D: Distribution<u8>>(
        rng: &mut T,
        dist: &D,
        list: &mut [HittableType],
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = dist.sample(rng);
        //let mut comparator = |a,b| HittableType::compare(a, b, axis);

        let length = list.len();
        if length < 1 {
            panic!("Called BVHNode::new on empty slice")
        } else if length == 1 {
            return BVHNode::Leaf(list[0]);
        } else {
            &list.sort_unstable_by(|a, b| HittableType::compare(a, b, axis));
            let midpoint = length / 2;
            let (list_left, list_right) = list.split_at_mut(midpoint);
            let left = Self::new(rng, dist, list_left, time0, time1);
            let right = Self::new(rng, dist, list_right, time0, time1);

            let l_bb = left
                .bounding_box(time0, time1)
                .expect("No bounding box in BHVNode constructor");
            let r_bb = right
                .bounding_box(time0, time1)
                .expect("No bounding box in BHVNode constructor");

            BVHNode::Branch(
                Box::new(left),
                Box::new(right),
                AABB::surrounding_box(l_bb, r_bb),
            )
        }
    }

    pub fn make(
        list: &mut Vec<HittableType>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let dist = Uniform::new_inclusive(0, 2);
        let mut rng = rand::thread_rng();

        Self::new(&mut rng, &dist, &mut list.as_mut_slice(), time0, time1)
    }
}
