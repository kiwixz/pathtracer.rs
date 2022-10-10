use std::error::Error;

use nalgebra::{Point3, Rotation3, Scale3, Vector3};

use crate::shapes::{plane::Plane, sphere::Sphere, Shape};

mod config;

pub type Color = Vector3<f64>;

pub struct Scene {
    pub width: i32,
    pub height: i32,
    pub samples: i32,
    pub min_bounces: i32,
    pub max_bounces: i32,
    pub background_color: Color,

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
                    &Rotation3::from_euler_angles(
                        plane.rotation[0].to_radians(),
                        plane.rotation[1].to_radians(),
                        plane.rotation[2].to_radians(),
                    ),
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
            samples: config.samples,
            min_bounces: config.min_bounces,
            max_bounces: config.max_bounces,
            background_color: config.background_color.into(),

            camera: Camera {
                position: config.camera.position.into(),
                rotation: Rotation3::from_euler_angles(
                    config.camera.rotation[1].to_radians(),
                    -config.camera.rotation[0].to_radians(),
                    -config.camera.rotation[2].to_radians(),
                ),
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
    pub diffusion: Color,
    pub specular: Color,
    pub refraction: Color,
    pub emission: Color,
}

impl Object {
    fn new(config: config::Object, shape: Box<dyn Shape>) -> Object {
        Object {
            shape,
            diffusion: config.diffusion.into(),
            specular: config.specular.into(),
            refraction: config.refraction.into(),
            emission: config.emission.into(),
        }
    }
}
