use std::error::Error;

use nalgebra::Rotation3;

use crate::shapes::{plane::Plane, sphere::Sphere, Shape};

pub mod config;

pub struct Scene {
    pub config: config::Scene,
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
                        plane.rotation[0],
                        plane.rotation[1],
                        plane.rotation[2],
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

        Ok(Scene { config, objects })
    }
}

pub struct Object {
    pub shape: Box<dyn Shape>,
    pub diffusion: [f64; 3],
    pub specular: [f64; 3],
    pub refraction: [f64; 3],
    pub emission: [f64; 3],
}

impl Object {
    fn new(config: config::Object, shape: Box<dyn Shape>) -> Object {
        Object {
            shape,
            diffusion: config.diffusion,
            specular: config.specular,
            refraction: config.refraction,
            emission: config.emission,
        }
    }
}
