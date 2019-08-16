mod camera;
mod hittable;
mod ray;
mod vec3;
extern crate rand;
use camera::Camera;
use hittable::{Hittable, Sphere, World};
use rand::prelude::ThreadRng;
use rand::Rng;
use ray::Ray;
use vec3::Vec3;

fn color(ray: Ray<f64>, min_t: f64, max_t: f64, hittable: &Box<dyn Hittable<f64>>) -> Vec3<f64> {
    let hit = hittable.hit(ray, min_t, max_t);
    if hit.is_some() {
        let normal = hit.unwrap().normal();
        let target = normal.origin() + normal.direction() + random_in_unit_sphere();
        0.5 * color(
            Ray::new(normal.origin(), target - normal.origin()),
            0.0001,
            std::f64::MAX,
            hittable,
        )
    //TODO: decide how to toggle normals rather than commenting this out
    //0.5 * normal.direction().map(|i: f64| -> f64 { i + 1.0 })
    } else {
        sky_color(ray)
    }
}

fn random_in_unit_sphere() -> Vec3<f64> {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut p: Vec3<f64>;
    loop {
        p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
            - Vec3::new(1f64, 1f64, 1f64);
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
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
fn get_pixel_with_randomness(i: u32, nx: u32) -> f64 {
    let mut rng: ThreadRng = rand::thread_rng();
    (f64::from(i) + rng.gen::<f64>()) / f64::from(nx)
}

#[inline]
fn get_pixel(i: u32, nx: u32) -> f64 {
    f64::from(i) / f64::from(nx)
}

fn main() {
    let nx = 800u32;
    let ny = 400u32;
    let samples = 9;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let camera: Camera<f64> = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
    );

    let world: Box<Hittable<f64>> = Box::new(World::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]));

    let get_pixel_location: fn(u32, u32) -> f64 = match samples {
        1 => get_pixel,
        _ => get_pixel_with_randomness,
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let average_colour: Vec3<f64> = (0..samples)
                .map(|sample| camera.get_ray(get_pixel_location(i, nx), get_pixel_location(j, ny)))
                .map(|ray| color(ray, 0.0001, std::f64::MAX, &world))
                .fold(Vec3::new(0.0, 0.0, 0.0), |a, b| a + b)
                / f64::from(samples);

            let pixel_colour: Vec3<f64> = average_colour.map(f64::sqrt) * 255.99;
            println!(
                "{} {} {}",
                pixel_colour.r() as u32,
                pixel_colour.g() as u32,
                pixel_colour.b() as u32
            );
        }
    }
}
