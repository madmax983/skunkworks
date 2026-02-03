use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A simple 2D vector for physics calculations.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Self::zero()
        } else {
            *self / mag
        }
    }

    pub fn limit(&self, max: f64) -> Self {
        if self.magnitude_squared() > max * max {
            self.normalize() * max
        } else {
            *self
        }
    }

    pub fn distance_squared(&self, other: Vec2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn distance(&self, other: Vec2) -> f64 {
        self.distance_squared(other).sqrt()
    }

    pub fn dot(&self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn reflect(&self, normal: Vec2) -> Self {
        let n = normal.normalize();
        let d = *self;
        let dot = d.dot(n);
        // r = d - 2(d . n)n
        Self {
            x: d.x - 2.0 * dot * n.x,
            y: d.y - 2.0 * dot * n.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y)
    }
}

impl From<Vec2> for (f64, f64) {
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_ops() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
        assert_eq!(v2 - v1, Vec2::new(2.0, 2.0));
        assert_eq!(v1 * 2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn test_vec2_reflect() {
        let v = Vec2::new(1.0, 0.0);
        let n = Vec2::new(-1.0, 0.0);
        let r = v.reflect(n);
        assert!((r.x - -1.0).abs() < 1e-6);
        assert!((r.y - 0.0).abs() < 1e-6);
    }
}
