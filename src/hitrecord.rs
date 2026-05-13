use crate::vector3::Vector3;
use crate::vector3::dot;

pub struct HitRecord {
    pub t: f32,
    pub point: Vector3,
    pub normal: Vector3,
    pub t_min: f32,
    pub t_max: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            point: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t_min: 0.0,
            t_max: 10.0,
            front_face: false,
        }
    }
    pub fn resolve_front_face_and_normal(&mut self, ray_direction: Vector3) {
        if dot(ray_direction, self.normal) > 0.0 {
            self.normal = self.normal * -1.0;
        } else {
            self.front_face = true;
        }
    }
}
