use crate::vector3::Vector3;
use crate::vector3::dot;

pub fn sample_from_unit_sphere() -> Vector3 {
    loop {
        let p = Vector3::new_random_in_range(-1.0, 1.0);
        let lensq = p.length_squared();

        if lensq >= 1.0 || lensq < 1e-24 {
            continue;
        }
        return p / lensq.sqrt();
    }
}

pub fn random_on_hemisphere(normal: Vector3) -> Vector3 {
    let on_unit_sphere = sample_from_unit_sphere();
    if dot(normal, on_unit_sphere) > 0.0 {
        return on_unit_sphere;
    }
    return on_unit_sphere * -1.0;
}
