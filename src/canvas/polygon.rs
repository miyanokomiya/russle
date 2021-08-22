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

    let polygon_segs = get_polygon_segs(polygon);

    let cross_list_row: Vec<Vec<Vec2>> = ((min_y.floor() as usize)..(max_y.ceil()) as usize)
        .map(|px_y| {
            let px_y_f = px_y as f64;
            let x_seg = Seg2::new(Vec2::new(min_x, px_y_f), Vec2::new(max_x, px_y_f));
            let mut cross_list: Vec<Vec2> = polygon_segs
                .iter()
                .filter_map(|seg| seg.cross(x_seg, true, false))
                .collect();

            if cross_list.len() == 1 {
                return vec![];
            }

            cross_list.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
            cross_list
        })
        .collect();

    move |px_x, px_y| {
        let px_y_f = px_y as f64;
        if px_y_f < min_y || max_y <= px_y_f {
            return 0.0;
        }

        let cross_list = &cross_list_row[px_y - min_y as usize];
        get_polygon_opacity_by_x(cross_list, px_x)
    }
}

fn get_polygon_segs(polygon: Vec<Vec2>) -> Vec<Seg2> {
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
