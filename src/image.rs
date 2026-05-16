use crate::vector3::Vector3;

pub struct Imagef32 {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vector3>,
}

impl Imagef32 {
    pub fn new(width: u32, height: u32) -> Self {
        Imagef32 {
            width,
            height,
            pixels: vec![Vector3::new(0.0, 0.0, 0.0); width as usize * height as usize],
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Vector3 {
        // trust the caller to be PERFECT: small project
        self.pixels[(y * self.width + x) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Vector3) {
        self.pixels[(y * self.width + x) as usize] = pixel;
    }
}
