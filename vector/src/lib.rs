use std::ops::{Add, Sub, Mul, Div};

pub const WIDTH: i32 = 600;
pub const HEIGHT: i32 = 600;
const MARGIN: i32 = 15;

// If a value cannot be converted into int, returns 0
fn float_to_int(f: f32) -> i32 {
    if f.is_nan() || f > i32::MAX as f32 || f < i32::MIN as f32 {
        0
    } else {
        unsafe { f.to_int_unchecked() }
    }
}

#[derive(Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(mut x: f32, mut y: f32) -> Self {
        while x > WIDTH as f32 / 2.0 + MARGIN as f32 {
            x -= WIDTH as f32 + MARGIN as f32 * 2.0;
        }
        while x < -WIDTH as f32 / 2.0 - MARGIN as f32 {
            x += WIDTH as f32 + MARGIN as f32 * 2.0;
        }
        while y > HEIGHT as f32 / 2.0 + MARGIN as f32 {
            y -= HEIGHT as f32 + MARGIN as f32 * 2.0;
        }
        while y < -WIDTH as f32 / 2.0 - MARGIN as f32 {
            y += WIDTH as f32 + MARGIN as f32 * 2.0;
        }

        Self { x, y }
    }

    pub fn to_canvas_coordinate(&self) -> (i32, i32) {
        (
            float_to_int(self.x) + WIDTH / 2,
            -float_to_int(self.y) + HEIGHT / 2,
        )
    }

    pub fn rotate(&self, rad: f32) -> Self {
        Self::new(
            self.x * rad.cos() - self.y * rad.sin(),
            self.x * rad.sin() + self.y * rad.cos(),
        )
    }
}

impl Add<Self> for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Self> for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<Self> for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Div<Self> for Vector {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector::new(self.x / rhs.x, self.y / rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overflow_test() {
        let a = Vector::new(300.0, 400.0);
        let b = Vector::new(100.0, 1100.0);

        assert_eq!(a, Vector::new(300.0, -230.0));
        assert_eq!(b, Vector::new(100.0, -160.0));
        assert_eq!(a + b, Vector::new(-230.0, 240.0));
    }
}
