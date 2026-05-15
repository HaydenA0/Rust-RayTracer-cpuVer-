use crate::vector3::Vector3;

pub struct Camera {
    pub origin: Vector3,
    pub pixel_00_loc: Vector3,
    pub pixel_delta_u: Vector3,
    pub pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        let focal_length = 0.6;
        let viewport_width = 2.0;
        let viewport_height = viewport_width / (width as f32 / height as f32);
        let origin = Vector3::new(0.0, 0.0, 0.0);

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / width as f32;
        let pixel_delta_v = viewport_v / height as f32;

        let viewport_upper_left =
            origin - Vector3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);

        let pixel_00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            origin,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}
