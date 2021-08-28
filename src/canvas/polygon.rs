use super::Canvas;
use crate::seg2::Seg2;
use crate::vec2::Vec2;

impl Canvas {
    pub fn draw_polygon(&mut self, polygon: Vec<Vec2>) -> &Self {
        self.draw_pixels(get_polygon_opacity_fn(polygon))
    }
}

fn get_polygon_opacity_fn(polygon: Vec<Vec2>) -> impl Fn(usize, usize) -> f64 {
    let min_x = polygon.iter().fold(0.0 / 0.0, |m, v| v.x.min(m));
    let max_x = polygon.iter().fold(0.0 / 0.0, |m, v| v.x.max(m));
    let min_y = polygon.iter().fold(0.0 / 0.0, |m, v| v.y.min(m));
    let max_y = polygon.iter().fold(0.0 / 0.0, |m, v| v.y.max(m));

    let polygon_segs = get_polygon_segs(&polygon);

    let cross_list_row: Vec<Vec<Vec2>> = ((min_y.floor() as usize)
        ..((max_y + 1.0).floor()) as usize)
        .map(|px_y| get_cross_list_for_y(&polygon_segs, min_x, max_x, px_y as f64))
        .collect();

    move |px_x, px_y| {
        let px_y_f = px_y as f64;
        if px_y_f < min_y || max_y < px_y_f {
            return 0.0;
        }

        let cross_list = &cross_list_row[px_y - min_y as usize];
        get_polygon_opacity_by_x(cross_list, px_x)
    }
}

fn get_cross_list_for_y(
    polygon_segs: &Vec<Seg2>,
    min_x: f64,
    max_x: f64,
    px_y_f: f64,
) -> Vec<Vec2> {
    if polygon_segs.len() == 0 {
        return vec![];
    }

    let x_seg = Seg2::new(Vec2::new(min_x, px_y_f), Vec2::new(max_x, px_y_f));
    let mut cross_list: Vec<Vec2> = vec![];

    for seg in polygon_segs.iter() {
        // ignore horizontal segs
        if seg.p1.y == seg.p2.y {
            continue;
        }

        let to_up = seg.p2.y - seg.p1.y > 0.0;
        if let Some(cross) = seg.cross(x_seg, to_up, !to_up) {
            cross_list.push(cross);
        }
    }

    cross_list.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    cross_list
}

fn get_polygon_segs(polygon: &Vec<Vec2>) -> Vec<Seg2> {
    (0..polygon.len())
        .map(|i| Seg2::new(polygon[i], polygon[(i + 1) % polygon.len()]))
        .collect()
}

fn get_polygon_opacity_by_x(cross_list: &Vec<Vec2>, px_x: usize) -> f64 {
    let px_x_f = px_x as f64;
    let left_cross_list: Vec<&Vec2> = cross_list.iter().filter(|&c| c.x <= px_x_f).collect();
    if left_cross_list.len() % 2 == 0 {
        0.0
    } else {
        255.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec2::Vec2;

    #[test]
    fn test_get_polygon_opacity_fn() {
        let f = get_polygon_opacity_fn(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(5.0, 0.0),
            Vec2::new(5.0, 5.0),
            Vec2::new(0.0, 5.0),
        ]);
        assert_eq!(f(0, 0), 255.0);
        assert_eq!(f(4, 0), 255.0);
        assert_eq!(f(5, 0), 0.0);
        assert_eq!(f(0, 4), 255.0);
        assert_eq!(f(0, 5), 0.0);
    }
}
