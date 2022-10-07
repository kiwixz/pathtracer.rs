mod scene;
mod thread_pool;

use scene::Scene;
use std::{
    error::Error,
    num::NonZeroUsize,
    sync::{mpsc, Arc},
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let workers = std::thread::available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap());

    let scene_toml = std::fs::read_to_string("scenes/cornell.toml")?;
    let scene: Arc<Scene> = Arc::new(toml::from_str(&scene_toml)?);

    let (sender, receiver) = mpsc::sync_channel(workers.get() * 2);

    let pool = thread_pool::Static::build(workers)?;
    for _ in 0..scene.samples {
        let sender = sender.clone();
        let scene = scene.clone();
        pool.submit(move || sender.send(pathtrace_sample(&scene).unwrap()).unwrap());
    }

    drop(sender);

    let mut image = vec![[0.0, 0.0, 0.0]; (scene.width * scene.height) as usize];
    while let Ok(sample) = receiver.recv() {
        for (image_pixel, sample_pixel) in image.iter_mut().zip(sample) {
            for (image_color, sample_color) in image_pixel.iter_mut().zip(sample_pixel) {
                *image_color += sample_color;
            }
        }
    }
    for pixel in image.iter_mut() {
        for color in pixel.iter_mut() {
            *color /= scene.samples as f64;
        }
    }

    let file = std::fs::File::create("output.png")?;
    let mut encoder = png::Encoder::new(file, scene.width as u32, scene.height as u32);
    encoder.set_color(png::ColorType::Rgb);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(
        &image
            .iter()
            .flatten()
            .map(|color| (color * 256.0) as u8)
            .collect::<Vec<u8>>(),
    )?;
    writer.finish()?;

    Ok(())
}

fn pathtrace_sample(scene: &Scene) -> Result<Vec<[f64; 3]>, Box<dyn Error>> {
    (0..scene.height)
        .flat_map(|y| (0..scene.width).map(move |x| Ok(pathtrace_pixel(scene, x, y)?)))
        .collect()
}

fn pathtrace_pixel(scene: &Scene, x: i32, y: i32) -> Result<[f64; 3], Box<dyn Error>> {
    Ok([
        x as f64 / scene.width as f64,
        y as f64 / scene.height as f64,
        0.0,
    ])
}
