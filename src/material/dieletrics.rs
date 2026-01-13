use std::sync::Arc;

use crate::{
    color::{self, Color},
    common::random,
    material::Material,
    math::{Ray, Vec3},
    shape::HitRecord,
    texture::{Texture, solid_color::SolidColor},
};

#[derive(Clone)]
pub struct Dielectric {
    /// The index of refraction of the dielectric material.
    pub index_of_refraction: f32,

    /// The texture of the material.
    pub tex: Arc<dyn Texture>,
}

impl Default for Dielectric {
    fn default() -> Self {
        Self {
            index_of_refraction: 1.5,
            tex: Arc::new(SolidColor::new(color::WHITE)),
        }
    }
}

impl Dielectric {
    /// Create a dielectric material from index of refraction and color.
    pub fn new(index_of_refraction: f32, color: Color) -> Self {
        Self {
            index_of_refraction,
            tex: Arc::new(SolidColor::new(color)),
        }
    }

    /// Create a dielectric material from index of refraction and texture.
    pub fn from_texture<T>(index_of_refraction: f32, tex: T) -> Self
    where
        T: Texture + 'static,
    {
        Self {
            index_of_refraction,
            tex: Arc::new(tex),
        }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scatter: &mut Ray,
    ) -> bool {
        let ir = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_direction = r_in.direction.normalize();

        // Ensure the incident ray has refract ray
        let cos_theta = -unit_direction.dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let refractable = ir * sin_theta < 1.0;

        // The smaller the cosine, the larger the incident angle, and
        // the more reflection component the human eye sees
        let direction: Vec3 = if refractable
            && Self::reflectance(&self, cos_theta, self.index_of_refraction) <= random()
        {
            unit_direction.refract(rec.normal, ir)
        } else {
            unit_direction.reflect(rec.normal)
        };

        *attenuation = self.tex.sample(rec.u, rec.v, rec.p);
        *scatter = Ray::new(rec.p, direction, rec.t);
        true
    }
}
