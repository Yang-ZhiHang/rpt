use std::sync::Arc;

use crate::{
    color::{self, Color},
    material::Material,
    math::{Ray, vec3::random_unit_vector},
    texture::{Texture, solid_color::SolidColor},
};

#[derive(Clone)]
pub struct Isotropic {
    /// The texture of the material
    pub tex: Arc<dyn Texture>,
}

impl Default for Isotropic {
    /// Create a default isotropic material with gray albedo
    fn default() -> Self {
        Self {
            tex: Arc::new(SolidColor::new(color::WHITE)),
        }
    }
}

impl Isotropic {
    /// Create a isotropic material from albedo
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    /// Create a isotropic material from texture
    pub fn from_texture<T>(tex: T) -> Self
    where
        T: Texture + 'static,
    {
        Self { tex: Arc::new(tex) }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &crate::shape::HitRecord,
        attenuation: &mut Color,
        scatter: &mut Ray,
    ) -> bool {
        *attenuation = self.tex.sample(rec.u, rec.v, rec.p);
        *scatter = Ray::new(rec.p, random_unit_vector(), r_in.t);
        true
    }
}
