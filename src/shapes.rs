use nalgebra::{Point3, UnitVector3};

use crate::ray::Ray;

pub mod plane;
pub mod sphere;

pub trait Shape: Send + Sync {
    fn intersect(&self, ray: &Ray, max_distance: Option<f64>) -> Option<Intersection>;
}

pub struct Intersection {
    pub distance: f64,
    pub point: Point3<f64>,
    pub normal: UnitVector3<f64>,
    pub from_inside: bool,
}
