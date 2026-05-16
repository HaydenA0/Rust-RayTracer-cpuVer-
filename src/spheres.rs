use crate::camera::Camera;
use crate::ray::Ray;
use crate::utils::generate_random_float_in_range;
use crate::utils::generate_random_float_unit;
use crate::vector3::Vector3;
use crate::vector3::dot;

use crate::material::Material;

use crate::hitrecord::HitRecord;

pub struct Spheres {
    pub spheres_centers: Vec<Vector3>,
    pub spheres_radius: Vec<f32>,
    pub spheres_materials: Vec<Material>,
}

impl Spheres {
    pub fn new() -> Self {
        Self {
            spheres_centers: Vec::new(),
            spheres_radius: Vec::new(),
            spheres_materials: Vec::new(),
        }
    }
    pub fn get_len(&self) -> usize {
        self.spheres_centers.len()
    }
}
pub fn setup_spheres_benchmark() -> Spheres {
    let mut spheres = Spheres::new();

    spheres
        .spheres_centers
        .push(Vector3::new(0.0, -1000.0, 0.0));
    spheres.spheres_radius.push(1000.0);
    spheres
        .spheres_materials
        .push(Material::new_lambertian(Vector3::new(0.5, 0.5, 0.5)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = generate_random_float_unit();
            let center = Vector3::new(
                a as f32 + 0.9 * generate_random_float_unit(),
                0.2,
                b as f32 + 0.9 * generate_random_float_unit(),
            );

            let diff = center - Vector3::new(4.0, 0.2, 0.0);
            let distance = (diff.x * diff.x + diff.y * diff.y + diff.z * diff.z).sqrt();

            if distance > 0.9 {
                spheres.spheres_centers.push(center);
                spheres.spheres_radius.push(0.2);

                if choose_mat < 0.8 {
                    let albedo = Vector3::new(
                        generate_random_float_unit() * generate_random_float_unit(),
                        generate_random_float_unit() * generate_random_float_unit(),
                        generate_random_float_unit() * generate_random_float_unit(),
                    );
                    spheres
                        .spheres_materials
                        .push(Material::new_lambertian(albedo));
                } else if choose_mat < 0.95 {
                    let albedo = Vector3::new(
                        generate_random_float_in_range(0.5, 1.0),
                        generate_random_float_in_range(0.5, 1.0),
                        generate_random_float_in_range(0.5, 1.0),
                    );
                    let fuzz = generate_random_float_in_range(0.0, 0.5);
                    spheres
                        .spheres_materials
                        .push(Material::new_metal(albedo, fuzz));
                } else {
                    spheres
                        .spheres_materials
                        .push(Material::new_metal(Vector3::new(0.5, 0.5, 0.5), 0.5));
                }
            }
        }
    }

    spheres.spheres_centers.push(Vector3::new(0.0, 1.0, 0.0));
    spheres.spheres_radius.push(1.0);
    spheres
        .spheres_materials
        .push(Material::new_metal(Vector3::new(0.4, 0.2, 0.1), 0.7));

    spheres.spheres_centers.push(Vector3::new(-4.0, 1.0, 0.0));
    spheres.spheres_radius.push(1.0);
    spheres
        .spheres_materials
        .push(Material::new_lambertian(Vector3::new(0.4, 0.2, 0.1)));

    spheres.spheres_centers.push(Vector3::new(4.0, 1.0, 0.0));
    spheres.spheres_radius.push(1.0);
    spheres
        .spheres_materials
        .push(Material::new_metal(Vector3::new(0.7, 0.6, 0.5), 0.0));

    spheres
}

pub fn is_hit_sphere(
    ray: Ray,
    sphere_center: Vector3,
    sphere_radius: f32,
    hit_record: &mut HitRecord,
) -> bool {
    let oc = sphere_center - ray.get_origin();

    let a = ray.get_direction().length_squared();

    let h = dot(ray.get_direction(), oc);

    let c = oc.length_squared() - sphere_radius * sphere_radius;

    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        return false;
    }

    let delta_sqrt = discriminant.sqrt();

    let roots = [(h - delta_sqrt) / a, (h + delta_sqrt) / a];

    let root = if let Some(&r) = roots
        .iter()
        .find(|&&r| r >= hit_record.t_min && r <= hit_record.t_max)
    {
        r
    } else {
        return false;
    };

    hit_record.t = root;
    hit_record.t_max = root;
    hit_record.point = ray.at(root);
    hit_record.normal = (hit_record.point - sphere_center) / sphere_radius;
    hit_record.resolve_front_face_and_normal(ray.get_direction());

    return true;
}

pub fn setup_sphere_scene1(camera: &Camera) -> Spheres {}
