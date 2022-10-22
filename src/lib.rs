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

use nalgebra::{Point3, Rotation3, Unit, UnitVector3, Vector3};

use ray::Ray;
use scene::{Color, Object, Scene};
use shapes::Intersection;

pub fn run() -> Result<(), Box<dyn Error>> {
    let scene: Arc<Scene> = Arc::new(Scene::open("scenes/cornell.toml")?);

    let workers = std::thread::available_parallelism().unwrap_or(NonZeroUsize::new(1).unwrap());
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
    (0..scene.supersampling)
        .flat_map(|super_y| {
            (0..scene.supersampling).map(move |super_x| {
                pathtrace_subpixel(
                    scene,
                    x as f64
                        + super_x as f64 / scene.supersampling as f64
                        + 0.5 / scene.supersampling as f64,
                    y as f64
                        + super_y as f64 / scene.supersampling as f64
                        + 0.5 / scene.supersampling as f64,
                )
            })
        })
        .sum::<Color>()
        / (scene.supersampling * scene.supersampling) as f64
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

    let color_sum: Color = (0..scene.samples).map(|_| radiance(scene, &ray, 0)).sum();
    (color_sum / scene.samples as f64).map(|a| a.clamp(0.0, 1.0))
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

    if bounce >= scene.min_bounces
        && (bounce >= scene.max_bounces
            || !math::rand_bool(
                (scene.max_bounces - bounce) as f64
                    / (scene.max_bounces - scene.min_bounces + 1) as f64
                    * obj.color.mean(),
            ))
    {
        return obj.emission;
    }

    let direction = bounce_direction(ray, obj, &inter);

    obj.emission
        + radiance(
            scene,
            &Ray {
                position: inter.point + direction.scale(scene.epsilon),
                direction,
            },
            bounce + 1,
        )
        .component_mul(&obj.color)
}

fn bounce_direction(ray: &Ray, object: &Object, intersection: &Intersection) -> UnitVector3<f64> {
    let reflection = || math::reflect(&ray.direction, &intersection.normal);

    if !math::rand_bool(object.refraction) {
        if object.specular == 1.0 {
            return reflection();
        }

        let diffuse =
            Rotation3::rotation_between(&Vector3::z_axis(), &intersection.normal).unwrap_or(
                Rotation3::from_axis_angle(&Vector3::x_axis(), std::f64::consts::PI),
            ) * math::rand_direction_z();

        if object.specular == 0.0 {
            return diffuse;
        }

        return diffuse.slerp(&reflection(), object.specular);
    }

    let incident_eta = if intersection.from_inside { 1.5 } else { 1.0 };
    let refraction_eta = if intersection.from_inside { 1.0 } else { 1.5 };

    if math::rand_bool(math::reflectance(
        &ray.direction,
        &intersection.normal,
        incident_eta,
        refraction_eta,
    )) {
        return reflection();
    }

    math::refract(
        &ray.direction,
        &intersection.normal,
        incident_eta,
        refraction_eta,
    )
    .unwrap_or_else(reflection)
}
