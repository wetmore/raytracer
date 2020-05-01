#![allow(warnings)]

extern crate indicatif;
extern crate rand;
extern crate rayon;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle};
use rand::{thread_rng, Rng};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::*;
use std::time::Instant;

mod aabb;
mod camera;
mod color;
mod hittable;
mod materials;
mod pixmap;
mod ray;
mod scenes;
mod solids;
mod texture;
mod vec;

use camera::{Camera, CameraOptions};
use color::{Color, Samples};
use hittable::HittableList;
use materials::{Material, MaterialType};
use pixmap::PixMap;
use ray::Ray;
use scenes::{use_scene, Scene};
use solids::Sphere;
use texture::TextureType;
use vec::Vec3;

fn ray_color<T: Rng>(ray: &Ray, world: &HittableList, rng: &mut T, depth: u16) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001, f64::INFINITY) {
        None => {
            // Show gradient for background
            let unit_direction = ray.direction().to_unit();
            let t = 0.5 * (unit_direction.y() + 1.0);
            let lerped = (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
            return lerped;
        }
        Some(rec) => match rec.mat.scatter(ray, &rec, rng) {
            None => Vec3::new(0.0, 0.0, 0.0),
            Some((attentuation, scattered)) => {
                attentuation * ray_color(&scattered, world, rng, depth - 1)
            }
        },
    }
}

//took 788 seconds
//const SAMPLES_PER_PIXEL : u16 = 512;
//const IMAGE_WIDTH : u16 = 1280;
//const IMAGE_HEIGHT : u16 = 960;
//const MAX_DEPTH : u16 = 100;

const SAMPLES_PER_PIXEL: u16 = 5;
const IMAGE_WIDTH: u16 = 800;
const IMAGE_HEIGHT: u16 = 600;
const MAX_DEPTH: u16 = 1000;

#[derive(Clone, Copy)]
struct Pixel {
    x: u16,
    y: u16,
}

fn main() {
    let mut pm = PixMap::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut pixels = Vec::new();

    let world = use_scene(Scene::SHINY);
    let cam = Camera::new(CameraOptions::cool2(&pm));

    let start = Instant::now();

    eprintln!(
        "{}x{} image with {} samples per pixel",
        IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL
    );
    for j in (0..pm.height()).rev() {
        for i in 0..pm.width() {
            pixels.push(Pixel { x: i, y: j });
        }
    }

    let pb = ProgressBar::new(pixels.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {wide_bar} {pos:>7}/{len:7} {msg} [{eta_precise}]"),
    );

    let colors: Vec<Color> = pixels
        .into_par_iter()
        .progress_with(pb)
        .map(|p| {
            let mut rng = thread_rng();
            let color = raytrace_pixel(p as Pixel, &pm, cam, &world, &mut rng);
            return color;
        })
        .collect();

    for (i, color) in colors.iter().enumerate() {
        pm.push(*color);
    }
    let duration = start.elapsed();

    eprint!("\nDone. Took {:?}", duration);

    pm.save();

    eprintln!("\nSaved");
}

fn raytrace_pixel<T: Rng>(
    p: Pixel,
    pm: &PixMap,
    camera: Camera,
    world: &HittableList,
    rng: &mut T,
) -> Color {
    let mut samples = Samples::default();
    let i = p.x as f64;
    let j = p.y as f64;
    // TODO: Parallelize sampling as well
    for _ in 0..SAMPLES_PER_PIXEL {
        let u = (i + rng.gen::<f64>()) / pm.width() as f64;
        let v = (j + rng.gen::<f64>()) / pm.height() as f64;
        let r = camera.get_ray(u, v, rng);
        samples.add_sample(ray_color(&r, world, rng, MAX_DEPTH));
    }
    return samples.into();
}
