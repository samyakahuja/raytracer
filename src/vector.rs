use std::ops::{Add, Div, Mul, Neg, Sub};

/// f64 does not implement Eq so neither does Vec3
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.dot(&self))
    }

    pub fn unit_vector(v: Self) -> Self {
        v / v.length()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

// vec3 * scalar
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// scalar * vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn v3_length() {
        let v = Vec3::new(0.0, 3.0, 4.0);

        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn v3_negate() {
        let v1 = Vec3::new(0.0, 3.0, 4.0);
        let v2 = Vec3::new(-0.0, -3.0, -4.0);

        assert_eq!(-v1, v2);
    }

    #[test]
    fn v3_add() {
        let v1 = Vec3::new(0.0, 3.0, 3.8);
        let v2 = Vec3::new(4.0, 1.0, 0.2);
        let v3 = Vec3::new(4.0, 4.0, 4.0);

        assert_eq!(v1 + v2, v3);
    }

    #[test]
    fn v3_sub() {
        let v1 = Vec3::new(0.0, 3.0, 3.0);
        let v2 = Vec3::new(4.0, 1.0, 3.0);
        let v3 = Vec3::new(-4.0, 2.0, 0.0);

        assert_eq!(v1 - v2, v3);
    }

    #[test]
    fn v3_mul() {
        let v1 = Vec3::new(0.0, 3.0, 2.0);
        let v2 = Vec3::new(4.0, 1.0, 3.0);
        let s1 = 3.0;

        assert_eq!(v1 * v2, Vec3::new(0.0, 3.0, 6.0));
        assert_eq!(v1 * s1, Vec3::new(0.0, 9.0, 6.0));
        assert_eq!(s1 * v1, Vec3::new(0.0, 9.0, 6.0));
    }

    #[test]
    fn v3_div() {
        let v1 = Vec3::new(0.0, 3.0, 2.0);
        let v2 = Vec3::new(4.0, 1.0, 4.0);
        let s1 = 2.0;

        assert_eq!(v1 / v2, Vec3::new(0.0, 3.0, 0.5));
        assert_eq!(v2 / s1, Vec3::new(2.0, 0.5, 2.0));
    }
}
