use crate::utils::degrees_to_radians;
use crate::vector3::Vector3;

pub struct Camera {
    pub origin: Vector3,
    pub pixel_00_loc: Vector3,
    pub pixel_delta_u: Vector3,
    pub pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        let lookfrom = Vector3::new(13.0, 2.0, 3.0);
        let lookat = Vector3::new(0.0, 0.0, 0.0);
        let vup = Vector3::new(0.0, 1.0, 0.0);

        let vfov = 20.0;
        let focus_dist = 10.0;

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * (width as f32 / height as f32);

        let w_dir = lookfrom - lookat;
        let w_len = (w_dir.x * w_dir.x + w_dir.y * w_dir.y + w_dir.z * w_dir.z).sqrt();
        let w = w_dir / w_len;

        let u_cross = Vector3::new(
            vup.y * w.z - vup.z * w.y,
            vup.z * w.x - vup.x * w.z,
            vup.x * w.y - vup.y * w.x,
        );
        let u_len = (u_cross.x * u_cross.x + u_cross.y * u_cross.y + u_cross.z * u_cross.z).sqrt();
        let u = u_cross / u_len;

        let v = Vector3::new(
            w.y * u.z - w.z * u.y,
            w.z * u.x - w.x * u.z,
            w.x * u.y - w.y * u.x,
        );

        let origin = lookfrom;

        let viewport_u = u * viewport_width * focus_dist;
        let viewport_v = v * -viewport_height * focus_dist;

        let pixel_delta_u = viewport_u / width as f32;
        let pixel_delta_v = viewport_v / height as f32;

        let viewport_upper_left =
            origin - (w * focus_dist) - (viewport_u / 2.0) - (viewport_v / 2.0);

        let pixel_00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            origin,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
    pub fn new_cin(width: u32, height: u32) -> Self {
        let lookfrom = Vector3::new(6.0, 2.5, 5.0);
        let lookat = Vector3::new(0.0, 0.7, 0.0);
        let vup = Vector3::new(0.0, 1.0, 0.0);

        let vfov = 25.0;

        let w_dir = lookfrom - lookat;
        let w_len = (w_dir.x * w_dir.x + w_dir.y * w_dir.y + w_dir.z * w_dir.z).sqrt();
        let w = w_dir / w_len;

        let focus_dist = w_len;

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * (width as f32 / height as f32);

        let u_cross = Vector3::new(
            vup.y * w.z - vup.z * w.y,
            vup.z * w.x - vup.x * w.z,
            vup.x * w.y - vup.y * w.x,
        );
        let u_len = (u_cross.x * u_cross.x + u_cross.y * u_cross.y + u_cross.z * u_cross.z).sqrt();
        let u = u_cross / u_len;

        let v = Vector3::new(
            w.y * u.z - w.z * u.y,
            w.z * u.x - w.x * u.z,
            w.x * u.y - w.y * u.x,
        );

        let origin = lookfrom;

        let viewport_u = u * viewport_width * focus_dist;
        let viewport_v = v * -viewport_height * focus_dist;

        let pixel_delta_u = viewport_u / width as f32;
        let pixel_delta_v = viewport_v / height as f32;

        let viewport_upper_left =
            origin - (w * focus_dist) - (viewport_u / 2.0) - (viewport_v / 2.0);

        let pixel_00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            origin,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}
