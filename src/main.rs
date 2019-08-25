mod camera;
mod canvas;
mod hittable;
mod material;
mod ray;
mod vec3;

extern crate image;
extern crate piston_window;
extern crate rand;

use camera::Camera;
use canvas::Canvas;
use hittable::{Hittable, Sphere, World};
use image::{ImageBuffer, Rgba};
use material::{LightInteraction, Material};
use piston_window::*;
use rand::prelude::ThreadRng;
use rand::Rng;
use ray::Ray;
use std::cmp::{max, min};
use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use vec3::Vec3;

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
    let nx = 800usize;
    let ny = 400usize;

    let canvas = Arc::new(Canvas::new_blank(nx, ny, Vec3::new(0.0, 0.0, 0.0)));

    let window_thread = run_window_thread(Arc::clone(&canvas));

    let camera: Camera<f64> = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
    );

    let world = create_world();

    let samples = 9;
    draw_on_canvas(&canvas, camera, &world, samples);
    output_ppm(canvas);
    window_thread.join().unwrap();
}

fn run_window_thread(image_data: Arc<Canvas<Vec3<f64>>>) -> JoinHandle<()> {
    thread::spawn(move || {
        let width = image_data.x_size() as u32;
        let height = image_data.y_size() as u32;
        let mut window: PistonWindow = WindowSettings::new("RustRay!", [width, height])
            .exit_on_esc(true)
            .build()
            .unwrap();

        let mut texture_context = TextureContext {
            factory: window.factory.clone(),
            encoder: window.factory.create_command_buffer().into(),
        };

        let initial_empty = ImageBuffer::new(width, height);

        let mut texture: G2dTexture = Texture::from_image(
            &mut texture_context,
            &initial_empty,
            &TextureSettings::new(),
        )
        .unwrap();

        while let Some(event) = window.next() {
            if let Some(_) = event.render_args() {
                let pixels: Vec<Vec3<f64>> = image_data.deref().into();
                let max_intensity = max_intensity_from(&pixels);
                let mut sub_pixels: Vec<u8> = pixels
                    .iter()
                    .map(|p| {
                        vec![
                            255u8,
                            f64_to_u8(gamma_correction(normalise(p.b(), max_intensity))),
                            f64_to_u8(gamma_correction(normalise(p.g(), max_intensity))),
                            f64_to_u8(gamma_correction(normalise(p.r(), max_intensity))),
                        ]
                    })
                    .flatten()
                    .collect();
                sub_pixels.reverse(); //TODO fix this hack that was to flip the image vertically but also mirrored it horizontally
                let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
                    image::ImageBuffer::from_raw(width, height, sub_pixels).unwrap();
                texture.update(&mut texture_context, &buffer).unwrap();

                window.draw_2d(&event, |context, graphics, device| {
                    texture_context.encoder.flush(device);
                    clear([1.0; 4], graphics);
                    image(&texture, context.transform, graphics);
                });
            }
        }
    })
}

fn max_intensity_from(pixels: &Vec<Vec3<f64>>) -> f64 {
    pixels.iter().fold(1.0, |max_so_far, colour| {
        let mut max = max_so_far;
        if colour.r() > max {
            max = colour.r()
        };
        if colour.g() > max {
            max = colour.g()
        };
        if colour.b() > max {
            max = colour.b()
        };
        max
    })
}

#[inline]
fn normalise(value: f64, max_intensity: f64) -> f64 {
    value / max_intensity
}

#[inline]
fn gamma_correction(value: f64) -> f64 {
    value.sqrt()
}

#[inline]
fn f64_to_u8(value: f64) -> u8 {
    (value * 255.0) as u8
}

fn draw_on_canvas(
    canvas: &Canvas<Vec3<f64>>,
    camera: Camera<f64>,
    world: &Box<Hittable<f64>>,
    samples: i32,
) {
    let get_pixel_location: fn(usize, usize) -> f64 = match samples {
        1 => get_pixel,
        _ => get_pixel_with_randomness,
    };

    for j in (0..canvas.y_size()).rev() {
        for i in 0..canvas.x_size() {
            let average_colour: Vec3<f64> = (0..samples)
                .map(|_| {
                    camera.get_ray(
                        get_pixel_location(i, canvas.x_size()),
                        get_pixel_location(j, canvas.y_size()),
                    )
                })
                .map(|ray| color(ray, &world, 0.0001, std::f64::MAX, 50))
                .fold(Vec3::new(0.0, 0.0, 0.0), |a, b| a + b)
                / f64::from(samples);

            canvas.update_pixel(i, j, |_| average_colour);
        }
    }
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

fn output_ppm(image_data: Arc<Canvas<Vec3<f64>>>) -> () {
    println!("P3");
    println!("{} {}", image_data.x_size(), image_data.y_size());
    println!("255");
    let mut pixels: Vec<Vec3<f64>> = image_data.deref().into();
    let max_intensity = max_intensity_from(&pixels);
    pixels.reverse(); //TODO: same issue to fix here
    pixels.iter().for_each(|p| {
        println!(
            "{} {} {}",
            f64_to_u8(gamma_correction(normalise(p.r(), max_intensity))),
            f64_to_u8(gamma_correction(normalise(p.g(), max_intensity))),
            f64_to_u8(gamma_correction(normalise(p.b(), max_intensity))),
        );
    });
}
