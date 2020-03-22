mod math;
use math::*;


fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin.subtract_vec3(center);
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn color(ray: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0).multiply_scalar(1.0 - t)
            .add_vec3(&Vec3::new(0.5, 0.7, 1.0).multiply_scalar(t))
    }
}

fn main() {

    let nx: u32 = 200;
    let ny: u32 = 100;

    let nxf = nx as f32;
    let nyf = ny as f32;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    // Iterate over the coordinates and pixels of the image
    for (x, y_flip, pixel) in imgbuf.enumerate_pixels_mut() {

        // The y value runs from top to bottom so flip it.
        let y = ny - y_flip;

        let u = (x as f32) / nxf;
        let v = (y as f32) / nyf;

        // Create a ray from the origin to each point
        let ray = Ray::new(origin, lower_left_corner.add_vec3(&horizontal.multiply_scalar(u).add_vec3(&vertical.multiply_scalar(v))));

        // Compute the color for the pixel
        let color = color(&ray);

        // Assign
        *pixel = image::Rgb(color.to_rgb());
    }

    // Save the image as “img.png”, the format is deduced from the extension
    imgbuf.save("image.png").unwrap();
}
