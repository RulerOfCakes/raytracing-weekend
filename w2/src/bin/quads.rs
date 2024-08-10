use w2::samples::quads::quads;

fn main() {
    let mut out = std::io::stdout().lock();
    if let Err(e) = quads(&mut out) {
        eprintln!("Error: {}", e);
    }
}
