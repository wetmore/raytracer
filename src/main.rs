extern crate rand;

use rand::Rng;
use std::time::{Duration, Instant};

mod color;
mod vec;
mod solids;

use color::Color;
use color::Samples;
use vec::Vec3;
use solids::Sphere;


struct HitRecord {
    t : f32,
    p : Vec3,
    normal : Vec3,
    front_face : bool,
}

// This shouldn't be necessary, considering making hit fn return
// Option<HitRecord>
impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            t : 0.0,
            p : Vec3::new(0.0,0.0,0.0),
            normal : Vec3::new(0.0,0.0,0.0),
            front_face : true
        }
    }
}

impl HitRecord {
    fn set_face_normal(&mut self, ray : &Ray, outward_normal : Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

trait Hittable {
    fn hit(&self, ray : &Ray, t_min : f32, t_max : f32, rec : &mut HitRecord) -> bool;
}

impl Hittable for Sphere {
    fn hit(&self, ray : &Ray, t_min : f32, t_max : f32, rec : &mut HitRecord) -> bool {
        let oc = ray.origin - self.center();
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius()*self.radius();
        let discriminant = half_b*half_b - a*c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let t_hit = (-half_b - root) / a;
            if t_hit < t_max && t_hit > t_min {
                rec.t = t_hit;
                rec.p = ray.at(t_hit);
                let outward_normal = (rec.p - self.center()) / self.radius();
                rec.set_face_normal(ray, outward_normal);
                return true;
            }
            let t_hit = (-half_b + root) / a;
            if t_hit < t_max && t_hit > t_min {
                rec.t = t_hit;
                rec.p = ray.at(rec.t);
                let outward_normal = (rec.p - self.center()) / self.radius();
                rec.set_face_normal(ray, outward_normal);
                return true;
            }
            return false;
        } else {
            return false;
        }
    }
}

struct HittableList {
    objects : Vec<Box<Sphere>>
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

    pub fn add(&mut self, object : Sphere) {
        self.objects.push(Box::new(object))
    }

    pub fn hit(&self, ray : &Ray, t_min : f32, t_max : f32, rec : &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.t = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.front_face = temp_rec.front_face;
            }
        }

        return hit_anything;
    }
}

struct Camera {
    origin : Vec3,
    lower_left_corner : Vec3,
    horizontal : Vec3,
    vertical : Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
        }
    }
}

impl Camera {
    fn get_ray(&self, u : f32, v : f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}

fn ray_color(ray: &Ray, world : &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(ray, 0.0, f32::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vec3::new(1.0,1.0,1.0));
    }

    // Show gradient for background
    let unit_direction = ray.direction().to_unit();
    let t = 0.5*(unit_direction.y() + 1.0);
    let lerped = (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    return lerped;
}

const SAMPLES_PER_PIXEL : u16 = 100;
const IMAGE_WIDTH : u16 = 512;
const IMAGE_HEIGHT : u16 = 256;

fn main() {
    let mut rng = rand::thread_rng();
    let mut pm = PixMap::default();

    let mut world = HittableList::new();
    //world.add(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5));
   //world.add(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0));

    for _ in 0..100 {
        world.add(Sphere::new(Vec3::new(
            rng.gen_range(-100.0,100.0),
            rng.gen_range(-100.0,100.0),
            rng.gen_range(-200.0,-20.0)
        ), rng.gen_range(1.0,20.0)));
    }

    let cam = Camera::default();

    let start = Instant::now();

    for j in (0..pm.height).rev() {
        eprint!("\rScanlines remaining: {}", j);

        let j = j as f32;
        for i in 0..pm.width {
            let mut samples = Samples::default();
            let i = i as f32;
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i + rng.gen::<f32>()) / pm.width as f32;
                let v = (j + rng.gen::<f32>()) / pm.height as f32;
                let r = cam.get_ray(u, v);
                samples.add_sample(ray_color(&r, &world));
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

struct Ray {
    origin: Vec3,
    vec: Vec3,
}

impl Ray {
    pub fn new(origin : Vec3, dir : Vec3) -> Ray {
        Ray {
            origin: origin,
            vec: dir
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.vec
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.vec * t
    }
}
