use super::Canvas;

impl Canvas {
    pub fn draw_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) -> &Self {
        self.draw_pixels(get_line_opacity_fn(x1, y1, x2, y2))
    }
}

fn get_line_opacity_fn(x1: f64, y1: f64, x2: f64, y2: f64) -> impl Fn(usize, usize) -> f64 {
    let min_x = x1.min(x2);
    let max_x = x1.max(x2);
    let min_y = y1.min(y2);
    let max_y = y1.max(y2);
    let dx = x2 - x1;
    let dy = y2 - y1;

    move |px_x, px_y| -> f64 {
        let px_x_f = px_x as f64;
        let px_y_f = px_y as f64;

        if px_x_f < min_x || max_x < px_x_f || px_y_f < min_y || max_y < px_y_f {
            return 0.0;
        }

        if dx <= dy {
            let beta = dx / dy;
            let x = x1 + beta * (px_y_f - y1);
            let d = (x - px_x_f).abs();

            if d > 1.0 {
                0.0
            } else {
                (1.0 - d) * 255.0
            }
        } else {
            let beta = dy / dx;
            let y = y1 + beta * (px_x_f - x1);
            let d = (y - px_y_f).abs();

            if d > 1.0 {
                0.0
            } else {
                (1.0 - d) * 255.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_line_opacity_fn() {
        let target = get_line_opacity_fn(0.0, 0.0, 10.0, 5.0);
        assert_eq!(target(0, 0), 255.0);
        assert_eq!(target(1, 0), 255.0 / 2.0);
        assert_eq!(target(1, 1), 255.0 / 2.0);
        assert_eq!(target(9, 4), 255.0 / 2.0);
        assert_eq!(target(9, 5), 255.0 / 2.0);
        assert_eq!(target(10, 5), 255.0);

        let target = get_line_opacity_fn(0.0, 0.0, 5.0, 10.0);
        assert_eq!(target(0, 0), 255.0);
        assert_eq!(target(0, 1), 255.0 / 2.0);
        assert_eq!(target(1, 1), 255.0 / 2.0);
        assert_eq!(target(4, 9), 255.0 / 2.0);
        assert_eq!(target(5, 9), 255.0 / 2.0);
        assert_eq!(target(5, 10), 255.0);
    }
}
