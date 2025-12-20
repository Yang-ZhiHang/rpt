use crate::math::{Point3, Ray, Vec3};

#[derive(Default)]
/// Why we need a record struct for hit?
/// Because when we scan the viewport, we should compare each object and know
/// which t is the nearest so that we can shade it.
///
/// A HitRecord contains p and t.
/// As we know, a point in the ray can be performed like: p = origin + t * direction
/// point: p, origin
/// vector: direction
/// t: variable
pub struct HitRecord {
    pub p: Point3,
    pub t: f32,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new() -> Self {
        Default::default()
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
