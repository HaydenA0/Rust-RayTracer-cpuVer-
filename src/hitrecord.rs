use crate::utils::INFINITY;
use crate::vector3::Vector3;
use crate::vector3::dot;

const T_MIN: f32 = 0.0001;

#[repr(C)]
pub struct HitRecord {
    pub point: Vector3,   // 12 bytes
    pub normal: Vector3,  // 12 bytes
    pub t: f32,           // 4 bytes
    pub t_min: f32,       // 4 bytes
    pub t_max: f32,       // 4 bytes
    pub front_face: bool, // 1 byte
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            point: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t_min: T_MIN,
            t_max: INFINITY,
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

    pub fn reset(&mut self) {
        self.t = 0.0;
        self.point = Vector3::new(0.0, 0.0, 0.0);
        self.normal = Vector3::new(0.0, 0.0, 0.0);
        self.t_min = T_MIN;
        self.t_max = INFINITY;
        self.front_face = false;
    }
}
