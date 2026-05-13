mod camera;
mod hitrecord;
mod ray;
mod spheres;
mod utils;
mod vector3;

use crate::camera::Camera;
use crate::hitrecord::HitRecord;
use crate::ray::Ray;
use crate::spheres::Spheres;
use crate::spheres::is_hit_sphere;
use crate::spheres::setup_spheres;
use crate::utils::INFINITY;
use crate::vector3::Vector3;

struct Pixelu8 {
    r: u8,
    g: u8,
    b: u8,
}

fn from_f32_to_u8(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn pixel_from_f32_to_u8(pixel: Vector3) -> Pixelu8 {
    Pixelu8 {
        r: from_f32_to_u8(pixel.x),
        g: from_f32_to_u8(pixel.y),
        b: from_f32_to_u8(pixel.z),
    }
}

fn generate_image_dimensions(width: u32, aspect_ratio: f32) -> (u32, u32) {
    let height = (width as f32 / aspect_ratio).round() as u32;
    assert!(height >= 1);
    return (width, height);
}

pub fn pixel_from_ray_and_spheres(
    ray: Ray,
    spheres: &Spheres,
    hit_record: &mut HitRecord,
) -> Vector3 {
    for i in 0..spheres.spheres_centers.len() {
        if is_hit_sphere(
            ray,
            spheres.spheres_centers[i],
            spheres.spheres_radius[i],
            hit_record,
        ) {
            // return spheres.spheres_colors[i];
            return (hit_record.normal + 1.0) * 0.5;
        }
    }
    let unit_direction = ray.get_direction();
    let unit_direction: Vector3 = unit_direction / unit_direction.length();
    let a = 0.5 * (unit_direction.y + 1.0);
    let output = Vector3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vector3::new(0.5, 0.7, 1.0) * a;
    return output;
}

fn main() {
    let (image_x_width, image_y_height) = generate_image_dimensions(400, 16.0 / 9.0);

    let camera = Camera::new(image_x_width, image_y_height);

    let spheres = setup_spheres();

    let mut hit_record = HitRecord::new();
    hit_record.t_max = INFINITY;

    println!("P3");
    println!("{} {}", image_x_width, image_y_height);
    println!("255");

    for y in 0..image_y_height {
        eprintln!("{} Scan lines remaining", (image_y_height - y));
        for x in 0..image_x_width {
            let xf = x as f32;
            let yf = y as f32;

            let pixel_center =
                camera.pixel_00_loc + (camera.pixel_delta_u * xf + camera.pixel_delta_v * yf);

            let ray_direction = pixel_center - camera.origin;

            let ray = Ray::new(camera.origin, ray_direction);

            let pixel_f = pixel_from_ray_and_spheres(ray, &spheres, &mut hit_record);

            let pixel_u = pixel_from_f32_to_u8(pixel_f);

            println!("{} {} {}", pixel_u.r, pixel_u.g, pixel_u.b);
        }
    }
}
