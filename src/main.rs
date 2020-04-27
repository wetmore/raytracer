extern crate rand;

use rand::Rng;
use std::time::{Instant};

mod color;
mod vec;
mod solids;
mod camera;
mod ray;
mod materials;
mod pixmap;
mod hittable;

use color::Samples;
use vec::Vec3;
use solids::Sphere;
use ray::Ray;
use camera::Camera;
use materials::{MaterialType, Material, HitRecord};
use pixmap::PixMap;
use hittable::HittableList;

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

const SAMPLES_PER_PIXEL : u16 = 400;
const IMAGE_WIDTH : u16 = 2880;
const IMAGE_HEIGHT : u16 = 1800;
const MAX_DEPTH : u16 = 1000;

fn main() {
    let mut rng = rand::thread_rng();
    let mut pm = PixMap::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5), MaterialType::Metal(Vec3::new(0.7, 0.3, 0.3), 0.0));
    world.add(Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0), MaterialType::Metal(Vec3::new(0.8,0.8,0.0), 0.3));

    world.add(Sphere::new(Vec3::new(1.0,0.0,-1.0), 0.5), MaterialType::Metal(Vec3::new(0.8, 0.6, 0.2), 1.0));
    world.add(Sphere::new(Vec3::new(-1.0,0.0,-1.0), 0.5), MaterialType::Dielectric(1.5));
    world.add(Sphere::new(Vec3::new(-1.0,0.0,-1.0), -0.45), MaterialType::Dielectric(1.5));

    //for _ in 0..100 {
     //   world.add(Sphere::new(Vec3::new(
     //       rng.gen_range(-100.0,100.0),
    //        rng.gen_range(-100.0,100.0),
    //        rng.gen_range(-200.0,-20.0)
    //    ), rng.gen_range(1.0,20.0)),
    //MaterialType::Metal(Vec3::new(rng.gen_range(0.0,1.0), rng.gen_range(0.0,1.0), rng.gen_range(0.0,1.0)), rng.gen_range(0.0,1.0)));
    //}

    let vup = Vec3::new(0.0,1.0,0.0);
    let look_at = Vec3::new(0.0,0.0,-1.0);
    let look_from = Vec3::new(-2.0,1.0,-0.2);
    let cam = Camera::new(20.0, pm.aspect(), look_at, look_from, vup);

    //let vup = Vec3::new(0.0,1.0,0.0);
    //let look_at = Vec3::new(0.0,0.0,-1.0);
    //let look_from = Vec3::new(0.0,0.0,0.0);
    //let cam = Camera::new(90.0, pm.aspect(), look_at, look_from, vup);

    let start = Instant::now();

    eprintln!("{}x{} image with {} samples per pixel", IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL);
    for j in (0..pm.height()).rev() {
        eprint!("\rScanlines remaining: {}", j);

        let j = j as f64;
        for i in 0..pm.width() {
            let mut samples = Samples::default();
            let i = i as f64;
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i + rng.gen::<f64>()) / pm.width() as f64;
                let v = (j + rng.gen::<f64>()) / pm.height() as f64;
                let r = cam.get_ray(u, v);
                samples.add_sample(ray_color(&r, &world, &mut rng, MAX_DEPTH));
            }
            pm.push(samples.into());
        }
    }
    eprint!("\nDone");

    pm.save();
    let duration = start.elapsed();

    eprintln!("\nSaved. Took {:?}", duration);
}
