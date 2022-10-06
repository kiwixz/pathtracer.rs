mod thread_pool;
mod scene;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let scene_toml = std::fs::read_to_string("scenes/cornell.toml")?;
    let scene: scene::Scene = toml::from_str(&scene_toml)?;

    println!("{}", scene.planes.len());

    let pool = thread_pool::Static::build(
        std::thread::available_parallelism().unwrap_or(std::num::NonZeroUsize::new(1).unwrap()),
    )?;

    for _ in 0..4 {
        pool.submit(|| {
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("hello");
        });
    }

    Ok(())
}
