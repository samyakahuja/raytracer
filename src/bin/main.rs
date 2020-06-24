use raytracer::vector::Vec3;
use raytracer::ray::Ray;

fn write_color(color: Vec3) {
    let color = 255.999 * color;
    println!(
        "{} {} {}",
        color.x as i32,
        color.y as i32,
        color.z as i32,
    );
}

fn ray_color(ray: &Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(ray.direction);
    // 0.0 <= t <= 1.0
    let t = 0.5 * (unit_direction.y + 1.0);
    // linear interpolation between white and some bluish color
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: i32 = 384;
    const HEIGHT: i32 = (WIDTH as f64 / ASPECT_RATIO as f64) as i32;

    // PP3 file header
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    // viewport has same aspect ratio as the image.
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    // distance between the projection point and the projection plane
    let focal_length: f64 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for j in (0..HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..WIDTH {
            let u = i as f64 / (WIDTH - 1) as f64;
            let v = j as f64 / (HEIGHT - 1) as f64;
            // start traversing at lower_left_corner
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let col = ray_color(&ray);
            write_color(col);
        }
    }
    eprintln!("Done.");
}
