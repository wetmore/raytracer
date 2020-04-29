extern crate rand;
extern crate rayon;
extern crate indicatif;

use rayon::prelude::*;
use indicatif::{ProgressIterator,ParallelProgressIterator,ProgressBar, ProgressStyle};
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
use rand::{thread_rng, Rng};
use std::time::{Instant};

mod color;
mod vec;
mod solids;
mod camera;
mod ray;
mod materials;
mod pixmap;
mod hittable;
mod texture;
mod scenes;

use color::{Color, Samples};
use vec::Vec3;
use solids::Sphere;
use ray::Ray;
use camera::{Camera, CameraOptions};
use materials::{MaterialType, Material};
use pixmap::PixMap;
use hittable::HittableList;
use texture::TextureType;
use scenes::{Scene, use_scene};

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

//took 788 seconds
//const SAMPLES_PER_PIXEL : u16 = 512;
//const IMAGE_WIDTH : u16 = 1280;
//const IMAGE_HEIGHT : u16 = 960;
//const MAX_DEPTH : u16 = 100;


const SAMPLES_PER_PIXEL : u16 = 500;
const IMAGE_WIDTH : u16 = 800;
const IMAGE_HEIGHT : u16 = 600;
const MAX_DEPTH : u16 = 100;

#[derive(Clone ,Copy)]
struct Pixel {
    x: u16,
    y: u16,
}

fn main() {
    let mut pm = PixMap::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut pixels = Vec::new();

    let world = use_scene(Scene::THREE_BALLS);
    let cam = Camera::new(CameraOptions::cool1(&pm));

    let start = Instant::now();

    eprintln!("{}x{} image with {} samples per pixel", IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL);
    for j in (0..pm.height()).rev() {
        for i in 0..pm.width() {
            pixels.push(Pixel {x:i, y:j});
        }
    }

    let pb = ProgressBar::new(pixels.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "[{elapsed_precise}] {wide_bar} {pos:>7}/{len:7} {msg} [{eta_precise}]"
            ),
    );


    let colors: Vec<Color> = pixels.par_iter().progress_with(pb).map(|p| {
        let mut rng = thread_rng();
        let color = raytrace_pixel(*p, &pm, cam, &world, &mut rng);
        return color;
    }).collect();


    for (i, color) in colors.iter().enumerate() {
        pm.push(*color);
    }

    eprint!("\nDone");

    pm.save();
    let duration = start.elapsed();

    eprintln!("\nSaved. Took {:?}", duration);
}

fn raytrace_pixel<T : Rng>(p : Pixel, pm: &PixMap, camera : Camera, world: &HittableList, rng : &mut T) -> Color {
    let mut samples = Samples::default();
    let i = p.x as f64;
    let j = p.y as f64;
    for _ in 0..SAMPLES_PER_PIXEL {
        let u = (i + rng.gen::<f64>()) / pm.width() as f64;
        let v = (j + rng.gen::<f64>()) / pm.height() as f64;
        let r = camera.get_ray(u, v, rng);
        samples.add_sample(ray_color(&r, world, rng, MAX_DEPTH));
    }
    return samples.into();
}
