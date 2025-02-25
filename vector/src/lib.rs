use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::default::Default;
use serde::{Serialize, Deserialize};

pub const WIDTH: i32 = 600;
pub const HEIGHT: i32 = 600;
const MARGIN: i32 = 15;

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(mut x: f32, mut y: f32) -> Self {
        while x > WIDTH as f32 + MARGIN as f32 {
            x -= WIDTH as f32 + MARGIN as f32 * 2.0;
        }
        while x < -MARGIN as f32 {
            x += WIDTH as f32 + MARGIN as f32 * 2.0;
        }
        while y > HEIGHT as f32 + MARGIN as f32 {
            y -= HEIGHT as f32 + MARGIN as f32 * 2.0;
        }
        while y < -MARGIN as f32 {
            y += WIDTH as f32 + MARGIN as f32 * 2.0;
        }

        Self { x, y }
    }

    pub fn rotate(&self, rad: f32) -> Self {
        Self::new(
            self.x * rad.cos() - self.y * rad.sin(),
            self.x * rad.sin() + self.y * rad.cos(),
        )
    }

    pub fn dist2(a: Vector, b: Vector) -> f32 {
        (a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)
    }
}

impl Add<Self> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Self> for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vector::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl Sub<Self> for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign<Self> for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vector::new(self.x - rhs.x, self.y - rhs.y);
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vector::new(self.x * rhs, self.y * rhs);
    }
}

impl Div<f32> for Vector {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, rhs: f32) {
        *self = Vector::new(self.x / rhs, self.y / rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overflow_test() {
        let a = Vector::new(300.0, 400.0);
        let b = Vector::new(100.0, 1100.0);

        assert_eq!(a, Vector::new(300.0, 400.0));
        assert_eq!(b, Vector::new(100.0, 470.0));
        assert_eq!(a + b, Vector::new(400.0, 240.0));
    }

    #[test]
    fn dist_test() {
        let mut before = Vector::new(0.0, 0.0);
        let mut after = Vector::new(0.0, 0.0);
        let diff = Vector::new(100.0, 0.0);

        after += diff;
        for _ in 0..6 {
            assert!(Vector::dist2(before, after) <= 10001.0);
            before = after;
            after += diff;
        }
        assert!(Vector::dist2(before, after) >= 10001.0);
    }

    #[test]
    fn serialize_test() {
        let v = Vector::new(123.0, 456.0);

        assert_eq!(serde_json::to_string(&v).unwrap(), "{\"x\":123.0,\"y\":456.0}");
    }
}
