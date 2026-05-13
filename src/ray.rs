use crate::vector3::Vector3;

pub type Point = Vector3;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Point,
    direction: Vector3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
    pub fn new(origin: Point, direction: Vector3) -> Self {
        Self { origin, direction }
    }
    pub fn get_direction(&self) -> Vector3 {
        self.direction
    }
    pub fn get_origin(&self) -> Point {
        self.origin
    }
}
