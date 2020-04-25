mod color;
mod vec;

use color::Color;
use vec::Vec3;

fn hit_sphere(center : Vec3, radius : f32, r : &Ray) -> bool {
    let oc = r.origin - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc, oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn ray_color(ray: Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0,0.0,-1.0), 0.5, &ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction = ray.direction().to_unit();
    let t = 0.5*(unit_direction.y() + 1.0);
    (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let mut pm = PixMap::default();

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..pm.height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..pm.width {
            let u = i as f32 / pm.width as f32;
            let v = j as f32 / pm.height as f32;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let color = ray_color(r);
            pm.pixels.push(color.into());
        }
    }
    eprint!("\nDone");

    pm.save();
    eprint!("\nSaved");

}

struct PixMap {
    pixels: Vec<Color>,
    width: u16,
    height: u16,
}

impl Default for PixMap {
    fn default() -> Self {
        Self {
            width: 200,
            height: 100,
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
