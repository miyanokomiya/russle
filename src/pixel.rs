use crate::math::*;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Pixel {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_u8(self) -> (u8, u8, u8, u8) {
        (
            self.r.round() as u8,
            self.g.round() as u8,
            self.b.round() as u8,
            self.a.round() as u8,
        )
    }

    pub fn combine(self, target: Self) -> Self {
        let alpha = target.a / 255.0;
        Self::new(
            lerp(self.r, target.r, alpha),
            lerp(self.g, target.g, alpha),
            lerp(self.b, target.b, alpha),
            self.a,
        )
    }
}

impl Add for Pixel {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: (self.r + other.r).min(255.0),
            g: (self.g + other.g).min(255.0),
            b: (self.b + other.b).min(255.0),
            a: (self.a + other.a).min(255.0),
        }
    }
}

impl Sub for Pixel {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            r: (self.r - other.r).max(0.0),
            g: (self.g - other.g).max(0.0),
            b: (self.b - other.b).max(0.0),
            a: (self.a - other.a).max(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine() {
        let v1 = Pixel::new(10.0, 20.0, 30.0, 40.0);
        let v2 = Pixel::new(110.0, 220.0, 330.0, 102.0);
        assert_eq!(Pixel::new(50.0, 100.0, 150.0, 40.0), v1.combine(v2));
    }

    #[test]
    fn test_add_normal() {
        let v1 = Pixel::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Pixel::new(3.0, 4.0, 5.0, 6.0);
        assert_eq!(Pixel::new(4.0, 6.0, 8.0, 10.0), v1 + v2);
    }

    #[test]
    fn test_add_overflow() {
        let v1 = Pixel::new(200.0, 200.0, 200.0, 200.0);
        let v2 = Pixel::new(100.0, 100.0, 100.0, 100.0);
        assert_eq!(Pixel::new(255.0, 255.0, 255.0, 255.0), v1 + v2);
    }

    #[test]
    fn test_sub_normal() {
        let v1 = Pixel::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Pixel::new(3.0, 4.0, 5.0, 6.0);
        assert_eq!(Pixel::new(2.0, 2.0, 2.0, 2.0), v2 - v1);
    }

    #[test]
    fn test_sub_overflow() {
        let v1 = Pixel::new(200.0, 200.0, 200.0, 200.0);
        let v2 = Pixel::new(100.0, 100.0, 100.0, 100.0);
        assert_eq!(Pixel::new(0.0, 0.0, 0.0, 0.0), v2 - v1);
    }
}
