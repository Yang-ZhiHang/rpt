use glam::{Mat4, Vec3A, Vec4Swizzles};
use std::f32::EPSILON;

use crate::common::random;

pub type Vec3 = Vec3A;
pub type Point3 = Vec3A;

/// Vector utilities module for Vec3 operations
pub mod vec3 {
    use super::*;
    use rand::random_range;

    /// Generate a random vector with each component in [0, 1)
    #[inline]
    pub fn random() -> Vec3 {
        Vec3::new(super::random(), super::random(), super::random())
    }

    /// Generate a random vector with each component in [min, max)
    #[inline]
    pub fn random_in_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            random_range(min..max),
            random_range(min..max),
            random_range(min..max),
        )
    }

    /// Randomly generate a vector in a unit sphere (length <= 1.0)
    #[inline]
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = random_in_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    /// Randomly generate a vector on the surface of a unit sphere (length == 1.0)
    #[inline]
    pub fn random_unit_vector() -> Vec3 {
        random_in_unit_sphere().normalize()
    }

    /// Randomly generate a vector in a unit disk (length <= 1.0, z=0)
    #[inline]
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_range(-1.0..1.0), random_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

pub trait Vec3Ext {
    /// Check if the vector is close to zero in length
    fn near_zero(&self) -> bool;
}

impl Vec3Ext for Vec3 {
    #[inline]
    fn near_zero(&self) -> bool {
        self.length_squared() < EPSILON
    }
}

#[derive(Default)]
/// A ray can be represented as: A + t*B where A is origin, B is direction, and t is a scalar
/// that indicates how long the ray has traveled. For any given value of t, we can compute the
/// point along the ray using the `at` method below.
pub struct Ray {
    /// The origin coordinate of ray
    pub origin: Point3,

    /// The direction vector of ray
    pub direction: Vec3,

    /// The time to define the position of moving objects. It can be understood as the ray
    /// entering the camera at time t.
    pub t: f32,
}

impl Ray {
    /// Create a ray from origin, direction and time
    pub fn new(origin: Point3, direction: Vec3, time: f32) -> Self {
        Ray {
            origin,
            direction,
            t: time,
        }
    }

    /// Get the point along the ray at time t.
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn apply_transform(&self, trans: &Mat4) -> Ray {
        let origin = trans.mul_vec4(self.origin.extend(1.0));
        // Direction no need to translate
        let direction = trans.mul_vec4(self.direction.extend(0.0));
        Ray {
            origin: origin.xyz().to_vec3a(),
            direction: direction.xyz().to_vec3a(),
            t: self.t,
        }
    }
}
