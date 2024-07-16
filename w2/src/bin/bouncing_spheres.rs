use w2::samples::bouncing_spheres::bouncing_spheres;

fn main() {
    let mut out = std::io::stdout().lock();
    if let Err(e) = bouncing_spheres(&mut out) {
        eprintln!("Error: {}", e);
    }
}
