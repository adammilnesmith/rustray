use std::ops;

#[derive(Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
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



macro_rules! div_impl {
    ($($t:ty)*) => ($(
        impl ops::Div<$t> for Vec3<$t> {
            type Output = Vec3<$t>;

            #[inline]
            fn div(self, other: $t) -> Self::Output {
                Self {
                    x: self.x / other,
                    y: self.y / other,
                    z: self.z / other,
                }
            }
        }

        impl ops::Div<Vec3<$t>> for $t {
            type Output = Vec3<$t>;

            #[inline]
            fn div(self, other: Vec3<$t>) -> Self::Output {
                Vec3 {
                    x: self / other.x,
                    y: self / other.y,
                    z: self / other.z,
                }
            }
        }

    )*)
}

div_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Vec3::new(1f64, 0f64, -1f64) + Vec3::new(2f64, 3f64, 2f64),
                   Vec3::new(3f64, 3f64, 1f64));

        assert_eq!(Vec3::new(1, 0, -1) + Vec3::new(2, 3, 2),
                   Vec3::new(3, 3, 1));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Vec3::new(1, 0, -1) - Vec3::new(2, 3, 2),
                   Vec3::new(-1, -3, -3));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3::new(1i32, 0i32, -1i32) * 2,
                   Vec3::new(2, 0, -2));
        assert_eq!(2 * Vec3::new(1i32, 0i32, -1i32),
                   Vec3::new(2, 0, -2));

        assert_eq!(Vec3::new(1f64, 0f64, -1f64) * 2f64,
                   Vec3::new(2f64, 0f64, -2f64));
        assert_eq!(2f64 * Vec3::new(1f64, 0f64, -1f64),
                   Vec3::new(2f64, 0f64, -2f64));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(2, 4, -6) / 2,
                   Vec3::new(1, 2, -3));
        assert_eq!(6 / Vec3::new(1, 2, -3),
                   Vec3::new(6, 3, -2));

        assert_eq!(Vec3::new(2f64, 4f64, -6f64) / 2f64,
                   Vec3::new(1f64, 2f64, -3f64));
        assert_eq!(6f64 / Vec3::new(1f64, 2f64, -3f64),
                   Vec3::new(6f64, 3f64, -2f64));
    }
}