mod camera;
mod hitrecord;
mod image;
mod material;
mod ray;
mod render;
mod sampling;
mod spheres;
mod utils;
mod vector3;

use crate::render::Renderer;

fn main() {
    let output_filename = "output.ppm";
    let renderer = Renderer::new(400, 16.0 / 9.0, 50, 5);
    let image = renderer.render_and_fill_image();
    let results = renderer.write_ppm(&image, output_filename);
    match results {
        Err(e) => println!("Error: {}", e),
        Ok(_) => (),
    }
}
