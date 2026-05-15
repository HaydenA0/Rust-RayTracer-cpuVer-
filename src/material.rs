use crate::hitrecord::HitRecord;
use crate::ray::Ray;
use crate::sampling::random_on_hemisphere;
use crate::utils::reflect;
use crate::vector3::Vector3;
use crate::vector3::dot;

pub enum Material {
    Lambertian { albedo: Vector3 },
    Metal { albedo: Vector3, fuzz: f32 },
}

impl Material {
    pub fn get_albedo(&self) -> Vector3 {
        match self {
            Material::Lambertian { albedo } => *albedo,
            Material::Metal { albedo, .. } => *albedo,
        }
    }
    pub fn get_fuzz(&self) -> f32 {
        match self {
            Material::Metal { fuzz, .. } => *fuzz,
            _ => 0.0,
        }
    }
    pub fn new_lambertian(albedo: Vector3) -> Self {
        Material::Lambertian { albedo }
    }
    pub fn new_metal(albedo: Vector3, fuzz: f32) -> Self {
        Material::Metal { albedo, fuzz }
    }

    pub fn scatter(&self, ray_in: Vector3, hit_record: &HitRecord) -> Option<(Vector3, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let direction = random_on_hemisphere(hit_record.normal) + hit_record.normal;
                let ray = Ray::new(hit_record.point, direction);
                let attenuation = *albedo;
                Some((attenuation, ray))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(ray_in.normalize(), hit_record.normal);

                let scattered_direction = reflected + (hit_record.normal * *fuzz);
                let scattered_ray = Ray::new(hit_record.point, scattered_direction);

                if dot(scattered_ray.get_direction(), hit_record.normal) > 0.0 {
                    let attenuation = *albedo;
                    Some((attenuation, scattered_ray))
                } else {
                    None
                }
            }
        }
    }
}
