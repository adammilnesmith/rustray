mod hittable;
mod ray;
mod vec3;
use hittable::{Hittable, Sphere, World};
use ray::Ray;
use vec3::Vec3;

fn color(ray: Ray<f64>, hittable: &Box<dyn Hittable<f64>>) -> Vec3<f64> {
    let hit = hittable.normal_if_hit(ray);
    if hit.is_some() {
        let normal = hit.unwrap().normal().direction();
        0.5 * normal.map(|i: f64| -> f64 { i + 1.0 })
    } else {
        sky_color(ray)
    }
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

fn main() {
    let nx = 800u32;
    let ny = 400u32;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lower_left: Vec3<f64> = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal: Vec3<f64> = Vec3::new(4.0, 0.0, 0.0);
    let vertical: Vec3<f64> = Vec3::new(0.0, 2.0, 0.0);
    let origin: Vec3<f64> = Vec3::new(0.0, 0.0, 0.0);

    let world: Box<Hittable<f64>> = Box::new(World::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = f64::from(i) / f64::from(nx);
            let v = f64::from(j) / f64::from(ny);

            let ray = Ray::new(origin, lower_left + (u * horizontal) + (v * vertical));

            let colour: Vec3<f64> = color(ray, &world) * 255.99;
            println!(
                "{} {} {}",
                colour.r() as u32,
                colour.g() as u32,
                colour.b() as u32
            );
        }
    }
}
