fn main() {
    let nx = 200u32;
    let ny = 100u32;
    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2f32;
            let ir = (255.99*r) as u32;
            let ig = (255.99*g) as u32;
            let ib = (255.99*b) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
