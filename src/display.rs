extern crate image;

extern crate find_folder;
extern crate piston_window;
extern crate rand;

use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use image::{ImageBuffer, Rgba};
use piston_window::*;

use image_data::ImageData;
use vec3::Vec3;

pub fn run_window_thread(image_data: Arc<ImageData<Vec3<f64>>>) -> JoinHandle<()> {
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

        let assets = find_folder::Search::ParentsThenKids(1, 1)
            .for_folder("assets")
            .unwrap();
        let mut glyphs = window
            .load_font(assets.join("FiraSans-Regular.ttf"))
            .unwrap();

        let initial_empty = ImageBuffer::new(width, height);

        let mut texture: G2dTexture = Texture::from_image(
            &mut texture_context,
            &initial_empty,
            &TextureSettings::new(),
        )
        .unwrap();

        while let Some(event) = window.next() {
            if let Some(_) = event.render_args() {
                let complete_percentage: String =
                    format!("{:.0}%", image_data.get_complete() * 100.0);
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

                    let text_position = context.transform.trans(10.0, 30.0);
                    text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                        .draw(
                            &complete_percentage,
                            &mut glyphs,
                            &context.draw_state,
                            text_position,
                            graphics,
                        )
                        .unwrap();
                    glyphs.factory.encoder.flush(device);
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

pub fn output_ppm(image_data: Arc<ImageData<Vec3<f64>>>) -> () {
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
