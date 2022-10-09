use nalgebra::{Point3, Rotation3, UnitVector3, Vector3};

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
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let den = ray.direction.dot(&self.up);
        if den == 0.0 {
            // ray is parallel to plane
            return None;
        }

        let ray_to_pos = self.position - ray.position;
        let distance = ray_to_pos.dot(&self.up) / den;
        if distance <= 0.0 {
            // plane is behind
            return None;
        }

        return Some(Intersection {
            distance,
            point: ray.position + ray.direction.scale(distance),
            normal: self.up,
        });
    }
}
