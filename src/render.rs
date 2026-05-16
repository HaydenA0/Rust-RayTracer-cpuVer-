use crate::camera::Camera;
use crate::hitrecord::HitRecord;
use crate::ray::Ray;
use crate::utils::display_progress;
use crate::utils::smaple_from_unit_square;

use std::fs::File;
use std::io::{BufWriter, Write};

use crate::image::Imagef32;
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
    samples_per_pixel: u32,
    max_depth_recursion: u32,
}

impl Renderer {
    pub fn new(
        width: u32,
        aspect_ratio: f32,
        samples_per_pixel: u32,
        max_depth_recursion: u32,
    ) -> Self {
        let height = (width as f32 / aspect_ratio).round() as u32;
        let height = if height < 1 { 1 } else { height };

        let camera = Camera::new(width, height);
        let spheres = setup_spheres();

        let samples_per_pixel = samples_per_pixel;
        let max_depth_recursion = max_depth_recursion;

        Self {
            width,
            height,
            camera,
            spheres,
            samples_per_pixel,
            max_depth_recursion,
        }
    }

    pub fn write_ppm(&self, image: &Imagef32, filename: &str) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        (0..self.height).for_each(|y| {
            display_progress(self.height, y);

            (0..self.width).for_each(|x| {
                let pixel = image.get_pixel(x, y);
                let pixelu8 = self.pixel_to_u8(pixel);

                let _ = writeln!(writer, "{} {} {}", pixelu8.r, pixelu8.g, pixelu8.b);
            });
        });

        Ok(())
    }

    pub fn render_and_fill_image(&self) -> Imagef32 {
        let mut image = Imagef32::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let mut hit_record = HitRecord::new();
                hit_record.t_min = EPSILON;
                hit_record.t_max = INFINITY;

                let mut pixel_f = Vector3::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let mut recursion_depth = 0;
                    hit_record.reset();
                    let ray = get_ray_at_coordinates(x, y, &self.camera);
                    pixel_f = pixel_f + self.ray_color(ray, &mut hit_record, &mut recursion_depth);
                }
                pixel_f = pixel_f * (1.0 / self.samples_per_pixel as f32);
                image.set_pixel(x, y, pixel_f);
            }
        }
        image
    }

    fn ray_color(
        &self,
        ray: Ray,
        hit_record: &mut HitRecord,
        recursion_depth: &mut u32,
    ) -> Vector3 {
        let mut hit_anything = false;
        let mut current_sphere = 0;

        for i in 0..self.spheres.spheres_centers.len() {
            if is_hit_sphere(
                ray,
                self.spheres.spheres_centers[i],
                self.spheres.spheres_radius[i],
                hit_record,
            ) {
                hit_anything = true;
                current_sphere = i;

                hit_record.t_max = hit_record.t;
            }
        }
        if *recursion_depth > self.max_depth_recursion {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if hit_anything {
            // return if return_color {
            //     self.spheres.spheres_colors[current_sphere]
            // } else {
            //     (final_normal + 1.0) * 0.5
            // };
            // NOTE : some reccursive magic

            if let Some((attenuation, ray)) = self.spheres.spheres_materials[current_sphere]
                .scatter(ray.get_direction(), hit_record)
            {
                *recursion_depth += 1;
                return self.ray_color(ray, hit_record, recursion_depth) * attenuation;
            } else {
                return Vector3::new(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = ray.get_direction();
        let unit_direction: Vector3 = unit_direction / unit_direction.length();
        let a = 0.5 * (unit_direction.y + 1.0);
        Vector3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vector3::new(0.5, 0.7, 1.0) * a
    }

    fn pixel_to_u8(&self, pixel: Vector3) -> Pixelu8 {
        // gamma_correction 2.0
        // TODO : make it configurable
        let gamma_correction = |value: f32| (value.sqrt()).clamp(0.0, 1.0) as f32;
        let f = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
        Pixelu8 {
            r: f(gamma_correction(pixel.x)),
            g: f(gamma_correction(pixel.y)),
            b: f(gamma_correction(pixel.z)),
        }
    }
}

pub fn get_ray_at_coordinates(i: u32, j: u32, camera: &Camera) -> Ray {
    let offset = smaple_from_unit_square();
    let pixel_sample = camera.pixel_00_loc
        + (camera.pixel_delta_u * (offset.x + i as f32))
        + (camera.pixel_delta_v * (offset.y + j as f32));

    let ray_origin = camera.origin;
    let ray_direction = pixel_sample - ray_origin;

    Ray::new(ray_origin, ray_direction)
}
