use std::ops;

#[derive(Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: ops::Add<Output=T>> ops::Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: ops::Sub<Output=T>> ops::Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


macro_rules! mul_impl {
    ($($t:ty)*) => ($(
        impl ops::Mul<$t> for Vec3<$t> {
            type Output = Vec3<$t>;

            #[inline]
            fn mul(self, other: $t) -> Self::Output {
                Self {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                }
            }
        }
        impl ops::Mul<Vec3<$t>> for $t {
            type Output = Vec3<$t>;

            #[inline]
            fn mul(self, other: Vec3<$t>) -> Self::Output {
                other * self
            }
        }

    )*)
}

mul_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Vec3 { x: 1f64, y: 0f64, z: -1f64 }
                       + Vec3 { x: 2f64, y: 3f64, z: 2f64 },
                   Vec3 { x: 3f64, y: 3f64, z: 1f64 });

        assert_eq!(Vec3 { x: 1, y: 0, z: -1 }
                       + Vec3 { x: 2, y: 3, z: 2 },
                   Vec3 { x: 3, y: 3, z: 1 });
    }

    #[test]
    fn test_sub() {
        assert_eq!(Vec3 { x: 1, y: 0, z: -1 }
                       - Vec3 { x: 2, y: 3, z: 2 },
                   Vec3 { x: -1, y: -3, z: -3 });
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3 { x: 1, y: 0, z: -1 }
                       * 2,
                   Vec3 { x: 2, y: 0, z: -2 });
        assert_eq!(Vec3 { x: 1f64, y: 0f64, z: -1f64 }
                       * 2f64,
                   Vec3 { x: 2f64, y: 0f64, z: -2f64 });
    }
}