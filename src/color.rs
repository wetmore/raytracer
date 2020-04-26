use crate::vec::Vec3;

pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r : u8, g : u8, b : u8) -> Self {
        Color(r,g,b)
    }

    fn r(&self) -> u8 {
        self.0
    }

    fn g(&self) -> u8 {
        self.1
    }

    fn b(&self) -> u8 {
        self.2
    }
}

pub struct Samples {
    vector : Vec3,
    num_samples : u16,
}

impl Default for Samples {
    fn default() -> Self {
        Samples {
            vector: Vec3::new(0.0,0.0,0.0),
            num_samples: 0,
        }
    }
}

impl Samples {
    pub fn add_sample(&mut self, sample : Vec3) {
        self.vector = self.vector + sample; // TODO make AddAssign impl?
        self.num_samples += 1;
    }
}

fn clamp(x : f64, min : f64, max : f64) -> f64 {
    if x < min { min } else if x > max { max } else { x }
}

impl Into<Color> for Samples {
    fn into(self) -> Color {
        let scale = 1.0 / self.num_samples as f64;
        let r = (scale * self.vector.x()).sqrt();
        let g = (scale * self.vector.y()).sqrt();
        let b = (scale * self.vector.z()).sqrt();

        Color(
            (256.0 * clamp(r, 0.0, 0.999)) as u8,
            (256.0 * clamp(g, 0.0, 0.999)) as u8,
            (256.0 * clamp(b, 0.0, 0.999)) as u8
        )
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}