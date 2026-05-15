mod camera;
mod hitrecord;
mod material;
mod ray;
mod render;
mod sampling;
mod spheres;
mod utils;
mod vector3;

use crate::render::Renderer;

fn main() {
    let renderer = Renderer::new(1920, 16.0 / 9.0);
    renderer.render();
}
