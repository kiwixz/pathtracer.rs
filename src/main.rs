use std::num::NonZeroUsize;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short = 'j', long, default_value_t = 0)]
    /// Number of workers, use 0 for auto
    threads: usize,

    #[arg()]
    /// Path to the scene file
    scene: String,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = pathtracer::run(&args.scene, NonZeroUsize::new(args.threads)) {
        eprint!("\x1b[91;1m");
        eprint!("error: {e}");
        eprint!("\x1b[0m\n");
        std::process::exit(1);
    }
}
