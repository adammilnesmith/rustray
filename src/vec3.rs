use std::ops;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
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
    }
}