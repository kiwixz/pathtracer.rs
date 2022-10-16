use std::cell::RefCell;

use nalgebra::{SVector, Unit, UnitVector3, Vector3};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

pub fn rand() -> f64 {
    thread_local! { static RNG: RefCell<Xoshiro256Plus> = RefCell::new(Xoshiro256Plus::from_entropy()); }

    RNG.with(|a| a.borrow_mut().gen())
}

pub fn rand_direction() -> UnitVector3<f64> {
    let mut x;
    let mut y;
    let mut norm_xy_sq;
    loop {
        x = -1.0 + rand() * 2.0;
        y = -1.0 + rand() * 2.0;
        norm_xy_sq = x * x + y * y;
        if norm_xy_sq <= 1.0 {
            break;
        }
    }

    let z = -1.0 + norm_xy_sq * 2.0;
    let k = (1.0 - norm_xy_sq).sqrt() * 2.0;
    x *= k;
    y *= k;

    UnitVector3::new_unchecked(Vector3::new(x, y, z))
}

pub fn rand_direction_z() -> UnitVector3<f64> {
    let direction = rand_direction();
    UnitVector3::new_unchecked(Vector3::new(direction.x, direction.y, direction.z.abs()))
}

pub fn reflect<const DIM: usize>(
    a: &SVector<f64, DIM>,
    b: &SVector<f64, DIM>,
) -> SVector<f64, DIM> {
    a - b.scale(b.dot(a) * 2.0)
}

pub fn reflect_unit<const DIM: usize>(
    a: &Unit<SVector<f64, DIM>>,
    b: &Unit<SVector<f64, DIM>>,
) -> Unit<SVector<f64, DIM>> {
    Unit::new_unchecked(reflect(a.as_ref(), b))
}
