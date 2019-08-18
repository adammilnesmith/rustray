use material::Material;
use ray::Ray;
use vec3::{NumWithVectorOps, Vec3, VectorWithOps};

pub trait Hittable<T> {
    fn hit(&self, ray: Ray<T>, min_t: T, max_t: T) -> Option<Hit<T>>;
}

pub struct Hit<T> {
    pub t: T,
    pub normal: Ray<T>,
    pub material: Material<T>,
}

impl<T> Hit<T>
where
    T: NumWithVectorOps,
    Vec3<T>: VectorWithOps<T>,
{
    #[inline]
    pub fn new(t: T, normal: Ray<T>, material: Material<T>) -> Hit<T> {
        Hit {
            t,
            normal,
            material,
        }
    }

    #[inline]
    pub fn t(&self) -> &T {
        &self.t
    }

    #[inline]
    pub fn normal(&self) -> &Ray<T> {
        &self.normal
    }

    #[inline]
    pub fn material(&self) -> &Material<T> {
        &self.material
    }
}

pub struct World<T> {
    pub objects: Vec<Box<dyn Hittable<T>>>,
}

impl World<f64> {
    #[inline]
    pub fn new(objects: Vec<Box<Hittable<f64>>>) -> World<f64> {
        World { objects }
    }

    #[inline]
    pub fn objects(&self) -> &Vec<Box<Hittable<f64>>> {
        &self.objects
    }
}

impl Hittable<f64> for World<f64> {
    fn hit(&self, ray: Ray<f64>, min_t: f64, max_t: f64) -> Option<Hit<f64>> {
        self.objects()
            .iter()
            .map(|hittable| hittable.hit(ray, min_t, max_t))
            .flat_map(|option| option.into_iter())
            .min_by(|hit_a, hit_b| hit_a.t().partial_cmp(&hit_b.t()).unwrap())
    }
}

pub struct Sphere<T> {
    pub center: Vec3<T>,
    pub radius: T,
    pub material: Material<T>,
}

impl Sphere<f64> {
    #[inline]
    pub fn new(center: Vec3<f64>, radius: f64, material: Material<f64>) -> Sphere<f64> {
        Sphere {
            center,
            radius,
            material,
        }
    }

    #[inline]
    pub fn center(&self) -> &Vec3<f64> {
        &self.center
    }

    #[inline]
    pub fn radius(self) -> f64 {
        self.radius
    }

    #[inline]
    pub fn material(&self) -> &Material<f64> {
        &self.material
    }
}

impl Hittable<f64> for Sphere<f64> {
    fn hit(&self, ray: Ray<f64>, min_t: f64, max_t: f64) -> Option<Hit<f64>> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if min_t < t && t < max_t {
                let hit_point = ray.point_at_parameter(t);
                let normal = (hit_point - *self.center()).unit();
                Some(Hit::new(t, Ray::new(hit_point, normal), *self.material()))
            } else {
                None
            }
        }
    }
}
