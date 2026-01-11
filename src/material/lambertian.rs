use std::sync::Arc;

use crate::material::Material;
use crate::math::{Color, ColorExt, Ray, Vec3, Vec3Ext};
use crate::shape::HitRecord;
use crate::texture::Texture;
use crate::texture::solid_color::SolidColor;

#[derive(Clone)]
pub struct Lambertian {
    /// The texture of the material.
    pub tex: Arc<dyn Texture>,
}

impl Default for Lambertian {
    /// Create a default lambertian material with gray albedo.
    fn default() -> Self {
        Self {
            tex: Arc::new(SolidColor::new(Color::white())),
        }
    }
}

impl Lambertian {
    /// Create a lambertian material from albedo.
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    /// Create a lambertian material from texture.
    pub fn from_texture<T>(tex: T) -> Self
    where
        T: Texture + 'static,
    {
        Self { tex: Arc::new(tex) }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scatter: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *attenuation = self.tex.sample(rec.u, rec.v, rec.normal);
        *scatter = Ray::new(rec.p, scatter_direction, r_in.t);
        true
    }
}
