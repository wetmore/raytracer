mod color;
mod vec;

use color::Color;
use vec::Vec3;

fn hit_sphere(center : Vec3, radius : f32, r : &Ray) -> f32 {
    let oc = r.origin - center;
    let a = r.direction().length_squared();
    let half_b = Vec3::dot(oc, r.direction());
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(ray: Ray) -> Color {
    let sphere_center = Vec3::new(0.0,0.0,-1.0);
    let t = hit_sphere(sphere_center, 0.5, &ray);
    if t > 0.0 {
        let n = Vec3::to_unit(&(ray.at(t) - sphere_center));
        return (0.5 * Vec3::new(n.x()+1.0, n.y()+1.0, n.z()+1.0)).into();
    }
    let unit_direction = ray.direction().to_unit();
    let t = 0.5*(unit_direction.y() + 1.0);
    let lerped = (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    lerped.into()
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
            width: 512,
            height: 256,
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
