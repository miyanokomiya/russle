use super::Canvas;

impl Canvas {
    pub fn draw_circle(&mut self, cx: f64, cy: f64, r: f64) -> &Self {
        self.draw_pixels(get_circle_opacity_fn(cx, cy, r))
    }
}

fn get_circle_opacity_fn(cx: f64, cy: f64, r: f64) -> impl Fn(usize, usize) -> f64 {
    let get_super_sampling_rate = get_super_sampling_rate_fn(cx, cy, r, 4);
    move |px_x, px_y| -> f64 { 255.0 * get_super_sampling_rate(px_x, px_y) }
}

fn get_super_sampling_rate_fn(
    cx: f64,
    cy: f64,
    r: f64,
    size: usize,
) -> impl Fn(usize, usize) -> f64 {
    let r2 = r * r;
    let size_f64 = size as f64;
    let unit_size = 1.0 / size_f64;
    let diff = -0.5 + unit_size / 2.0;

    move |px_x, px_y| -> f64 {
        let mut hit_count = 0;

        for sub_y in 0..size {
            let dy = cy - (px_y as f64 + diff + sub_y as f64 * unit_size);
            let dy2 = dy * dy;

            for sub_x in 0..size {
                let dx = cx - (px_x as f64 + diff + sub_x as f64 * unit_size);
                let dx2 = dx * dx;

                hit_count = if dx2 + dy2 <= r2 {
                    hit_count + 1
                } else {
                    hit_count
                };
            }
        }

        hit_count as f64 / size_f64 / size_f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_circle_opacity_fn() {
        let target = get_circle_opacity_fn(5.0, 5.0, 1.0);
        assert_eq!(target(3, 5), 0.0);
        assert!({
            let a = target(4, 5);
            0.0 < a && a < 255.0
        });
        assert_eq!(target(5, 5), 255.0);
        assert_eq!(target(5, 3), 0.0);
        assert!({
            let a = target(5, 4);
            0.0 < a && a < 255.0
        });
    }

    #[test]
    fn test_get_super_sampling_rate_fn_1() {
        let target = get_super_sampling_rate_fn(5.0, 5.0, 0.8, 1);
        assert_eq!(target(4, 5), 0.0);
        assert_eq!(target(5, 5), 1.0);
        assert_eq!(target(6, 5), 0.0);

        assert_eq!(target(5, 4), 0.0);
        assert_eq!(target(5, 5), 1.0);
        assert_eq!(target(5, 6), 0.0);
    }

    #[test]
    fn test_get_super_sampling_rate_fn_3() {
        let target = get_super_sampling_rate_fn(5.0, 5.0, 0.8, 3);
        assert_eq!(target(4, 5), 1.0 / 3.0);
        assert_eq!(target(5, 5), 1.0);
        assert_eq!(target(6, 5), 1.0 / 3.0);

        assert_eq!(target(5, 4), 1.0 / 3.0);
        assert_eq!(target(5, 5), 1.0);
        assert_eq!(target(5, 6), 1.0 / 3.0);
    }
}
