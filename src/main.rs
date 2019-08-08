mod ray;
mod vec3;
use ray::Ray;
use vec3::Vec3;

fn color(ray: Ray<f64>) -> Vec3<f64> {
    let sphere_center = Vec3::new(0.0, 0.0, -1.0);
    let sphere_radius = 0.5;
    let t = hit_sphere(ray, sphere_center, sphere_radius);
    if t.is_some() {
        let hit_point = ray.point_at_parameter(t.unwrap());
        let normal = (hit_point - sphere_center).unit();
        sphere_radius * normal.map(|i: f64| -> f64 { i + 1.0 })
    } else {
        sky_color(ray)
    }
}

fn hit_sphere(ray: Ray<f64>, center: Vec3<f64>, radius: f64) -> Option<f64> {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
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

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = f64::from(i) / f64::from(nx);
            let v = f64::from(j) / f64::from(ny);

            let ray = Ray::new(origin, lower_left + (u * horizontal) + (v * vertical));

            let colour: Vec3<f64> = color(ray) * 255.99;
            println!(
                "{} {} {}",
                colour.r() as u32,
                colour.g() as u32,
                colour.b() as u32
            );
        }
    }
}
