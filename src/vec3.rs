use std::ops;
use std::convert;

#[derive(Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where T: Copy + ops::Add<Output=T> + ops::Sub<Output=T> + ops::Mul<Output=T> + ops::Neg<Output=T> + convert::Into<f64>//, Vec<T>: ToF64
{
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }

    #[inline]
    pub fn dot(self, other: Vec3<T>) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    #[inline]
    pub fn cross(self, other: Vec3<T>) -> Vec3<T> {
        Self {
            x: (self.y * other.z) - (self.z * other.y),
            y: -((self.x * other.z) - (self.z * other.x)),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }
    /*
    #[inline]
    pub fn unit(self) -> Vec3<f64> {
        let self_as_f64: Vec3<f64> = self.into_f64();
        self_as_f64 / self.length()
    }*/

    #[inline]
    pub fn length(self) -> f64 {
        let squared_length: f64 = self.squared_length().into();
        squared_length.sqrt()
    }

    #[inline]
    pub fn squared_length(self) -> T {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
}

pub trait ToF64 {
    fn into_f64(self) -> Vec3<f64>;
}


impl ToF64 for Vec3<f64> {
    fn into_f64(self) -> Vec3<f64> {
        self
    }
}

macro_rules! into_impl {
    ($($t:ty)*) => ($(
        impl ToF64 for Vec3<$t> {
            fn into_f64(self) -> Vec3<f64> {
                Vec3 {
                    x: self.x as f64,
                    y: self.y as f64,
                    z: self.z as f64,
                }
            }
        }

    )*)
}

into_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 }


impl<T: ops::Add<Output=T>> ops::Add for Vec3<T> {
    type Output = Self;

    #[inline]
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

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: ops::Mul<Output=T>> ops::Mul for Vec3<T> {
    type Output = Vec3<T>;

    #[inline]
    fn mul(self, other: Vec3<T>) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
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


impl<T: ops::Div<Output=T>> ops::Div for Vec3<T> {
    type Output = Vec3<T>;

    #[inline]
    fn div(self, other: Vec3<T>) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

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

impl<T: ops::Neg<Output=T>> ops::Neg for Vec3<T> {
    type Output = Vec3<T>;
    fn neg(self) -> Vec3<T> {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


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
        assert_eq!(Vec3::new(1, 0, -1) * 2,
                   Vec3::new(2, 0, -2));
        assert_eq!(2 * Vec3::new(1, 0, -1),
                   Vec3::new(2, 0, -2));

        assert_eq!(Vec3::new(1f64, 0f64, -1f64) * 2f64,
                   Vec3::new(2f64, 0f64, -2f64));
        assert_eq!(2f64 * Vec3::new(1f64, 0f64, -1f64),
                   Vec3::new(2f64, 0f64, -2f64));

        assert_eq!(Vec3::new(1, 0, -1) * Vec3::new(4, 0, -2),
                   Vec3::new(4, 0, 2));
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

        assert_eq!(Vec3::new(2, 4, -6) / Vec3::new(2, 2, -3),
                   Vec3::new(1, 2, 2));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(2, 0, -6),
                   Vec3::new(-2, 0, 6));

        assert_eq!(-Vec3::new(2f64, 0f64, -6f64),
                   Vec3::new(-2f64, 0f64, 6f64));
    }

    #[test]
    fn test_dot() {
        assert_eq!(Vec3::new(1, 2, 3).dot(Vec3::new(4, -5, 6)),
                   12);
    }

    #[test]
    fn test_cross() {
        assert_eq!(Vec3::new(3, -3, 1).cross(Vec3::new(4, 9, 2)),
                   Vec3::new(-15, -2, 39));
    }

    #[test]
    fn test_length() {
        assert_eq!(Vec3::new(2, -2, 1).length(), 3f64);
    }
}