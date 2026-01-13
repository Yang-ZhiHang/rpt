pub mod checker_texture;
pub mod image_texture;
pub mod solid_color;

use crate::color::Color;
use crate::math::Vec3;

pub trait Texture: Send + Sync {
    /// Get the color of the texture in specified location from plane coordinates.
    fn sample(&self, u: f32, v: f32, p: Vec3) -> Color;
}
