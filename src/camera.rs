use ray::Ray;
use vec3::{NumWithVectorOps, Vec3, VectorWithOps};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Camera<T> {
    pub location: Vec3<T>,
    pub lower_left: Vec3<T>,
    pub horizontal: Vec3<T>,
    pub vertical: Vec3<T>,
}

impl<T> Camera<T>
where
    T: NumWithVectorOps,
    Vec3<T>: VectorWithOps<T>,
{
    pub fn new(
        location: Vec3<T>,
        lower_left: Vec3<T>,
        horizontal: Vec3<T>,
        vertical: Vec3<T>,
    ) -> Camera<T> {
        Camera {
            location,
            lower_left,
            horizontal,
            vertical,
        }
    }

    #[allow(dead_code)]
    pub fn location(self) -> Vec3<T> {
        self.location
    }

    pub fn get_ray(self, u: T, v: T) -> Ray<T> {
        Ray::new(
            self.location,
            self.lower_left + (u * self.horizontal) + (v * self.vertical),
        )
    }
}
