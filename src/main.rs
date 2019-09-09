extern crate image;

extern crate find_folder;
extern crate piston_window;
extern crate rand;

use std::sync::Arc;

use camera::Camera;
use display::output_ppm;
use display::run_window_thread;
use hittable::{Hittable, Sphere, World};
use image_data::ImageData;
use material::Material;
use tracer::draw_to_image_data;
use vec3::Vec3;

mod camera;
mod display;
mod hittable;
mod image_data;
mod material;
mod ray;
mod tracer;
mod vec3;

fn main() {
    let nx = 1920usize;
    let ny = 1080usize;

    let image_data = Arc::new(ImageData::new_blank(nx, ny, Vec3::new(0.0, 0.0, 0.0)));

    let window_thread = run_window_thread(Arc::clone(&image_data));

    let camera: Camera<f64> = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
    );

    let world = create_world();

    let samples = 64;
    draw_to_image_data(&image_data, &camera, &world, samples);
    output_ppm(image_data);
    //window_thread.join().unwrap();
}

fn create_world() -> Box<Hittable<f64>> {
    let normals = Material::Normal {};
    let red_matte = Material::Lambertian {
        albedo: Vec3::new(0.8, 0.3, 0.3),
    };
    let green_matte = Material::Lambertian {
        albedo: Vec3::new(0.3, 0.8, 0.3),
    };
    let blue_fuzzy_metal = Material::Metal {
        albedo: Vec3::new(0.3, 0.3, 0.5),
        fuzz: 0.5,
    };
    let shiny_metal = Material::Metal {
        albedo: Vec3::new(0.8, 0.8, 0.8),
        fuzz: 0.005,
    };
    let world: Box<Hittable<f64>> = Box::new(World::new(vec![
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.5),
            0.5,
            blue_fuzzy_metal,
        )),
        Box::new(Sphere::new(Vec3::new(0.0, 2.0, -3.5), 1.5, shiny_metal)),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.5), 0.5, normals)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.5), 0.5, red_matte)),
        Box::new(Sphere::new(
            Vec3::new(0.0, -200.5, -1.0),
            200.0,
            green_matte,
        )),
    ]));
    world
}
