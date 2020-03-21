mod math;
use math::*;

fn main() {

    let nx: u32 = 200;
    let ny: u32 = 100;

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    // Iterate over the coordinates and pixels of the image
    for (x, y_flip, pixel) in imgbuf.enumerate_pixels_mut() {

        // The y value runs from top to bottom so flip it.
        let y = ny - y_flip;

        let color = Vec3::new((x as f32) / (nx as f32), (y as f32) / (ny as f32), 0.2);

        *pixel = image::Rgb(color.to_rgb());
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("image.png").unwrap();
}
