use ray::Ray;
use vec3::Vec3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Camera<T> {
    pub location: Vec3<T>,
    pub lower_left: Vec3<T>,
    pub horizontal: Vec3<T>,
    pub vertical: Vec3<T>,
}

impl Camera<f64> {
    pub fn new(location: Vec3<f64>, vfov: f64, aspect: f64) -> Camera<f64> {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        Camera {
            location,
            lower_left: Vec3::new(-half_width, -half_height, -1.0),
            horizontal: Vec3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0 * half_height, 0.0),
        }
    }

    #[allow(dead_code)]
    pub fn location(self) -> Vec3<f64> {
        self.location
    }

    pub fn get_ray(self, u: f64, v: f64) -> Ray<f64> {
        Ray::new(
            self.location,
            self.lower_left + (u * self.horizontal) + (v * self.vertical),
        )
    }
}
