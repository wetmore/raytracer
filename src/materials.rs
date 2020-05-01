use crate::rand::Rng;
use crate::ray::Ray;
use crate::texture::{Texture, TextureType};
use crate::vec::Vec3;

#[derive(Clone, Copy)]
pub enum MaterialType {
    Lambertian(TextureType), // Albedo
    Metal(Vec3, f64),        // Albedo, Fuzz radius
    Dielectric(f64),         // Refractive Index
}

pub trait Material {
    fn scatter<T: Rng>(&self, r_in: &Ray, rec: &HitRecord, rng: &mut T) -> Option<(Vec3, Ray)>;
}

impl Material for MaterialType {
    fn scatter<T: Rng>(&self, r_in: &Ray, rec: &HitRecord, rng: &mut T) -> Option<(Vec3, Ray)> {
        match self {
            MaterialType::Lambertian(albedo) => {
                let scatter_direction = rec.normal + Vec3::random_unit_vector(rng);
                let attenuation = albedo.value(rec.u, rec.v, &rec.p);
                Some((attenuation, Ray::new(rec.p, scatter_direction)))
            }
            MaterialType::Metal(albedo, fuzz) => {
                let reflected = Vec3::reflect(Vec3::to_unit(&r_in.direction()), rec.normal);
                let scattered =
                    Ray::new(rec.p, reflected + *fuzz * Vec3::random_in_unit_sphere(rng));
                if Vec3::dot(scattered.direction(), rec.normal) > 0.0 {
                    Some((*albedo, scattered))
                } else {
                    None
                }
            }
            MaterialType::Dielectric(ref_idx) => {
                let etai_over_etat = if rec.front_face {
                    1.0 / ref_idx
                } else {
                    *ref_idx
                };
                let unit_direction = r_in.direction().to_unit();
                let cos_theta = f64::min(Vec3::dot(-unit_direction, rec.normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let new_direction = if etai_over_etat * sin_theta > 1.0 {
                    Vec3::reflect(unit_direction, rec.normal)
                } else {
                    let reflect_prob = schlick(cos_theta, etai_over_etat);
                    if rng.gen::<f64>() < reflect_prob {
                        Vec3::reflect(unit_direction, rec.normal)
                    } else {
                        Vec3::refract(unit_direction, rec.normal, etai_over_etat)
                    }
                };
                Some((Vec3::new(1.0, 1.0, 1.0), Ray::new(rec.p, new_direction)))
            }
        }
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * ((1.0 - cosine).powf(5.0));
}

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub mat: MaterialType,
    u: f64,
    v: f64,
}

impl HitRecord {
    pub fn new(
        t: f64,
        p: Vec3,
        outward_normal: Vec3,
        mat: MaterialType,
        ray: &Ray,
        u: f64,
        v: f64,
    ) -> Self {
        let front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            t,
            p,
            normal,
            front_face,
            mat,
            u,
            v,
        }
    }
}
