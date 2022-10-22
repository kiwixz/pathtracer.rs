use nalgebra::{Rotation3, UnitVector3, Vector3};

use crate::{math, ray::Ray, scene::Object, shapes::Intersection};

pub struct Bounce {
    pub probability: f64,
    pub direction: UnitVector3<f64>,
}

pub fn bounces(
    samples: i32,
    ray: &Ray,
    object: &Object,
    intersection: &Intersection,
) -> Vec<Bounce> {
    (0..samples)
        .map(|_| Bounce {
            probability: 1.0 / samples as f64,
            direction: bounce_direction(ray, object, intersection),
        })
        .collect()
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
