use crate::vector3::Vector3;

pub struct HitRecord {
    pub t: f32,
    pub point: Vector3,
    pub normal: Vector3,
    pub t_min: f32,
    pub t_max: f32,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            point: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t_min: 0.0,
            t_max: 10.0,
        }
    }
}
