extern crate rayon;

use std::sync::Arc;

use rand::prelude::ThreadRng;
use rand::Rng;
use rayon::prelude::*;

use camera::Camera;
use hittable::Hittable;
use image_data::ImageData;
use material::LightInteraction;
use ray::Ray;
use vec3::Vec3;

#[derive(Debug, PartialEq, Copy, Clone)]
struct WorkItem {
    pub j: usize,
    pub x_size: usize,
    pub y_size: usize,
    pub sample: i32,
}

pub fn draw_to_image_data(
    image_data: &Arc<ImageData<Vec3<f64>>>,
    camera: &Camera<f64>,
    world: &Arc<Box<dyn Hittable<f64>>>,
    samples: i32,
) {
    let work_items: Vec<WorkItem> = (0..samples)
        .flat_map(|sample| {
            (0..image_data.y_size())
                .rev()
                .map(|j| WorkItem {
                    j,
                    x_size: image_data.x_size,
                    y_size: image_data.y_size,
                    sample,
                })
                .collect::<Vec<WorkItem>>()
        })
        .collect();
    let work_item_fraction_of_total: f64 = 1.0 / work_items.len() as f64;
    work_items.par_iter().for_each(|work_item: &WorkItem| {
        for i in 0..image_data.x_size() {
            draw_to_image_data_for_pixel(
                image_data,
                camera,
                world,
                i,
                work_item.j,
                work_item.sample,
            );
        }
        image_data.update_complete(|prev| prev + work_item_fraction_of_total);
    });
}

fn draw_to_image_data_for_pixel(
    image_data: &ImageData<Vec3<f64>>,
    camera: &Camera<f64>,
    world: &Box<dyn Hittable<f64>>,
    i: usize,
    j: usize,
    sample: i32,
) {
    let ray = camera.get_ray(
        get_pixel_with_randomness(i, image_data.x_size()),
        get_pixel_with_randomness(j, image_data.y_size()),
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
#[allow(dead_code)]
fn get_pixel(i: usize, nx: usize) -> f64 {
    i as f64 / nx as f64
}
