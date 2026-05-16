use crate::hitrecord::HitRecord;
use crate::ray::Ray;
use crate::spheres::Spheres;
use crate::spheres::is_hit_sphere;
use crate::vector3::Vector3;

#[derive(Clone)]
pub struct SphereBHV {
    pub index: usize, // interesting choice
                      // since we are using an array
                      // of spheres, the index is enough
                      // to identify the sphere
}

#[derive(Clone)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}

impl BoundingBox {
    pub fn is_hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for axis in 0..3 {
            let inv_direction = 1.0 / ray.get_direction()[axis];
            let mut t_potential_min = (self.min[axis] - ray.get_origin()[axis]) * inv_direction;
            let mut t_potential_max = (self.max[axis] - ray.get_origin()[axis]) * inv_direction;

            if inv_direction < 0.0 {
                std::mem::swap(&mut t_potential_min, &mut t_potential_max);
            }

            t_min = t_potential_min.max(t_min);
            t_max = t_potential_max.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

pub struct BVHNode {
    pub bounding_box: BoundingBox,
    pub left: Option<Box<BVHNode>>,
    pub right: Option<Box<BVHNode>>,
    pub sphere: SphereBHV,
}

pub enum Axis {
    X,
    Y,
    Z,
}

impl BVHNode {
    pub fn new() -> BVHNode {
        BVHNode {
            bounding_box: BoundingBox {
                min: Vector3::new(0.0, 0.0, 0.0),
                max: Vector3::new(0.0, 0.0, 0.0),
            },
            left: None,
            right: None,
            sphere: SphereBHV { index: 0 },
        }
    }
}

pub fn build_bhv_tree(spheres: &Spheres, indices: &mut [usize]) -> BVHNode {
    let mut node = BVHNode::new();

    node.bounding_box = compute_surrounding_box(spheres, indices);

    let len = indices.len();

    if len == 1 {
        node.sphere = SphereBHV { index: indices[0] };
        node.left = None;
        node.right = None;
        return node;
    }

    let axis = choose_longest_axis(&node.bounding_box);

    sort_indices_by_center(spheres, indices, axis);

    let mid = len / 2;
    let (left_indices, right_indices) = indices.split_at_mut(mid);

    node.left = Some(Box::new(build_bhv_tree(spheres, left_indices)));
    node.right = Some(Box::new(build_bhv_tree(spheres, right_indices)));

    node
}

fn sort_indices_by_center(spheres: &Spheres, indices: &mut [usize], axis: Axis) {
    indices.sort_unstable_by(|&a, &b| {
        let center_a = &spheres.spheres_centers[a];
        let center_b = &spheres.spheres_centers[b];

        match axis {
            Axis::X => center_a.x.total_cmp(&center_b.x),
            Axis::Y => center_a.y.total_cmp(&center_b.y),
            Axis::Z => center_a.z.total_cmp(&center_b.z),
        }
    });
}

pub fn compute_surrounding_box(spheres: &Spheres, indices: &[usize]) -> BoundingBox {
    let first_idx = indices[0];
    let first_center = spheres.spheres_centers[first_idx];
    let first_radius = spheres.spheres_radius[first_idx];

    let mut master_min = first_center - first_radius;
    let mut master_max = first_center + first_radius;

    for &i in &indices[1..] {
        let center = spheres.spheres_centers[i];
        let radius = spheres.spheres_radius[i];

        let min = center - radius;
        let max = center + radius;

        master_min = Vector3::new(
            master_min.x.min(min.x),
            master_min.y.min(min.y),
            master_min.z.min(min.z),
        );
        master_max = Vector3::new(
            master_max.x.max(max.x),
            master_max.y.max(max.y),
            master_max.z.max(max.z),
        );
    }

    BoundingBox {
        min: master_min,
        max: master_max,
    }
}

pub fn choose_longest_axis(bounding_box: &BoundingBox) -> Axis {
    let extent_of_x = bounding_box.max.x - bounding_box.min.x;
    let extent_of_y = bounding_box.max.y - bounding_box.min.y;
    let extent_of_z = bounding_box.max.z - bounding_box.min.z;

    if extent_of_x > extent_of_y && extent_of_x > extent_of_z {
        Axis::X
    } else if extent_of_y > extent_of_z {
        Axis::Y
    } else {
        Axis::Z
    }
}

pub fn hit_bvh(node: &BVHNode, ray: &Ray, hit_record: &mut HitRecord, spheres: &Spheres) -> bool {
    if !node
        .bounding_box
        .is_hit(ray, hit_record.t_min, hit_record.t_max)
    {
        return false;
    }
    if node.left.is_none() && node.right.is_none() {
        let sphere_index = node.sphere.index;
        let center = spheres.spheres_centers[sphere_index];
        let radius = spheres.spheres_radius[sphere_index];
        let hit = is_hit_sphere(*ray, center, radius, hit_record);
        if hit {
            hit_record.current_sphere = sphere_index;
        }
        return hit;
    }

    let hit_left = if let Some(left) = &node.left {
        hit_bvh(left, ray, hit_record, spheres)
    } else {
        false
    };

    let hit_right = if let Some(right) = &node.right {
        hit_bvh(right, ray, hit_record, spheres)
    } else {
        false
    };

    hit_left || hit_right
}
