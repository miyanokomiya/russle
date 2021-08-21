use super::Canvas;

impl Canvas {
    pub fn draw_rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> &Self {
        self.draw_pixels(get_rect_opacity_fn(x, y, width, height))
    }
}

fn get_rect_opacity_fn(x: f64, y: f64, width: f64, height: f64) -> impl Fn(usize, usize) -> f64 {
    let right = x + width;
    let bottom = y + height;

    move |px_x, px_y| -> f64 {
        let px_x_f = px_x as f64;
        let px_y_f = px_y as f64;

        if px_x_f <= x - 1.0 || right + 1.0 <= px_x_f {
            return 0.0;
        }
        if px_y_f <= y - 1.0 || bottom + 1.0 <= px_y_f {
            return 0.0;
        }

        let rate_x = if px_x_f < x {
            1.0 + px_x_f - x
        } else if right < px_x_f {
            1.0 + right - px_x_f
        } else {
            1.0
        };

        let rate_y = if px_y_f < y {
            1.0 + px_y_f - y
        } else if bottom < px_y_f {
            1.0 + bottom - px_y_f
        } else {
            1.0
        };

        rate_x * rate_y * 255.0
    }
}
