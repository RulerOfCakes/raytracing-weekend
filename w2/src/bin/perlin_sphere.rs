use w2::samples::perlin_spheres::perlin_spheres;

fn main() {
    let mut out = std::io::stdout().lock();
    if let Err(e) = perlin_spheres(&mut out) {
        eprintln!("Error: {}", e);
    }
}
