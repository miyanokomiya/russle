extern crate image;

mod canvas;
mod math;
mod pixel;
mod vec2;

use canvas::Canvas;

fn main() {
    let mut canvas = Canvas::new(10, 40, 20);
    let (imgx, imgy) = canvas.get_image_size();

    // canvas.draw_rect(2.0, 2.0, 2.0, 4.0);
    // canvas.draw_rect(10.4, 5.4, 4.8, 2.8);
    // canvas.draw_rect(20.9, 5.0, 4.1, 2.3);

    // canvas.draw_circle(20.0, 10.0, 5.5);
    // canvas.draw_circle(3.0, 3.0, 1.0);
    // canvas.draw_circle(6.0, 3.0, 0.5);
    // canvas.draw_circle(6.0, 6.0, 0.8);

    canvas.draw_line(3.0, 3.0, 10.0, 3.0);
    canvas.draw_line(3.0, 5.0, 3.0, 12.0);
    canvas.stroke_width = 3.0;
    canvas.draw_line(7.0, 7.0, 12.0, 10.0);
    canvas.draw_line(15.0, 5.0, 20.0, 15.0);

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    for y in 0..imgy {
        for x in 0..imgx {
            let pixel = imgbuf.get_pixel_mut(x, y);
            let (r, g, b, a) = canvas.pixels[(y / canvas.px_size as u32) as usize]
                [(x / canvas.px_size as u32) as usize]
                .to_u8();
            *pixel = image::Rgba([r, g, b, a]);
        }
    }

    imgbuf.save("tmp/test.png").unwrap();
}
