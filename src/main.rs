extern crate image;

extern crate find_folder;
extern crate piston_window;
extern crate rand;

use std::sync::Arc;

use rand::prelude::ThreadRng;
use rand::Rng;

use camera::Camera;
use display::output_ppm;
use display::run_window_thread;
use hittable::{Hittable, Sphere, World};
use image_data::ImageData;
use material::{LightInteraction, Material};
use ray::Ray;
use vec3::Vec3;

mod camera;
mod display;
mod hittable;
mod image_data;
mod material;
mod ray;
mod vec3;

fn color(
    ray: Ray<f64>,
    hittable: &Box<dyn Hittable<f64>>,
    min_t: f64,
    max_t: f64,
    max_depth: u32,
) -> Vec3<f64> {
    hittable.hit(ray, min_t, max_t).map_or_else(
        || sky_color(ray),
        |hit| {
            let interaction: LightInteraction<f64> = if max_depth <= 0 {
                LightInteraction::new(Vec3::new(0.0, 0.0, 0.0), vec![])
            } else {
                hit.material().interact(ray, &hit.normal())
            };
            interaction
                .scattered_rays()
                .iter()
                .map(|scattered_ray| {
                    scattered_ray.attenuation()
                        * color(
                            scattered_ray.ray(),
                            hittable,
                            0.0001,
                            std::f64::MAX,
                            max_depth - 1,
                        )
                })
                .fold(interaction.directly_emitted(), |a, b| a + b)
        },
    )
}

fn sky_color(ray: Ray<f64>) -> Vec3<f64> {
    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    interpolate(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), t)
}

#[inline]
fn interpolate(first: Vec3<f64>, second: Vec3<f64>, factor: f64) -> Vec3<f64> {
    first * factor + second * (1.0 - factor)
}

#[inline]
fn get_pixel_with_randomness(i: usize, nx: usize) -> f64 {
    let mut rng: ThreadRng = rand::thread_rng();
    (i as f64 + rng.gen::<f64>()) / nx as f64
}

#[inline]
fn get_pixel(i: usize, nx: usize) -> f64 {
    i as f64 / nx as f64
}

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

fn draw_to_image_data(
    image_data: &ImageData<Vec3<f64>>,
    camera: &Camera<f64>,
    world: &Box<Hittable<f64>>,
    samples: i32,
) {
    let pixel_location: fn(usize, usize) -> f64 = match samples {
        1 => get_pixel,
        _ => get_pixel_with_randomness,
    };

    for sample in 0..samples {
        for j in (0..image_data.y_size()).rev() {
            for i in 0..image_data.x_size() {
                if i % 2 == j % 2 {
                    draw_to_image_data_for_pixel(
                        image_data,
                        camera,
                        world,
                        i,
                        j,
                        pixel_location,
                        sample,
                    );
                }
            }
            let number_of_rows_so_far = (1 + image_data.y_size() - j) as f64 / 2.0;
            let proportion_of_sample = (number_of_rows_so_far) / (image_data.y_size() as f64);
            let total_proportion = (f64::from(sample) + proportion_of_sample) / f64::from(samples);
            image_data.update_complete(|_| total_proportion);
        }
        for j in (0..image_data.y_size()).rev() {
            for i in 0..image_data.x_size() {
                if i % 2 != j % 2 {
                    draw_to_image_data_for_pixel(
                        image_data,
                        camera,
                        world,
                        i,
                        j,
                        pixel_location,
                        sample,
                    );
                }
            }
            let number_of_rows_so_far = (1 + image_data.y_size() - j) as f64 / 2.0;
            let proportion_of_sample = 0.5 + (number_of_rows_so_far / (image_data.y_size() as f64));
            let total_proportion = (f64::from(sample) + proportion_of_sample) / f64::from(samples);
            image_data.update_complete(|_| total_proportion);
        }
    }
}

fn draw_to_image_data_for_pixel(
    image_data: &ImageData<Vec3<f64>>,
    camera: &Camera<f64>,
    world: &Box<Hittable<f64>>,
    i: usize,
    j: usize,
    pixel_location: fn(usize, usize) -> f64,
    sample: i32,
) {
    let ray = camera.get_ray(
        pixel_location(i, image_data.x_size()),
        pixel_location(j, image_data.y_size()),
    );
    let pixel_colour = color(ray, &world, 0.0001, std::f64::MAX, 50);
    match sample {
        0 => image_data.update_pixel(i, j, |_| pixel_colour),
        _ => image_data.update_pixel(i, j, |old_avg| {
            let old_sum = old_avg * f64::from(sample - 1);
            let new_sum = old_sum + pixel_colour;
            new_sum / f64::from(sample)
        }),
    };
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
