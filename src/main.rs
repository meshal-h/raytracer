use std::io::Write;
use std::fs::File;

use raytracer::scenes::final_scene;
// use raytracer::scenes::penultimate_scene;

fn main() {
    
    // scene
    let (world, camera) = final_scene();
    
    // render
    let buffer = camera.render(&world);
    
    // write image
    let mut file =  File::create("image.ppm").expect("Unable to create image file.");
    file.write(&buffer).expect("Unable to write image file.");

}
