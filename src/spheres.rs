use crate::ray::Ray;
use crate::vector3::Vector3;
use crate::vector3::dot;

use crate::hitrecord::HitRecord;

pub struct Spheres {
    pub spheres_centers: Vec<Vector3>,
    pub spheres_radius: Vec<f32>,
    pub spheres_colors: Vec<Vector3>,
}

impl Spheres {
    pub fn new() -> Self {
        Self {
            spheres_centers: Vec::new(),
            spheres_radius: Vec::new(),
            spheres_colors: Vec::new(),
        }
    }
}
pub fn setup_spheres() -> Spheres {
    let mut spheres = Spheres::new();

    let sphere_center = Vector3::new(0.0, -101.0, -1.0);
    let sphere_radius = 100.0;

    spheres.spheres_centers.push(sphere_center);
    spheres.spheres_radius.push(sphere_radius);
    spheres.spheres_colors.push(Vector3::new(0.0, 1.0, 0.0));

    let sphere_center = Vector3::new(0.0, 0.0, -2.0);
    let sphere_radius = 0.5;

    spheres.spheres_centers.push(sphere_center);
    spheres.spheres_radius.push(sphere_radius);
    spheres.spheres_colors.push(Vector3::new(1.0, 0.0, 0.0));

    let sphere_center = Vector3::new(1.0, 0.0, -5.0);
    let sphere_radius = 0.75;

    spheres.spheres_centers.push(sphere_center);
    spheres.spheres_radius.push(sphere_radius);
    spheres.spheres_colors.push(Vector3::new(0.0, 0.0, 1.0));

    return spheres;
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

    let Some(&root) = roots
        .iter()
        .find(|&&r| r >= hit_record.t_min && r <= hit_record.t_max)
    else {
        return false;
    };

    hit_record.t = root;
    hit_record.point = ray.at(root);
    hit_record.normal = (hit_record.point - sphere_center) / sphere_radius;
    hit_record.resolve_front_face_and_normal(ray.get_direction());

    return true;
}
