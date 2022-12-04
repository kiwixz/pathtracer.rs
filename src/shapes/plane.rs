use nalgebra::{Point3, Rotation3, Unit, UnitVector3, Vector3};

use crate::ray::Ray;

use super::{Intersection, Shape};

pub struct Plane {
    position: Point3<f64>,
    up: UnitVector3<f64>,
}

impl Plane {
    pub fn new(position: Point3<f64>, rotation: &Rotation3<f64>) -> Plane {
        Plane {
            position,
            up: rotation * Vector3::y_axis(),
        }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray, max_distance: Option<f64>) -> Option<Intersection> {
        let denominator = ray.direction.dot(&self.up);
        if denominator == 0.0 {
            // ray is parallel to plane
            return None;
        }

        let ray_to_pos = self.position - ray.position;
        let distance = ray_to_pos.dot(&self.up) / denominator;
        if distance <= 0.0 {
            // plane is behind
            return None;
        }

        if max_distance.is_some() && distance > max_distance.unwrap() {
            return None;
        }

        return Some(Intersection {
            distance,
            point: ray.position + ray.direction.scale(distance),
            normal: if denominator < 0.0 {
                self.up
            } else {
                Unit::new_unchecked(self.up.scale(-1.0))
            },
            from_inside: false,
        });
    }
}
