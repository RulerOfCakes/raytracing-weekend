use w2::samples::simple_light::simple_light;

fn main() {
    let mut out = std::io::stdout().lock();
    if let Err(e) = simple_light(&mut out) {
        eprintln!("Error: {}", e);
    }
}
