use std::sync::Arc;

use crate::{
    aabb::Aabb,
    interval::Interval,
    material::Material,
    math::{Point3, Ray, Vec3},
};

pub mod quad;
pub mod sphere;

#[derive(Default, Clone)]
pub struct HitRecord {
    /// The 3d coordinations of intersection point.
    pub p: Point3,

    /// Time which can be used to compute point along the ray through the formula
    /// p = origin + t * direction. This attribute is more microscopic than the time of
    /// the `Ray` structure.
    pub t: f32,

    /// The 3d coordinations of the normal vector in the intersection surfaec towards 
    /// the incident ray.
    pub normal: Vec3,

    /// The flag to determine whether the normal vector towards you. e.g. if the radius is 
    /// negative, then the normal vector is inverted.
    pub front_face: bool,

    /// The material of intersect object.
    pub material: Option<Arc<dyn Material>>,

    /// The coordinates of the object surface mapping to the texture map
    pub u: f32,
    pub v: f32,
}

impl HitRecord {
    /// Create a intersection record in default.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the normal vector of intersections surface which face to the incident ray.
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    /// Used for HitRecord of incident ray.
    fn intersect(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub trait Bounded: Hittable {
    /// The bounding box of the shape.
    fn bbox(&self) -> Aabb;
}
