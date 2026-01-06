use std::sync::Arc;

use crate::{
    material::Material,
    math::{Color, Point3},
    texture::{Texture, solid_color::SolidColor},
};

pub struct Light {
    /// The texture representing the albedo of the material
    pub tex: Arc<dyn Texture>,
}

impl Light {
    /// Create a default light material in white light.
    pub fn default() -> Self {
        Self {
            tex: Arc::new(SolidColor::new(Color::splat(1.0))),
        }
    }

    /// Create a light material from color.
    pub fn new(color: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(color)),
        }
    }

    /// Create a light material from texture.
    pub fn from_texture<T>(tex: T) -> Self
    where
        T: Texture + 'static,
    {
        Self { tex: Arc::new(tex) }
    }
}

impl Material for Light {
    fn illustrate(&self, u: f32, v: f32, p: Point3) -> Color {
        self.tex.sample(u, v, p)
    }
}
