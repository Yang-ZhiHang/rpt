use std::sync::Arc;

use crate::{
    color::{self, Color},
    material::Material,
    math::random,
    math::{Ray, Vec3},
    shape::HitRecord,
    texture::{Texture, solid_color::SolidColor},
};

#[derive(Clone)]
pub struct Dielectric {
    /// The index of refraction of the dielectric material.
    pub index: f32,

    /// The texture of the material.
    pub tex: Arc<dyn Texture>,
}

impl Default for Dielectric {
    fn default() -> Self {
        Self {
            index: 1.5,
            tex: Arc::new(SolidColor::new(color::WHITE)),
        }
    }
}

impl Dielectric {
    /// Create a dielectric material from index of refraction and color.
    pub fn new(index_of_refraction: f32, color: Color) -> Self {
        Self {
            index: index_of_refraction,
            tex: Arc::new(SolidColor::new(color)),
        }
    }

    /// Create a dielectric material from index of refraction and texture.
    pub fn from_texture<T>(index_of_refraction: f32, tex: T) -> Self
    where
        T: Texture + 'static,
    {
        Self {
            index: index_of_refraction,
            tex: Arc::new(tex),
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let eta = if rec.front_face {
            1.0 / self.index
        } else {
            self.index
        };
        let unit_direction = r_in.dir.normalize();

        // Ensure the incident ray has refract ray
        let cos_theta = -unit_direction.dot(rec.normal).min(1.0);
        let sin_theta = (cos_theta.mul_add(-cos_theta, 1.0)).sqrt();
        let refractable = eta * sin_theta < 1.0;

        // The larger the incident angle, and the more reflection component the human eye sees
        let direction: Vec3 =
            if refractable && Self::reflectance(self, cos_theta, self.index) <= random() {
                unit_direction.refract(rec.normal, eta)
            } else {
                unit_direction.reflect(rec.normal)
            };

        let attenuation = self.tex.sample(rec.u, rec.v, rec.p);
        let scatter = Ray::new(rec.p, direction, rec.t);
        Some((attenuation, scatter))
    }
}
