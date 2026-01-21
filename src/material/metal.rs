use std::sync::Arc;

use crate::{
    color::Color,
    material::Material,
    math::{Ray, Vec3, vec3::random_unit_vector},
    texture::{Texture, solid_color::SolidColor},
};

#[derive(Clone)]
pub struct Metal {
    /// The texture of the material.
    pub tex: Arc<dyn Texture>,

    /// The fuzziness of the reflected rays
    pub fuzz: f32,
}

impl Metal {
    /// Create a metal material from albedo and fuzz
    pub fn new(color: Color, fuzz: f32) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(color)),
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }

    /// Create a metal material from texture.
    pub fn from_texture<T>(tex: T, fuzz: f32) -> Self
    where
        T: Texture + 'static,
    {
        Self {
            tex: Arc::new(tex),
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &crate::shape::HitRecord) -> Option<(Color, Ray)> {
        let mut reflect_direction: Vec3 = r_in.dir.reflect(rec.normal);

        // Normalize the `reflect_direction` or the direction change brought by `fuzz` will be very small.
        reflect_direction = reflect_direction.normalize() + self.fuzz * random_unit_vector();

        let attenuation = self.tex.sample(rec.u, rec.v, rec.p);
        let scatter = Ray::new(rec.p, reflect_direction, r_in.t);

        // After we add fuzz, we need to ensure the scattered ray is still in outer side of the surface of sphere
        if scatter.dir.dot(rec.normal) > 0.0 {
            Some((attenuation, scatter))
        } else {
            None
        }
    }
}
