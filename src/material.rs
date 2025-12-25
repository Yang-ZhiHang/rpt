use crate::{
    math::{Color, Ray},
    shape::HitRecord,
};

pub mod dieletrics;
pub mod lambertian;
pub mod metal;

pub trait Material: Send + Sync {
    /// Get the attenuation color and scattered ray to be able to compute the scattered color
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scatter: &mut Ray,
    ) -> bool;

    /// Use Schlick's approximation for reflectance
    fn reflectance(&self, cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
