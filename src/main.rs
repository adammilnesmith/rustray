mod camera;
mod canvas;
mod hittable;
mod material;
mod ray;
mod vec3;

extern crate rand;

use camera::Camera;
use canvas::Canvas;
use hittable::{Hittable, Sphere, World};
use material::{LightInteraction, Material};
use rand::prelude::ThreadRng;
use rand::Rng;
use ray::Ray;
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
    let samples = 9;

    let camera: Camera<f64> = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
    );

    let normals = Material::Normal {};
    let red_matte = Material::Lambertian {
        albedo: Vec3::new(0.8, 0.3, 0.3),
    };
    let green_matte = Material::Lambertian {
        albedo: Vec3::new(0.3, 0.8, 0.3),
    };
    let blue_fuzzy_metal = Material::Metal {
        albedo: Vec3::new(0.3, 0.3, 0.5),
        fuzz: 0.8,
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
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            green_matte,
        )),
    ]));

    let get_pixel_location: fn(usize, usize) -> f64 = match samples {
        1 => get_pixel,
        _ => get_pixel_with_randomness,
    };

    let canvas = Canvas::new_blank(nx, ny, Vec3::new(0.0, 0.0, 0.0));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let average_colour: Vec3<f64> = (0..samples)
                .map(|_| camera.get_ray(get_pixel_location(i, nx), get_pixel_location(j, ny)))
                .map(|ray| color(ray, &world, 0.0001, std::f64::MAX, 50))
                .fold(Vec3::new(0.0, 0.0, 0.0), |a, b| a + b)
                / f64::from(samples);

            canvas.update_pixel(i, j, |_| average_colour);
        }
    }

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
    for j in (0..ny).rev() {
        for i in 0..nx {
            let pixel_colour: Vec3<f64> = canvas.read_pixel(i, j).map(f64::sqrt) * 255.99;
            println!(
                "{} {} {}",
                pixel_colour.r() as u32,
                pixel_colour.g() as u32,
                pixel_colour.b() as u32
            );
        }
    }
}
