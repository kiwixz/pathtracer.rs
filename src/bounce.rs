use nalgebra::{Rotation3, UnitVector3, Vector3};

use crate::{math, ray::Ray, scene::Object, shapes::Intersection};

pub struct Bounce {
    pub probability: f64,
    pub direction: UnitVector3<f64>,
}

pub fn bounces(
    ray: &Ray,
    object: &Object,
    intersection: &Intersection,
    samples: i32,
) -> Vec<Bounce> {
    let always = |direction| {
        vec![Bounce {
            probability: 1.0,
            direction,
        }]
    };

    if !math::rand_bool(object.refraction) {
        if object.specular == 1.0 {
            return always(math::reflect(&ray.direction, &intersection.normal));
        }

        let diffuse_rotation =
            Rotation3::rotation_between(&Vector3::z_axis(), &intersection.normal).unwrap_or(
                Rotation3::from_axis_angle(&Vector3::x_axis(), std::f64::consts::PI),
            );

        let probability = 1.0 / samples as f64;

        if object.specular == 0.0 {
            return (0..samples)
                .map(|_| Bounce {
                    probability,
                    direction: diffuse_rotation * math::rand_direction_z(),
                })
                .collect();
        }

        let reflection = math::reflect(&ray.direction, &intersection.normal);
        return (0..samples)
            .map(|_| Bounce {
                probability,
                direction: (diffuse_rotation * math::rand_direction_z())
                    .slerp(&reflection, object.specular),
            })
            .collect();
    }

    let incident_eta = if intersection.from_inside { 1.5 } else { 1.0 };
    let refraction_eta = if intersection.from_inside { 1.0 } else { 1.5 };

    let reflectance = math::reflectance(
        &ray.direction,
        &intersection.normal,
        incident_eta,
        refraction_eta,
    );
    if reflectance >= 1.0 || samples == 1 && math::rand_bool(reflectance) {
        return always(math::reflect(&ray.direction, &intersection.normal));
    }

    let refraction = math::refract(
        &ray.direction,
        &intersection.normal,
        incident_eta,
        refraction_eta,
    );
    if refraction.is_none() {
        return always(math::reflect(&ray.direction, &intersection.normal));
    }

    if samples == 1 {
        return always(refraction.unwrap());
    }

    vec![
        Bounce {
            probability: reflectance,
            direction: math::reflect(&ray.direction, &intersection.normal),
        },
        Bounce {
            probability: 1.0 - reflectance,
            direction: refraction.unwrap(),
        },
    ]
}
