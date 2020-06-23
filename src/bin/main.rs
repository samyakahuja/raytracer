fn main() {
    const WIDTH: i32 = 256;
    const HEIGHT: i32 = 256;

    // PP3 file header
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    for j in (0..HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..WIDTH {
            let r = i as f32 / (WIDTH - 1) as f32;
            let g = j as f32 / (HEIGHT - 1) as f32;
            let b = 0.25;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprintln!("Done.");
}
