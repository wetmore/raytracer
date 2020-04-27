
use crate::vec::Vec3;
use crate::ray::Ray;
use crate::rand::Rng;

#[derive(Clone, Copy)]
pub enum MaterialType {
    Lambertian(Vec3), // Albedo
    Metal(Vec3, f64), // Albedo, Fuzz radius
}

pub trait Material {
    fn scatter<T : Rng>(&self, r_in : &Ray, rec : &HitRecord, rng : &mut T) -> Option<(Vec3, Ray)>;
}

impl Material for MaterialType {
    fn scatter<T : Rng>(&self, r_in : &Ray, rec : &HitRecord, rng : &mut T) -> Option<(Vec3, Ray)> {
        match self {
            MaterialType::Lambertian(albedo) => {
                let scatter_direction = rec.normal + Vec3::random_unit_vector(rng);
                Some((*albedo,Ray::new(rec.p, scatter_direction)))
            },
            MaterialType::Metal(albedo, fuzz) => {
                let reflected = Vec3::reflect(Vec3::to_unit(&r_in.direction()), rec.normal);
                let scattered = Ray::new(rec.p, reflected + *fuzz*Vec3::random_in_unit_sphere(rng));
                if Vec3::dot(scattered.direction(), rec.normal) > 0.0 {
                    Some((*albedo, scattered))
                } else {
                    None
                }
            }
        }
    }
}

pub struct HitRecord {
    pub t : f64,
    pub p : Vec3,
    pub normal : Vec3,
    pub front_face : bool,
    pub mat : MaterialType,
}

impl HitRecord {
    pub fn new(t : f64, p : Vec3, outward_normal : Vec3, mat : MaterialType, ray : &Ray) -> Self {
        let front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            front_face: front_face,
            mat: mat,
        }
    }
}