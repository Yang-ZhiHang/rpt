use crate::{
    math::{Color, ColorExt, Point3, Ray},
    shape::HitRecord,
};

pub mod dieletrics;
pub mod isotropic;
pub mod lambertian;
pub mod light;
pub mod metal;

pub trait Material: Send + Sync {
    /// Get the attenuation color and scattered ray to be able to compute the scattered color
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scatter: &mut Ray,
    ) -> bool {
        false
    }

    fn illustrate(&self, _u: f32, _v: f32, _p: Point3) -> Color {
        Color::black()
    }

    /// Use Schlick's approximation for reflectance
    fn reflectance(&self, cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
