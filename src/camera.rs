use ray::Ray;
use vec3::Vec3;

use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Camera<T> {
    pub location: Vec3<T>,
    pub lower_left: Vec3<T>,
    pub horizontal: Vec3<T>,
    pub vertical: Vec3<T>,
    pub u: Vec3<T>,
    pub v: Vec3<T>,
    pub w: Vec3<T>,
    pub lens_radius: T,
}

fn random_in_unit_disk() -> Vec3<f64> {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut p: Vec3<f64>;
    loop {
        p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(p) < 1.0 {
            break;
        }
    }
    p
}

impl Camera<f64> {
    pub fn new(
        location: Vec3<f64>,
        look_at: Vec3<f64>,
        vup: Vec3<f64>,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Camera<f64> {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (location - look_at).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        Camera {
            location,
            lower_left: location
                - half_width * focus_distance * u
                - half_height * focus_distance * v
                - focus_distance * w,
            horizontal: 2.0 * half_width * focus_distance * u,
            vertical: 2.0 * half_height * focus_distance * v,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    #[allow(dead_code)]
    pub fn location(self) -> Vec3<f64> {
        self.location
    }

    pub fn get_ray(self, s: f64, t: f64) -> Ray<f64> {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() - self.v * rd.y();
        Ray::new(
            self.location + offset,
            self.lower_left + (s * self.horizontal) + (t * self.vertical) - self.location - offset,
        )
    }
}
