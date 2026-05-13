pub const PI: f32 = 3.14159265358979323846264338327950288419716;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / PI
}

pub const INFINITY: f32 = f32::INFINITY;
