use std::error::Error;

use nalgebra::{Point3, Rotation3, Scale3, Vector3};

use crate::shapes::{plane::Plane, sphere::Sphere, Shape};

mod config;

pub type Color = Vector3<f64>;

pub struct Scene {
    pub width: i32,
    pub height: i32,

    pub iterations: i32,
    pub supersampling: i32,
    pub samples: i32,
    pub bounces: i32,

    pub background_color: Color,
    pub epsilon: f64,

    pub camera: Camera,
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn open(path: &str) -> Result<Scene, Box<dyn Error>> {
        let file = std::fs::read_to_string(path)?;
        let config: config::Scene = toml::from_str(&file)?;

        let mut objects = Vec::with_capacity(config.planes.len() + config.spheres.len());
        for plane in &config.planes {
            objects.push(Object::new(
                plane.object.clone(),
                Box::new(Plane::new(
                    plane.position.into(),
                    &make_rotation(&plane.rotation),
                )),
            ))
        }
        for sphere in &config.spheres {
            objects.push(Object::new(
                sphere.object.clone(),
                Box::new(Sphere::new(sphere.position.into(), sphere.radius)),
            ))
        }

        let screen_ratio = (2.0 * (config.camera.field_of_view.to_radians() / 2.0).tan())
            / std::cmp::min(config.width, config.height) as f64;

        Ok(Scene {
            width: config.width,
            height: config.height,

            iterations: config.iterations,
            supersampling: config.supersampling,
            samples: config.samples,
            bounces: config.bounces,

            background_color: config.background_color.into(),
            epsilon: config.epsilon,

            camera: Camera {
                position: config.camera.position.into(),
                rotation: make_rotation(&config.camera.rotation),
                scale: Scale3::new(
                    config.camera.scale[0] * screen_ratio,
                    config.camera.scale[1] * screen_ratio,
                    config.camera.scale[2],
                ),
            },

            objects,
        })
    }
}

pub struct Camera {
    pub position: Point3<f64>,
    pub rotation: Rotation3<f64>,
    pub scale: Scale3<f64>,
}

pub struct Object {
    pub shape: Box<dyn Shape>,
    pub emission: Color,
    pub color: Color,
    pub specular: f64,
    pub refraction: f64,
}

impl Object {
    fn new(config: config::Object, shape: Box<dyn Shape>) -> Object {
        Object {
            shape,
            emission: config.emission.into(),
            color: config.color.into(),
            specular: config.specular,
            refraction: config.refraction,
        }
    }
}

fn make_rotation(angles: &[f64; 3]) -> Rotation3<f64> {
    Rotation3::from_euler_angles(
        angles[1].to_radians(),
        -angles[0].to_radians(),
        -angles[2].to_radians(),
    )
}
