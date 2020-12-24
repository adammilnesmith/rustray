extern crate find_folder;
extern crate image;
extern crate piston_window;
extern crate rand;
extern crate rayon;

use rand::prelude::ThreadRng;
use rand::Rng;
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

    let image_data: Arc<ImageData<Vec3<f64>>> =
        Arc::new(ImageData::new_blank(nx, ny, Vec3::new(0.0, 0.0, 0.0)));

    let _window_thread = run_window_thread(Arc::clone(&image_data));

    let camera: Camera<f64> = Camera::new(
        Vec3::new(7.5, 1.5, -2.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        60.0,
        nx as f64 / ny as f64,
        0.05,
        4.00,
    );

    let world = Arc::new(create_random_world());

    let samples = 1024;
    draw_to_image_data(&image_data, &camera, &world, samples);
    output_ppm(image_data);
    //window_thread.join().unwrap();
}

fn create_world() -> Box<dyn Hittable<f64>> {
    let normals = Material::Normal {};
    let red_matte = Material::Lambertian {
        albedo: Vec3::new(0.8_f64, 0.3_f64, 0.3_f64),
    };
    let green_matte = Material::Lambertian {
        albedo: Vec3::new(0.3_f64, 0.8_f64, 0.3_f64),
    };
    let blue_fuzzy_metal = Material::Metal {
        albedo: Vec3::new(0.3_f64, 0.3_f64, 0.5_f64),
        fuzz: 0.5,
    };
    let shiny_metal = Material::Metal {
        albedo: Vec3::new(0.8_f64, 0.8_f64, 0.8_f64),
        fuzz: 0.005,
    };
    let glass = Material::Dieletric {
        refractive_index: 1.5,
    };
    let world: Box<dyn Hittable<f64>> = Box::new(World::new(vec![
        Box::new(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.5),
            0.5,
            blue_fuzzy_metal,
        )),
        Box::new(Sphere::new(Vec3::new(0.0, 2.0, -3.5), 1.5, shiny_metal)),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.5), 0.5, normals)),
        Box::new(Sphere::new(Vec3::new(0.5, -0.25, -1.0), 0.25, red_matte)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.5), 0.5, glass)),
        Box::new(Sphere::new(
            Vec3::new(0.0, -200.5, -1.0),
            200.0,
            green_matte,
        )),
    ]));
    world
}

fn create_random_world() -> Box<dyn Hittable<f64>> {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut hittables: Vec<Box<dyn Hittable<f64>>> = (0..500)
        .flat_map(|i| {
            let a = (((i / 22) % 22) - 11) as f64;
            let b = (i % 22 - 11) as f64;
            let center = Vec3::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());
            if (center.clone() - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = create_random_material(rng);
                let sphere: Box<dyn Hittable<f64>> = Box::new(Sphere::new(center, 0.2, material));
                vec![sphere]
            } else {
                vec![]
            }
        })
        .collect();
    hittables.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        },
    )));
    hittables.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dieletric {
            refractive_index: 1.5,
        },
    )));
    hittables.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            albedo: Vec3::new(0.4, 0.2, 0.1),
        },
    )));
    hittables.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            albedo: Vec3::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    )));
    Box::new(World::new(hittables))
}

fn create_random_material(mut rng: ThreadRng) -> Material<f64> {
    let choose_mat = rng.gen::<f64>();
    let material = if choose_mat < 0.8 {
        Material::Lambertian {
            albedo: Vec3::new(
                rng.gen::<f64>() * rng.gen::<f64>(),
                rng.gen::<f64>() * rng.gen::<f64>(),
                rng.gen::<f64>() * rng.gen::<f64>(),
            ),
        }
    } else if choose_mat < 0.95 {
        Material::Metal {
            albedo: Vec3::new(
                0.5 * (1.0 + rng.gen::<f64>()),
                0.5 * (1.0 + rng.gen::<f64>()),
                0.5 * (1.0 + rng.gen::<f64>()),
            ),
            fuzz: 0.5 * rng.gen::<f64>(),
        }
    } else {
        Material::Dieletric {
            refractive_index: 1.5,
        }
    };
    material
}
