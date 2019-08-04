mod ray;
mod vec3;
use vec3::Vec3;

fn main() {
    let nx = 200u32;
    let ny = 100u32;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
    for j in (0..ny).rev() {
        for i in 0..nx {
            let vector: Vec3<f64> =
                Vec3::new(i as f64 / nx as f64, j as f64 / ny as f64, 0.2f64) * 255.99;
            println!(
                "{} {} {}",
                vector.x as u32, vector.y as u32, vector.z as u32
            );
        }
    }
}
