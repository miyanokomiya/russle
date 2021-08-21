use crate::pixel::Pixel;

pub mod circle;
pub mod line;
pub mod rect;

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    pub px_size: usize,
    pub x_size: usize,
    pub y_size: usize,
    pub pixels: Vec<Vec<Pixel>>,
    pub fill_color: Pixel,
    pub stroke_color: Pixel,
    pub stroke_width: f64,
}

impl Canvas {
    pub fn new(px_size: usize, x_size: usize, y_size: usize) -> Self {
        let mut pixels: Vec<Vec<Pixel>> = vec![];
        for y in 0..y_size {
            let mut row: Vec<Pixel> = vec![];
            for x in 0..x_size {
                let val = if (x + y) % 2 == 0 { 255.0 } else { 150.0 };
                row.push(Pixel::new(val, val, val, 255.0));
            }
            pixels.push(row);
        }

        Self {
            px_size: px_size,
            x_size: x_size,
            y_size: y_size,
            pixels: pixels,
            fill_color: Pixel::new(0.0, 255.0, 0.0, 255.0),
            stroke_color: Pixel::new(0.0, 0.0, 255.0, 255.0),
            stroke_width: 1.0,
        }
    }

    pub fn get_image_size(&self) -> (u32, u32) {
        get_image_size(self.px_size, self.x_size, self.y_size)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) -> &Self {
        if y < self.pixels.len() && x < self.pixels[y].len() {
            self.pixels[y][x] = pixel;
        }
        self
    }

    pub fn draw_pixels(&mut self, get_opacity: impl Fn(usize, usize) -> f64) -> &Self {
        for (pixel_y, row) in self.pixels.iter_mut().enumerate() {
            for (pixel_x, pixel) in row.iter_mut().enumerate() {
                let a = get_opacity(pixel_x, pixel_y);
                if a > 0.0 {
                    let mut px = self.fill_color;
                    px.a = a;
                    *pixel = pixel.combine(px);
                }
            }
        }
        self
    }

    pub fn stroke_pixels(&mut self, get_opacity: impl Fn(usize, usize) -> f64) -> &Self {
        for (pixel_y, row) in self.pixels.iter_mut().enumerate() {
            for (pixel_x, pixel) in row.iter_mut().enumerate() {
                let a = get_opacity(pixel_x, pixel_y);
                if a > 0.0 {
                    let mut px = self.stroke_color;
                    px.a = a;
                    *pixel = pixel.combine(px);
                }
            }
        }
        self
    }
}

fn get_image_size(px_size: usize, x_size: usize, y_size: usize) -> (u32, u32) {
    ((px_size * x_size) as u32, (px_size * y_size) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixel::Pixel;

    #[test]
    fn test_new() {
        let canvas = Canvas::new(10, 8, 6);
        assert_eq!(canvas.pixels.len(), 6);
        assert_eq!(canvas.pixels[0].len(), 8);
    }

    #[test]
    fn test_get_image_size() {
        let canvas = Canvas::new(10, 8, 6);
        assert_eq!(canvas.get_image_size(), (80, 60));
    }

    #[test]
    fn test_set_pixel() {
        let mut canvas = Canvas::new(10, 3, 3);
        canvas.set_pixel(1, 2, Pixel::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(canvas.pixels[2][1], Pixel::new(1.0, 2.0, 3.0, 4.0));

        // should ignore if the pair of indexes is out of range
        canvas.set_pixel(4, 2, Pixel::new(1.0, 2.0, 3.0, 4.0));
        canvas.set_pixel(2, 4, Pixel::new(1.0, 2.0, 3.0, 4.0));
    }
}
