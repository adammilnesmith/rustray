use std::ops;

#[derive(Debug, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl <T: ops::Add<Output = T>> ops::Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl <T: ops::Sub<Output = T>> ops::Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


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
}