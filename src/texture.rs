use crate::vec::Vec3;

#[derive(Clone, Copy)]
pub enum TextureType {
    Constant(Vec3),
    JankChecker(f64, Vec3, Vec3), // size, odd, even
    Checker(f64, Vec3, Vec3), // size, odd, even
}

pub trait Texture {
    fn value(&self, u : f64, v : f64, p : &Vec3) -> Vec3 {
        Vec3::new(0.0,0.0,0.0)
    }
}

impl Texture for TextureType {
    fn value(&self, u : f64, v : f64, p : &Vec3) -> Vec3 {
        match *self {
            TextureType::Constant(color) => {
                color
            },
            
            TextureType::JankChecker(size, odd, even) => {
                let sines = (size * p.x()).sin() * (size * p.y()).sin() * (size * p.z()).sin();
                if sines < 0.0 { odd } else { even }
            },

            TextureType::Checker(size, odd, even) => {
                let sines = (u / size).sin() * (v / size).sin();
                if sines < 0.0 { odd } else { even }
            }
        }
    }

}