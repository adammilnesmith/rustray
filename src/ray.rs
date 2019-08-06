use vec3::{NumWithVectorOps, Vec3, VectorWithOps};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
}

impl<T> Ray<T>
where
    T: Copy + NumWithVectorOps,
    Vec3<T>: VectorWithOps<T>,
{
    pub fn new(origin: Vec3<T>, direction: Vec3<T>) -> Ray<T> {
        Ray { origin, direction }
    }

    #[inline]
    pub fn point_at_parameter(self, t: T) -> Vec3<T> {
        let direction_with_magnitude: Vec3<T> = self.direction * t;
        self.origin + direction_with_magnitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_at_parameter() {
        assert_eq!(
            Ray::new(Vec3::new(1f64, 2f64, 3f64), Vec3::new(2f64, 3f64, 4f64))
                .point_at_parameter(2f64),
            Vec3::new(5f64, 8f64, 11f64)
        );
    }
}
