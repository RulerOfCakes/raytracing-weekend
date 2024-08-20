use w2::samples::final_scene::final_scene;

fn main() {
    let mut out = std::io::stdout().lock();
    if let Err(e) = final_scene(&mut out, 400, 250, 4) {
        eprintln!("Error: {}", e);
    }
}
