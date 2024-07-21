use w2::samples::earth::earth;

fn main() {
    let mut out = std::io::stdout().lock();
    if let Err(e) = earth(&mut out) {
        eprintln!("Error: {}", e);
    }
}
