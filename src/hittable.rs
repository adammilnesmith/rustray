use ray::Ray;
use vec3::{NumWithVectorOps, Vec3, VectorWithOps};

pub trait Hittable<T> {
    fn hit(self, ray: Ray<T>) -> Option<Vec3<T>>;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere<T> {
    pub center: Vec3<T>,
    pub radius: T,
}

impl Sphere<f64> {
    pub fn new(center: Vec3<f64>, radius: f64) -> Sphere<f64> {
        Sphere { center, radius }
    }

    pub fn center(self) -> Vec3<f64> {
        self.center
    }

    pub fn radius(self) -> f64 {
        self.radius
    }
}

impl Hittable<f64> for Sphere<f64> {
    fn hit(self, ray: Ray<f64>) -> Option<Vec3<f64>> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            Some(ray.point_at_parameter(t))
        }
    }
}
