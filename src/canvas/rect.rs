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
                    *pixel = pixel.combine(Pixel::new(0, 255, 0, a));
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

    let rate_x = if dx < 0.0 {
        dx.round() - dx
    } else if width < dx {
        1.0 + width - dx
    } else {
        1.0
    };

    let rate_y = if dy < 0.0 {
        dy.round() - dy
    } else if height < dy {
        1.0 + height - dy
    } else {
        1.0
    };

    (rate_x * rate_y * 255.0).round() as u8
}
