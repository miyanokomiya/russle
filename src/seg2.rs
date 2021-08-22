use crate::vec2::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Seg2 {
    pub p1: Vec2,
    pub p2: Vec2,
}

impl Seg2 {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Self { p1, p2 }
    }

    pub fn cross(self, target: Self) -> Option<Vec2> {
        let a = self.p1;
        let b = self.p2;
        let c = target.p1;
        let d = target.p2;

        let ca_x = c.x - a.x;
        let ca_y = c.y - a.y;
        let dc_x = d.x - c.x;
        let dc_y = d.y - c.y;
        let ba_x = b.x - a.x;
        let ba_y = b.y - a.y;

        let s_top = ca_x * dc_y - ca_y * dc_x;
        let s_bottom = ba_x * dc_y - ba_y * dc_x;
        let t_top = -ca_x * ba_y + ca_y * ba_x;
        let t_bottom = dc_x * ba_y - dc_y * ba_x;

        if s_bottom == 0.0 || t_bottom == 0.0 {
            return None;
        }

        let s = s_top / s_bottom;
        let t = t_top / t_bottom;

        if s < 0.0 || 1.0 < s || t < 0.0 || 1.0 < t {
            return None;
        }

        Some(Vec2::new(a.x + s * ba_x, a.y + t * ba_y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross() {
        let seg1 = Seg2::new(Vec2::new(0.0, 0.0), Vec2::new(5.0, 0.0));

        let seg2 = Seg2::new(Vec2::new(2.0, -2.0), Vec2::new(2.0, 2.0));
        assert_eq!(seg1.cross(seg2), Some(Vec2::new(2.0, 0.0)));

        let seg3 = Seg2::new(Vec2::new(20.0, -2.0), Vec2::new(20.0, 2.0));
        assert_eq!(seg1.cross(seg3), None);

        let seg4 = Seg2::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        let seg5 = Seg2::new(Vec2::new(10.0, 0.0), Vec2::new(0.0, 10.0));
        assert_eq!(seg4.cross(seg5), Some(Vec2::new(5.0, 5.0)));
    }
}
