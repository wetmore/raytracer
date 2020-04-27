use crate::color::Color;

pub struct PixMap {
    pixels: Vec<Color>,
    width: u16,
    height: u16,
}

impl PixMap {
    pub fn new(w : u16, h : u16) -> Self {
        Self {
            width: w,
            height: h,
            pixels: Vec::new(),
        }
    }

    pub fn save(&self) {
        println!("P3\n{} {}\n255", self.width, self.height);

        for color in &self.pixels {
            println!("{}", color);
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn aspect(&self) -> f64 {
        self.width() as f64 / self.height() as f64
    }

    pub fn push(&mut self, pixel : Color){
        self.pixels.push(pixel)
    }
}