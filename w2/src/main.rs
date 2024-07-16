use w2::samples::{bouncing_spheres::bouncing_spheres, checkered_spheres::checkered_spheres};

fn main() {
    let mut out = std::io::stdout().lock();
    if let Err(e) = checkered_spheres(&mut out) {
        eprintln!("Error: {}", e);
    }
}
