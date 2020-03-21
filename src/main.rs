mod vec3;

// use vec3;

fn main() {

    let nx: u32 = 200;
    let ny: u32 = 100;

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let color = vec3::Vec3::new((x as f32) / (nx as f32), (y as f32) / (ny as f32), 0.2);

        *pixel = image::Rgb(color.to_rgb());
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("image.png").unwrap();
}
