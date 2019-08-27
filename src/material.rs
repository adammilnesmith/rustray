extern crate rand;

use rand::prelude::ThreadRng;
use rand::Rng;

use ray::Ray;
use vec3::Vec3;

#[derive(Debug, PartialEq)]
pub struct LightInteraction<T> {
    pub directly_emitted: Vec3<T>,
    pub scattered_rays: Vec<ScatteredRay<T>>,
}

impl LightInteraction<f64> {
    #[inline]
    pub fn new(
        directly_emitted: Vec3<f64>,
        scattered_rays: Vec<ScatteredRay<f64>>,
    ) -> LightInteraction<f64> {
        LightInteraction {
            directly_emitted,
            scattered_rays,
        }
    }

    #[inline]
    pub fn directly_emitted(&self) -> Vec3<f64> {
        self.directly_emitted
    }

    #[inline]
    pub fn scattered_rays(&self) -> &Vec<ScatteredRay<f64>> {
        &self.scattered_rays
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ScatteredRay<T> {
    pub ray: Ray<T>,
    pub attenuation: Vec3<T>,
}

impl ScatteredRay<f64> {
    #[inline]
    pub fn new(ray: Ray<f64>, attenuation: Vec3<f64>) -> ScatteredRay<f64> {
        ScatteredRay { ray, attenuation }
    }

    #[inline]
    pub fn ray(self) -> Ray<f64> {
        self.ray
    }

    #[inline]
    pub fn attenuation(self) -> Vec3<f64> {
        self.attenuation
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Material<T> {
    Normal {},
    Lambertian { albedo: Vec3<T> },
    Metal { albedo: Vec3<T>, fuzz: T },
}

impl Material<f64> {
    pub fn interact(&self, ray: Ray<f64>, hit_normal: &Ray<f64>) -> LightInteraction<f64> {
        match self {
            Material::Normal {} => LightInteraction::new(
                hit_normal.direction().map(|i: f64| -> f64 {
                    (i + 1.0) /* *100.0*/
                    /* / 2.0*/
                }),
                vec![],
            ),
            Material::Lambertian { albedo } => interact_with_lambertian(hit_normal, albedo),
            Material::Metal { albedo, fuzz } => interact_with_metal(ray, hit_normal, albedo, fuzz),
        }
    }
}

fn interact_with_lambertian(hit_normal: &Ray<f64>, albedo: &Vec3<f64>) -> LightInteraction<f64> {
    let target = *hit_normal.origin() + *hit_normal.direction() + random_in_unit_sphere();
    let scattered_rays = vec![ScatteredRay::new(
        Ray::new(*hit_normal.origin(), target - *hit_normal.origin()),
        *albedo,
    )];
    LightInteraction::new(Vec3::new(0.0, 0.0, 0.0), scattered_rays)
}

fn interact_with_metal(
    ray: Ray<f64>,
    hit_normal: &Ray<f64>,
    albedo: &Vec3<f64>,
    fuzz: &f64,
) -> LightInteraction<f64> {
    let reflected = reflect(*ray.direction(), hit_normal.direction());

    let scattered_rays = vec![ScatteredRay::new(
        Ray::new(
            *hit_normal.origin(),
            reflected + *fuzz * random_in_unit_sphere(),
        ),
        *albedo,
    )]
    .iter()
    .filter(|scattered_ray| (*scattered_ray.ray().direction()).dot(*hit_normal.direction()) > 0.0)
    .map(|scattered_ray| *scattered_ray)
    .collect();
    LightInteraction::new(Vec3::new(0.0, 0.0, 0.0), scattered_rays)
}

fn reflect(inbound: Vec3<f64>, normal: &Vec3<f64>) -> Vec3<f64> {
    inbound - 2.0 * inbound.dot(*normal) * *normal
}

fn random_in_unit_sphere() -> Vec3<f64> {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut p: Vec3<f64>;
    loop {
        p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
            - Vec3::new(1f64, 1f64, 1f64);
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
}
