use nalgebra::{Point3, Unit};

use crate::ray::Ray;

use super::{Intersection, Shape};

pub struct Sphere {
    position: Point3<f64>,
    radius_sq: f64,
}

impl Sphere {
    pub fn new(position: Point3<f64>, radius: f64) -> Sphere {
        Sphere {
            position,
            radius_sq: radius * radius,
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray, max_distance: Option<f64>) -> Option<Intersection> {
        // "a" is the point found by projecting sphere's position onto the ray

        let ray_to_pos = self.position - ray.position;
        let ray_to_a_norm = ray.direction.dot(&ray_to_pos);
        let pos_to_a_norm_sq = ray_to_pos.norm_squared() - ray_to_a_norm * ray_to_a_norm;
        let intersection_to_a_norm_sq = self.radius_sq - pos_to_a_norm_sq;
        if intersection_to_a_norm_sq < 0.0 {
            // no intersection
            return None;
        }

        let intersection_to_a_norm = intersection_to_a_norm_sq.sqrt();

        let mut distance = ray_to_a_norm - intersection_to_a_norm;
        let mut from_inside = false;
        if distance <= 0.0 {
            // this intersection is behind, return the other one

            distance = ray_to_a_norm + intersection_to_a_norm;
            if distance <= 0.0 {
                return None;
            }
            from_inside = true;
        }

        if max_distance.is_some() && distance > max_distance.unwrap() {
            return None;
        }

        let point = ray.position + ray.direction.scale(distance);
        return Some(Intersection {
            distance,
            point,
            normal: Unit::new_normalize(if from_inside {
                self.position - point
            } else {
                point - self.position
            }),
            from_inside,
        });
    }
}
