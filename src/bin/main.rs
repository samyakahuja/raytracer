use raytracer::vec3::Vec3;

fn write_color(color: Vec3) {
    let color = 255.999 * color;
    println!(
        "{} {} {}",
        color.x as i32,
        color.y as i32,
        color.z as i32,
    );
}

fn main() {
    const WIDTH: i32 = 256;
    const HEIGHT: i32 = 256;

    // PP3 file header
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for j in (0..HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..WIDTH {
            let r = i as f64 / (WIDTH - 1) as f64;
            let g = j as f64 / (HEIGHT - 1) as f64;
            let b = 0.25;
            let col = Vec3::new(r, g, b);
            write_color(col);
        }
    }
    eprintln!("Done.");
}
