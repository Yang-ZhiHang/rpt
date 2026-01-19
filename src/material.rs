use crate::{
    color::{self, Color},
    math::{Point3, Ray},
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

    /// Get the illustrate color of the material at the given uv coordinate and position
    fn illustrate(&self, _u: f32, _v: f32, _p: Point3) -> Color {
        color::BLACK
    }

    /// Use Schlick's approximation for reflectance
    fn reflectance(&self, cos: f32, eta: f32) -> f32 {
        let mut r0 = (1.0 - eta) / (1.0 + eta);
        r0 = r0 * r0;
        (1.0 - r0).mul_add((1.0 - cos).powi(5), r0)
    }
}
