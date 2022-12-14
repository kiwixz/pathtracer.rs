use std::{
    error::Error,
    num::NonZeroUsize,
    sync::{mpsc, Arc},
};

use nalgebra::{Point3, Unit};

use crate::{
    bounce::bounces,
    ray::Ray,
    scene::{Color, Scene},
    utils::*,
};

mod bounce;
mod math;
mod ray;
mod scene;
mod shapes;
mod thread_pool;
mod utils;

pub fn run(scene_path: &str, threads: Option<NonZeroUsize>) -> Result<(), Box<dyn Error>> {
    let scene: Arc<Scene> = Arc::new(Scene::open(scene_path)?);
    let workers = threads.unwrap_or_else(|| {
        std::thread::available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap())
    });

    let (senders, receivers): (Vec<_>, Vec<_>) = (0..scene.iterations)
        .map(|_| mpsc::sync_channel(workers.get() * 2))
        .unzip();

    let pool = thread_pool::StaticPool::new(workers);
    for sender in senders {
        for y in 0..scene.height {
            let sender = sender.clone();
            let scene = scene.clone();
            pool.submit(move || sender.send((y, pathtrace_row(&scene, y))).unwrap());
        }
    }

    let mut image = vec![Color::zeros(); (scene.width * scene.height) as usize];
    for (iteration, receiver) in receivers.iter().enumerate() {
        while let Ok((y, row)) = receiver.recv() {
            for (image_pixel, row_pixel) in
                image.iter_mut().skip((y * scene.width) as usize).zip(row)
            {
                *image_pixel += row_pixel;
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
                .map(|color| (color / (iteration + 1) as f64 * u8::MAX as f64) as u8)
                .collect::<Vec<u8>>(),
        )?;
        writer.finish()?;
    }

    Ok(())
}

fn pathtrace_row(scene: &Scene, y: i32) -> Vec<Color> {
    (0..scene.width)
        .map(|x| pathtrace_pixel(scene, x, y))
        .collect()
}

fn pathtrace_pixel(scene: &Scene, x: i32, y: i32) -> Color {
    (0..scene.pixel_samples)
        .map(|_| pathtrace_subpixel(scene, x as f64 + math::rand(), y as f64 + math::rand()))
        .sum::<Color>()
        / scene.pixel_samples as f64
}

fn pathtrace_subpixel(scene: &Scene, x: f64, y: f64) -> Color {
    let screen_subpixel = Point3::new(
        x - scene.width as f64 / 2.0,
        scene.height as f64 / 2.0 - y,
        -1.0,
    );
    let ray = Ray {
        position: scene.camera.position,
        direction: scene.camera.rotation
            * Unit::new_normalize(scene.camera.scale * screen_subpixel.coords),
    };

    let color = radiance(scene, &ray, 0, 1.0);
    let color_max = color.max();
    if color_max > 1.0 {
        color / color_max
    } else {
        color
    }
}

fn radiance(scene: &Scene, ray: &Ray, depth: i32, importance: f64) -> Color {
    let mut closest_match: Option<(&scene::Object, shapes::Intersection)> = None;
    for next_obj in &scene.objects {
        if let Some(next_inter) = next_obj
            .shape
            .intersect(ray, inspect(&closest_match, |(_, inter)| inter.distance))
        {
            closest_match = Some((next_obj, next_inter));
        }
    }

    if closest_match.is_none() {
        return scene.background_color;
    }
    let (obj, inter) = closest_match.unwrap();

    if obj.color == Color::zeros() || depth == scene.bounces {
        return obj.emission;
    }

    let bounces_importance = importance * obj.color.max();
    let bounces_samples =
        std::cmp::max((scene.bounce_samples as f64 * bounces_importance) as i32, 1);
    let bounces_color: Color = bounces(ray, obj, &inter, bounces_samples)
        .iter()
        .map(|bounce| {
            radiance(
                scene,
                &Ray {
                    position: inter.point + bounce.direction.scale(scene.epsilon),
                    direction: bounce.direction,
                },
                depth + 1,
                bounces_importance * bounce.probability,
            ) * bounce.probability
        })
        .sum();

    obj.emission + bounces_color.component_mul(&obj.color)
}
