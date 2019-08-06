mod ray;
mod vec3;
use ray::Ray;
use vec3::Vec3;

fn sky_color(ray: Ray<f64>) -> Vec3<f64> {
    let unit_direction = ray.direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0)) + (t * Vec3::new(0.5, 0.7, 1.0))
}

fn main() {
    let nx = 200u32;
    let ny = 100u32;
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

            let colour: Vec3<f64> = sky_color(ray) * 255.99;
            println!(
                "{} {} {}",
                colour.r() as u32,
                colour.g() as u32,
                colour.b() as u32
            );
        }
    }
}
