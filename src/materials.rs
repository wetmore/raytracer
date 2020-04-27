pub enum MaterialType {
    Lambertian(Vec3) // Albedo
}

pub struct Reftion {
    attenuation : Vec3,
    scattered : Ray,
}

trait Material {
    fn scatter<T : Rng>(&self, r_in : &Ray, rec : &HitRecord, rng : &mut T) -> Option<Reftion>;
}

impl Material for MaterialType {
    fn scatter<T : Rng>(&self, r_in : &Ray, rec : &HitRecord, rng : &mut T) -> Option<Reftion> {
        match self {
            MaterialType::Lambertian(albedo) => {
                let scatter_direction = rec.normal + random_unit_vector(rng);
                Some(Reftion {
                    attenuation: *albedo,
                    scattered: Ray::new(rec.p, scatter_direction)
                })
            }
        }
    }
}