fn main() {
    if let Err(e) = pathtracer::run() {
        eprint!("\x1b[91;1m");
        eprint!("error: {e}");
        eprint!("\x1b[0m\n");
        std::process::exit(1);
    }
}
