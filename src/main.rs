extern crate rand;

use rand::Rng;
use std::time::{Instant};
use std::f64::{consts};

mod color;
mod vec;
mod solids;
mod camera;
mod ray;

use color::Color;
use color::Samples;
use vec::Vec3;
use solids::Sphere;
use ray::Ray;
use camera::Camera;


struct HitRecord {
    t : f64,
    p : Vec3,
    normal : Vec3,
    front_face : bool,
    mat : MaterialType,
}

impl HitRecord {
    fn new(t : f64, p : Vec3, outward_normal : Vec3, mat : MaterialType, ray : &Ray) -> Self {
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

#[derive(Clone, Copy)]
enum MaterialType {
    Lambertian(Vec3), // Albedo
    Metal(Vec3, f64), // Albedo, Fuzz radius
}

trait Material {
    fn scatter<T : Rng>(&self, r_in : &Ray, rec : &HitRecord, rng : &mut T) -> Option<(Vec3, Ray)>;
}

impl Material for MaterialType {
    fn scatter<T : Rng>(&self, r_in : &Ray, rec : &HitRecord, rng : &mut T) -> Option<(Vec3, Ray)> {
        match self {
            MaterialType::Lambertian(albedo) => {
                let scatter_direction = rec.normal + random_unit_vector(rng);
                Some((*albedo,Ray::new(rec.p, scatter_direction)))
            },
            MaterialType::Metal(albedo, fuzz) => {
                let reflected = Vec3::reflect(Vec3::to_unit(&r_in.direction()), rec.normal);
                let scattered = Ray::new(rec.p, reflected + *fuzz*random_in_unit_sphere(rng));
                if Vec3::dot(scattered.direction(), rec.normal) > 0.0 {
                    Some((*albedo, scattered))
                } else {
                    None
                }
            }
        }
    }
}

trait Hittable {
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

struct HittableList {
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

fn random_in_unit_sphere<T : Rng>(rng : &mut T) -> Vec3 {
    loop {
        let p = Vec3::random_interval(rng, -1.0, 1.0);
        if p.length_squared() > 1.0 { continue };
        return p;
    }
}

fn random_unit_vector<T : Rng>(rng : &mut T) -> Vec3 {
    let a = rng.gen_range(0.0, 2.0 * consts::PI);
    let z : f64 = rng.gen_range(-1.0, 1.0);
    let r = (1.0 - z*z).sqrt();
    return Vec3::new(r * a.cos(), r*a.sin(), z);
}

fn random_in_hemisphere<T : Rng>(rng : &mut T, normal : &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere(rng);
    // In the same hemisphere as the normal
    if Vec3::dot(in_unit_sphere, *normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

fn ray_color<T : Rng>(ray: &Ray, world : &HittableList, rng : &mut T, depth : u16) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0,0.0,0.0);
    }

    match world.hit(ray, 0.001, f64::INFINITY) {
        None => {
            // Show gradient for background
            let unit_direction = ray.direction().to_unit();
            let t = 0.5*(unit_direction.y() + 1.0);
            let lerped = (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
            return lerped;
        },
        Some(rec) => {
            match rec.mat.scatter(ray, &rec, rng) {
                None => Vec3::new(0.0,0.0,0.0),
                Some((attentuation, scattered)) => {
                    attentuation * ray_color(&scattered, world, rng, depth-1)
                }
            }
        }
    }
}

const SAMPLES_PER_PIXEL : u16 = 256;
const IMAGE_WIDTH : u16 = 800;
const IMAGE_HEIGHT : u16 = 400;
const MAX_DEPTH : u16 = 256;

fn main() {
    let mut rng = rand::thread_rng();
    let mut pm = PixMap::default();

    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0,0.0,-2.0), 0.5), MaterialType::Metal(Vec3::new(0.7, 0.3, 0.3), 0.0));
    world.add(Sphere::new(Vec3::new(0.0,-100.5,-2.0), 100.0), MaterialType::Metal(Vec3::new(0.8,0.8,0.0), 0.3));

    world.add(Sphere::new(Vec3::new(1.0,0.0,-2.0), 0.5), MaterialType::Metal(Vec3::new(0.8, 0.6, 0.2), 1.0));
    world.add(Sphere::new(Vec3::new(-1.0,0.0,-2.0), 0.5), MaterialType::Metal(Vec3::new(0.8,0.8,0.8), 1.0));

    //for _ in 0..100 {
    //    world.add(Sphere::new(Vec3::new(
    //        rng.gen_range(-100.0,100.0),
    //        rng.gen_range(-100.0,100.0),
    //        rng.gen_range(-200.0,-20.0)
    //    ), rng.gen_range(1.0,20.0)));
    //}

    let cam = Camera::default();

    let start = Instant::now();

    eprintln!("{}x{} image with {} samples per pixel", IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL);
    for j in (0..pm.height).rev() {
        eprint!("\rScanlines remaining: {}", j);

        let j = j as f64;
        for i in 0..pm.width {
            let mut samples = Samples::default();
            let i = i as f64;
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i + rng.gen::<f64>()) / pm.width as f64;
                let v = (j + rng.gen::<f64>()) / pm.height as f64;
                let r = cam.get_ray(u, v);
                samples.add_sample(ray_color(&r, &world, &mut rng, MAX_DEPTH));
            }
            pm.pixels.push(samples.into());
        }
    }
    eprint!("\nDone");

    pm.save();
    let duration = start.elapsed();

    eprintln!("\nSaved. Took {:?}", duration);
}

struct PixMap {
    pixels: Vec<Color>,
    width: u16,
    height: u16,
}

impl Default for PixMap {
    fn default() -> Self {
        Self {
            width: IMAGE_WIDTH,
            height: IMAGE_HEIGHT,
            pixels: Vec::new(),
        }
    }
}

impl PixMap {
    fn save(&self) {
        println!("P3\n{} {}\n255", self.width, self.height);

        for color in &self.pixels {
            println!("{}", color);
        }
    }
}
