use crate::{
    material::Material,
    math::{Point3, Ray, Vec3},
};
use std::sync::Arc;

pub mod sphere;

#[derive(Default, Clone)]
pub struct HitRecord {
    /// The origin coordinate of light ray
    pub p: Point3,

    /// Time
    pub t: f32,

    /// The normal vector of the intersection surfaec towards the incident ray
    pub normal: Vec3,

    /// If the normal vector towards you
    pub front_face: bool,

    /// The material of intersect object
    pub material: Option<Arc<dyn Material>>,
}

impl HitRecord {
    /// Create a intersection record in default
    pub fn new() -> Self {
        Default::default()
    }

    /// Let normal vector face to the incident ray
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Shape: Send + Sync {
    /// Used for intersection informations of incident rays
    fn intersect(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
