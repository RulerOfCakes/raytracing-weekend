use w2::samples::cornell_box::cornell_box;

fn main() {
    let mut out = std::io::stdout().lock();
    if let Err(e) = cornell_box(&mut out) {
        eprintln!("Error: {}", e);
    }
}
