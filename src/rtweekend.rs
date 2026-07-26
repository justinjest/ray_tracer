pub use crate::aabb::*;
pub use crate::bvh::*;
pub use crate::camera::*;
pub use crate::color::*;
pub use crate::constant_medium::*;
pub use crate::hittable::*;
pub use crate::hittable_list::*;
pub use crate::interval::*;
pub use crate::material::*;
pub use crate::ray::*;
pub use crate::rotation::*;
pub use crate::sphere::*;
pub use crate::texture::*;
pub use crate::tri::*;
pub use crate::vec3::*;
pub use crate::{perlin::*, quad::*};
pub use rand::prelude::*;
pub use std::io;
pub use std::sync::Arc;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::rng().random()
}

pub fn random_between(min: f64, max: f64) -> f64 {
    assert!(min < max);
    let mut rng = rand::rng();
    let x: f64 = rng.random();
    min + (max - min) * x
}

pub fn random_int(min: usize, max: usize) -> usize {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub fn random_vector_between(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_between(min, max),
        random_between(min, max),
        random_between(min, max),
    )
}
