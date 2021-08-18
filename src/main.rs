extern crate image;

fn main() {
    let px_size = 10;
    let px_x: usize = 30;
    let px_y: usize = 20;
    let imgx = (px_size * px_x) as u32;
    let imgy = (px_size * px_y) as u32;

    let mut matrix: Vec<Vec<Pixel>> = vec![];
    for x in 0..px_x {
        let mut row: Vec<Pixel> = vec![];
        for y in 0..px_y {
            row.push(Pixel {
                r: 0,
                g: if (x + y) % 2 == 0 { 0 } else { 255 },
                b: 0,
                a: 255,
            });
        }
        matrix.push(row);
    }

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let pixel = imgbuf.get_pixel_mut(x, y);
            let px = &matrix[(x / px_size as u32) as usize][(y / px_size as u32) as usize];
            *pixel = image::Rgba([px.r, px.g, px.b, px.a]);
        }
    }

    imgbuf.save("tmp/test.png").unwrap();
}

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
