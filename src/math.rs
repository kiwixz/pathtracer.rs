use std::cell::RefCell;

use nalgebra::{Unit, UnitVector3, Vector3};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

pub fn lerp(a: f64, b: f64, ratio: f64) -> f64 {
    a * (1.0 - ratio) + b * ratio
}

pub fn rand() -> f64 {
    thread_local! { static RNG: RefCell<Xoshiro256Plus> = RefCell::new(Xoshiro256Plus::from_entropy()); }

    RNG.with(|a| a.borrow_mut().gen())
}

pub fn rand_bool(probability: f64) -> bool {
    probability >= 1.0 || probability > 0.0 && rand() < probability
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

pub fn reflect(incident: &UnitVector3<f64>, normal: &UnitVector3<f64>) -> UnitVector3<f64> {
    Unit::new_unchecked(incident.into_inner() - normal.scale(normal.dot(incident) * 2.0))
}

pub fn refract(
    incident: &UnitVector3<f64>,
    normal: &UnitVector3<f64>,
    incident_eta: f64,
    refraction_eta: f64,
) -> Option<UnitVector3<f64>> {
    let eta_ratio_inv = incident_eta / refraction_eta;
    let incident_angle_cos = -normal.dot(incident);
    let refraction_angle_cos_sq =
        1.0 - (1.0 - incident_angle_cos * incident_angle_cos) * eta_ratio_inv * eta_ratio_inv;
    if refraction_angle_cos_sq < 0.0 {
        // total internal reflection
        return None;
    }
    let refraction_angle_cos = refraction_angle_cos_sq.sqrt();

    Some(Unit::new_unchecked(
        incident.scale(eta_ratio_inv)
            + normal.scale(eta_ratio_inv * incident_angle_cos - refraction_angle_cos),
    ))
}

pub fn reflectance(
    incident: &UnitVector3<f64>,
    normal: &UnitVector3<f64>,
    incident_eta: f64,
    refraction_eta: f64,
) -> f64 {
    // fresnel schlick approximation
    let incident_angle_cos = -normal.dot(incident);
    if incident_angle_cos <= 0.0 {
        return 1.0;
    }

    let normal_reflectance_sqrt = (incident_eta - refraction_eta) / (incident_eta + refraction_eta);
    let normal_reflectance = normal_reflectance_sqrt * normal_reflectance_sqrt;

    lerp(
        normal_reflectance,
        1.0,
        (1.0 - incident_angle_cos).powf(5.0),
    )
}
