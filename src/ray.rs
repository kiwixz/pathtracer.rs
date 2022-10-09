use nalgebra::{Point3, UnitVector3};

pub struct Ray {
    pub position: Point3<f64>,
    pub direction: UnitVector3<f64>,
}
