use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

impl Add for Pixel {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
            a: self.a.saturating_add(other.a),
        }
    }
}

impl Sub for Pixel {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r.saturating_sub(other.r),
            g: self.g.saturating_sub(other.g),
            b: self.b.saturating_sub(other.b),
            a: self.a.saturating_sub(other.a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_normal() {
        let v1 = Pixel::new(1, 2, 3, 4);
        let v2 = Pixel::new(3, 4, 5, 6);
        assert_eq!(Pixel::new(4, 6, 8, 10), v1 + v2);
    }

    #[test]
    fn test_add_overflow() {
        let v1 = Pixel::new(200, 200, 200, 200);
        let v2 = Pixel::new(100, 100, 100, 100);
        assert_eq!(Pixel::new(255, 255, 255, 255), v1 + v2);
    }

    #[test]
    fn test_sub_normal() {
        let v1 = Pixel::new(1, 2, 3, 4);
        let v2 = Pixel::new(3, 4, 5, 6);
        assert_eq!(Pixel::new(2, 2, 2, 2), v2 - v1);
    }

    #[test]
    fn test_sub_overflow() {
        let v1 = Pixel::new(200, 200, 200, 200);
        let v2 = Pixel::new(100, 100, 100, 100);
        assert_eq!(Pixel::new(0, 0, 0, 0), v2 - v1);
    }
}
