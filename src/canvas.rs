use crate::pixel::Pixel;

pub mod rect;

#[derive(Clone, Debug, PartialEq)]
pub struct Canvas {
    pub px_size: usize,
    pub x_size: usize,
    pub y_size: usize,
    pub pixels: Vec<Vec<Pixel>>,
}

impl Canvas {
    pub fn new(px_size: usize, x_size: usize, y_size: usize) -> Self {
        let mut pixels: Vec<Vec<Pixel>> = vec![];
        for _ in 0..x_size {
            let mut row: Vec<Pixel> = vec![];
            for _ in 0..y_size {
                row.push(Pixel::new(0, 0, 0, 255));
            }
            pixels.push(row);
        }

        Self {
            px_size: px_size,
            x_size: x_size,
            y_size: y_size,
            pixels: pixels,
        }
    }

    pub fn get_image_size(&self) -> (u32, u32) {
        get_image_size(self.px_size, self.x_size, self.y_size)
    }
}

fn get_image_size(px_size: usize, x_size: usize, y_size: usize) -> (u32, u32) {
    ((px_size * x_size) as u32, (px_size * y_size) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let canvas = Canvas::new(10, 8, 6);
        assert_eq!(canvas.pixels.len(), 8);
        assert_eq!(canvas.pixels[0].len(), 6);
    }

    #[test]
    fn test_get_image_size() {
        let canvas = Canvas::new(10, 8, 6);
        assert_eq!(canvas.get_image_size(), (80, 60));
    }
}
