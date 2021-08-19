extern crate image;

mod canvas;
mod pixel;
mod vec2;

use canvas::Canvas;

fn main() {
    let mut canvas = Canvas::new(10, 40, 20);
    let (imgx, imgy) = canvas.get_image_size();

    // canvas.draw_rect(2.0, 2.0, 2.0, 4.0);
    // canvas.draw_rect(10.0, 10.0, 4.0, 2.0);
    canvas.draw_rect(10.4, 5.4, 4.8, 2.8);

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    for x in 0..imgx {
        for y in 0..imgy {
            let pixel = imgbuf.get_pixel_mut(x, y);
            let px = canvas.pixels[(x / canvas.px_size as u32) as usize]
                [(y / canvas.px_size as u32) as usize];
            *pixel = image::Rgba([px.r, px.g, px.b, px.a]);
        }
    }

    imgbuf.save("tmp/test.png").unwrap();
}
