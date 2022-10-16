mod math;
mod ray;
mod scene;
mod shapes;
mod thread_pool;

use std::{
    error::Error,
    num::NonZeroUsize,
    sync::{mpsc, Arc},
};

use nalgebra::{Point3, Rotation3, Unit, Vector3};

use ray::Ray;
use scene::{Color, Scene};

pub fn run() -> Result<(), Box<dyn Error>> {
    let scene: Arc<Scene> = Arc::new(Scene::open("scenes/cornell.toml")?);

    let workers = std::thread::available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap());
    let (sender, receiver) = mpsc::sync_channel(workers.get() * 2);

    let pool = thread_pool::Static::build(workers)?;
    for _ in 0..scene.samples {
        let sender = sender.clone();
        let scene = scene.clone();
        pool.submit(move || sender.send(pathtrace_sample(&scene)).unwrap());
    }

    drop(sender);

    let mut image = vec![Color::zeros(); (scene.width * scene.height) as usize];
    while let Ok(sample) = receiver.recv() {
        for (image_pixel, sample_pixel) in image.iter_mut().zip(sample) {
            *image_pixel += sample_pixel;
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
            .map(|color| (color * u8::MAX as f64) as u8)
            .collect::<Vec<u8>>(),
    )?;
    writer.finish()?;

    Ok(())
}

fn pathtrace_sample(scene: &Scene) -> Vec<Color> {
    (0..scene.height)
        .flat_map(|y| (0..scene.width).map(move |x| pathtrace_pixel(scene, x, y)))
        .collect()
}

fn pathtrace_pixel(scene: &Scene, x: i32, y: i32) -> Color {
    let pixel_on_screen = Point3::new(
        x as f64 - scene.width as f64 / 2.0,
        (scene.height - 1 - y) as f64 - scene.height as f64 / 2.0,
        -1.0,
    );

    let ray = Ray {
        position: scene.camera.position,
        direction: scene.camera.rotation
            * Unit::new_normalize(scene.camera.scale * pixel_on_screen.coords),
    };

    radiance(scene, &ray, 0)
}

fn radiance(scene: &Scene, ray: &Ray, bounce: i32) -> Color {
    let closest_match = scene
        .objects
        .iter()
        .filter_map(|o| Some((o, o.shape.intersect(ray)?)))
        .min_by(|(_, a), (_, b)| a.distance.partial_cmp(&b.distance).unwrap());
    if closest_match.is_none() {
        return scene.background_color;
    }
    let (obj, inter) = closest_match.unwrap();

    if obj.color == Color::zeros() {
        return obj.emission;
    }

    if bounce >= scene.min_bounces {
        if bounce >= scene.max_bounces {
            return obj.emission;
        }

        let survive_probability =
            (scene.max_bounces - bounce) as f64 / (scene.max_bounces - scene.min_bounces) as f64;
        if math::rand() > survive_probability {
            return obj.emission;
        }
    }

    let diffuse_direction =
        Rotation3::rotation_between(&Vector3::z_axis(), &inter.normal).unwrap_or(
            Rotation3::from_axis_angle(&Vector3::x_axis(), std::f64::consts::PI),
        ) * math::rand_direction_z();
    let specular_direction = math::reflect_unit(&ray.direction, &inter.normal);
    let direction = diffuse_direction.slerp(&specular_direction, obj.specular);

    radiance(
        scene,
        &Ray {
            position: inter.point + direction.scale(scene.epsilon),
            direction,
        },
        bounce + 1,
    )
    .component_mul(&obj.color)
}
