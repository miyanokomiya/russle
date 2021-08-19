use super::Canvas;
use crate::pixel::Pixel;
use crate::vec2::Vec2;

impl Canvas {
    pub fn draw_rect(&mut self, x: f64, y: f64, width: f64, height: f64) -> &Self {
        for (pixel_y, row) in self.pixels.iter_mut().enumerate() {
            for (pixel_x, pixel) in row.iter_mut().enumerate() {
                let a = get_rect_opacity(
                    Vec2::new(pixel_x as f64, pixel_y as f64),
                    x,
                    y,
                    width,
                    height,
                );
                if a > 0 {
                    *pixel = Pixel::new(pixel.r, 255, pixel.b, a);
                }
            }
        }
        self
    }
}

fn get_rect_opacity(vec: Vec2, x: f64, y: f64, width: f64, height: f64) -> u8 {
    let dx = vec.x - x;
    if dx <= -1.0 || width + 1.0 <= dx {
        return 0;
    }
    let dy = vec.y - y;
    if dy <= -1.0 || height + 1.0 <= dy {
        return 0;
    }

    let small_x = dx - dx.round();
    let rate_x = if dx < 0.0 || width < dx {
        small_x.abs()
    } else {
        1.0
    };

    let small_y = dy - dy.round();
    let rate_y = if dy < 0.0 || height < dy {
        small_y.abs()
    } else {
        1.0
    };

    // (rate_x * rate_y * 255.0).round() as u8
    // rate_x.max(rate_y) as u8
    ((rate_x + rate_y) / 2.0 * 255.0) as u8
}
