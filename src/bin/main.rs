use raytracer::vector::Vec3;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::hittable::{Hittable, HittableList};
use raytracer::camera::Camera;

use rand::Rng;

fn write_color(color: Vec3, samples: u32) {
    let color = color / samples as f64;
    println!(
        "{} {} {}",
        (255.999 * clamp(color.x, 0.0, 0.99)) as i32,
        (255.999 * clamp(color.y, 0.0, 0.99)) as i32,
        (255.999 * clamp(color.z, 0.0, 0.99)) as i32,
    );
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min } else if x > max { max } else { x }
}

fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.0, f64::MAX) {
        // map the normal to be between 0 and 1 and use it as color at that point
        0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = Vec3::unit_vector(ray.direction);
        // 0.0 <= t <= 1.0
        let t = 0.5 * (unit_direction.y + 1.0);
        // linear interpolation between white and some bluish color
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = (WIDTH as f64 / ASPECT_RATIO as f64) as i32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    // PP3 file header
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);
    let cam = Camera::new();

    let mut rng = rand::thread_rng();

    for j in (0..HEIGHT).rev() {
        if j % 10 == 0 { eprintln!("\rScanlines remaining: {}", j); }
        for i in 0..WIDTH {
            let mut pixel_color = Vec3::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (HEIGHT - 1) as f64;
                let ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done.");
}
