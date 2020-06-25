use raytracer::vector::Vec3;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::hittable::{Hittable, HittableList};
use raytracer::camera::Camera;

use rand::Rng;

fn write_color(color: Vec3, samples: u32) {
    let color = color / samples as f64;
    // gamma corrected and clamped to between 0.0 and 0.99
    let r = clamp(f64::sqrt(color.x), 0.0, 0.99);
    let g = clamp(f64::sqrt(color.y), 0.0, 0.99);
    let b = clamp(f64::sqrt(color.z), 0.0, 0.99);
    println!(
        "{} {} {}",
        (255.999 * r) as i32,
        (255.999 * g) as i32,
        (255.999 * b) as i32,
    );
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min } else if x > max { max } else { x }
}

/// pick a random point in a *unit radius* sphere
fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen::<f64>();
        let y = rng.gen::<f64>();
        let z = rng.gen::<f64>();
        let p = 2.0 * Vec3::new(x, y, z) - Vec3::new(1.0, 1.0, 1.0);

        if p.dot(&p) < 1.0 {
            return p;
        }
    }
}

/// provides more uniform scattering than `random_in_unit_sphere`
fn random_unit_vector() -> Vec3 {
    let mut rng = rand::thread_rng();
    let a: f64 = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
    let z: f64 = rng.gen_range(-1.0, 1.0);
    let r = f64::sqrt(1.0 - z * z);
    Vec3::new(r * f64::cos(a), r * f64::sin(a), z)
}

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    if let Some(rec) = world.hit(ray, 0.001, f64::MAX) {
        let target = rec.p + rec.normal + random_unit_vector();
        // send a new ray from hit point `p` to random point `target`
        0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1)
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
    const MAX_DEPTH: i32 = 50;

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
                pixel_color = pixel_color + ray_color(&ray, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done.");
}
