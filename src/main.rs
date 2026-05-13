mod camera;
mod hitrecord;
mod ray;
mod spheres;
mod utils;
mod vector3;

use crate::camera::Camera;
use crate::hitrecord::HitRecord;
use crate::ray::Ray;
use crate::spheres::{Spheres, is_hit_sphere, setup_spheres};
use crate::utils::{EPSILON, INFINITY};
use crate::vector3::Vector3;

struct Pixelu8 {
    r: u8,
    g: u8,
    b: u8,
}

pub struct Renderer {
    width: u32,
    height: u32,
    camera: Camera,
    spheres: Spheres,
}

impl Renderer {
    pub fn new(width: u32, aspect_ratio: f32) -> Self {
        let height = (width as f32 / aspect_ratio).round() as u32;
        let height = if height < 1 { 1 } else { height };

        let camera = Camera::new(width, height);
        let spheres = setup_spheres();

        Self {
            width,
            height,
            camera,
            spheres,
        }
    }

    pub fn render(&self) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");

        for y in 0..self.height {
            eprintln!("{} Scan lines remaining", (self.height - y));
            for x in 0..self.width {
                let mut hit_record = HitRecord::new();
                hit_record.t_min = EPSILON;
                hit_record.t_max = INFINITY;

                let xf = x as f32;
                let yf = y as f32;

                let pixel_center = self.camera.pixel_00_loc
                    + (self.camera.pixel_delta_u * xf + self.camera.pixel_delta_v * yf);

                let ray_direction = pixel_center - self.camera.origin;
                let ray = Ray::new(self.camera.origin, ray_direction);

                let pixel_f = self.ray_color(ray, &mut hit_record);
                let pixel_u = self.pixel_to_u8(pixel_f);

                println!("{} {} {}", pixel_u.r, pixel_u.g, pixel_u.b);
            }
        }
        eprintln!("Done.");
    }

    fn ray_color(&self, ray: Ray, hit_record: &mut HitRecord) -> Vector3 {
        let mut hit_anything = false;
        let mut final_normal = Vector3::new(0.0, 0.0, 0.0);

        for i in 0..self.spheres.spheres_centers.len() {
            if is_hit_sphere(
                ray,
                self.spheres.spheres_centers[i],
                self.spheres.spheres_radius[i],
                hit_record,
            ) {
                hit_anything = true;
                final_normal = hit_record.normal;

                hit_record.t_max = hit_record.t;
            }
        }

        if hit_anything {
            return (final_normal + 1.0) * 0.5;
        }

        let unit_direction = ray.get_direction();
        let unit_direction: Vector3 = unit_direction / unit_direction.length();
        let a = 0.5 * (unit_direction.y + 1.0);
        Vector3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vector3::new(0.5, 0.7, 1.0) * a
    }

    fn pixel_to_u8(&self, pixel: Vector3) -> Pixelu8 {
        let f = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
        Pixelu8 {
            r: f(pixel.x),
            g: f(pixel.y),
            b: f(pixel.z),
        }
    }
}

fn main() {
    let renderer = Renderer::new(400, 16.0 / 9.0);
    renderer.render();
}
