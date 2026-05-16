use crate::hitrecord::HitRecord;
use crate::ray::Ray;
use crate::sampling::random_on_hemisphere;
use crate::utils::reflect;
use crate::vector3::Vector3;
use crate::vector3::dot;

pub enum Material {
    Lambertian { albedo: Vector3 },
    Metal { albedo: Vector3, fuzz: f32 },
    Dielectric { albedo: Vector3, ir: f32 },
    DiffuseLight { albedo: Vector3 },
}

impl Material {
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    pub fn emitted(&self) -> Vector3 {
        match self {
            Material::DiffuseLight { albedo } => *albedo,
            Material::Lambertian { albedo } => *albedo * 0.0,
            Material::Metal { albedo, .. } => *albedo * 0.0,
            Material::Dielectric { albedo, .. } => *albedo * 0.0,
        }
    }

    pub fn new_lambertian(albedo: Vector3) -> Self {
        Material::Lambertian { albedo }
    }

    pub fn new_metal(albedo: Vector3, fuzz: f32) -> Self {
        Material::Metal { albedo, fuzz }
    }

    pub fn new_dielectric(albedo: Vector3, ir: f32) -> Self {
        Material::Dielectric { albedo, ir }
    }

    pub fn new_diffuse_light(albedo: Vector3) -> Self {
        Material::DiffuseLight { albedo }
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
            Material::Dielectric { albedo, ir } => {
                let unit_direction = ray_in.normalize();
                let cos_theta = dot(unit_direction * -1.0, hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let refraction_ratio = if dot(unit_direction, hit_record.normal) > 0.0 {
                    *ir
                } else {
                    1.0 / *ir
                };
                let cannot_refract = refraction_ratio * sin_theta > 1.0;

                let direction = if cannot_refract
                    || Self::reflectance(cos_theta, refraction_ratio) > rand::random::<f32>()
                {
                    reflect(unit_direction, hit_record.normal)
                } else {
                    let r_out_perp =
                        (unit_direction + hit_record.normal * cos_theta) * refraction_ratio;
                    let r_out_parallel =
                        hit_record.normal * -((1.0 - dot(r_out_perp, r_out_perp)).abs().sqrt());
                    r_out_perp + r_out_parallel
                };

                Some((*albedo, Ray::new(hit_record.point, direction)))
            }

            Material::DiffuseLight { .. } => None,
        }
    }
}
