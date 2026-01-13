use image::RgbImage;
use indicatif::ProgressBar;
use rayon::prelude::*;

use crate::buffer::Buffer;
use crate::camera::Camera;
use crate::color::{self, Color};
use crate::common::random;
use crate::interval::Interval;
use crate::math::Ray;
use crate::scene::Scene;
use crate::shape::{HitRecord, Hittable};

pub struct Renderer {
    /// The camera to use
    pub cam: Camera,

    /// The scene to render
    pub scene: Scene,

    /// The width of output image
    pub width: u32,

    /// The height of output image
    pub height: u32,

    /// The number of samplings for one pixel in an image.
    pub num_samples: u32,

    /// The maximum number of the light bounces in the image.
    pub max_bounces: u32,

    /// The progress bar to show
    pub pb: Option<ProgressBar>,
}

impl Renderer {
    /// Create a renderer from camera and scene.
    pub fn new(cam: Camera, scene: Scene) -> Self {
        Self {
            cam,
            scene,
            width: 800,
            height: 600,
            pb: None,
            max_bounces: 50,
            num_samples: 100,
        }
    }

    /// Set the width of output image.
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Set the height of output image.
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Set progress bar for renderer.
    pub fn progress_bar(mut self, pb: ProgressBar) -> Self {
        self.pb = Some(pb);
        self
    }

    /// Set number of samplings for one pixel.
    pub fn num_samples(mut self, n: u32) -> Self {
        self.num_samples = n;
        self
    }

    /// Set maximum number of the light bounces for renderer.
    pub fn max_bounces(mut self, n: u32) -> Self {
        self.max_bounces = n;
        self
    }

    /// Trace the ray and return the color.
    pub fn trace_ray(
        &self,
        ray: &Ray,
        scene: &dyn Hittable,
        num_bounces: u32,
        rec: &mut HitRecord,
    ) -> Color {
        if num_bounces <= 0 {
            return color::BLACK;
        }

        // Start ray interval above zero to avoid shadow acne.
        if !scene.intersect(ray, Interval::new(1e-3, f32::INFINITY), rec) {
            return self.scene.background;
        }

        let mut attenuation = Color::default();
        let mut scatter = Ray::default();
        let illustrate_color = rec
            .material
            .as_ref()
            .unwrap()
            .illustrate(rec.u, rec.v, rec.p);

        // The material could use `unwrap` instead of `map_or` because it will not be `None` if
        // scene.intersect is true.
        if !rec
            .material
            .as_ref()
            .unwrap()
            .scatter(ray, rec, &mut attenuation, &mut scatter)
        {
            return illustrate_color;
        }
        return illustrate_color
            + attenuation * self.trace_ray(&scatter, scene, num_bounces - 1, rec);
    }

    /// Get the pixel color of a specified location in film plane.
    pub fn get_color(&self, col: u32, row: u32, iterations: u32) -> Color {
        let mut pixel_color = Color::default();
        let mut rec = HitRecord::new();
        for _ in 0..iterations {
            let u = (col as f32 + random()) / (self.width - 1) as f32;
            let v = (row as f32 + random()) / (self.height - 1) as f32;
            let r = self.cam.get_ray(u, v);
            pixel_color += self.trace_ray(&r, &self.scene, self.max_bounces, &mut rec);
        }
        pixel_color / iterations as f32
    }

    /// Get all pixel colors in film plane and store into `buffer`.
    pub fn sample(&self, iterations: u32, buffer: &mut Buffer) {
        let colors: Vec<_> = (0..self.height)
            .into_par_iter()
            .rev()
            .map(|row| {
                let row_pixels: Vec<Color> = (0..self.width)
                    .map(|col| self.get_color(col, row, iterations))
                    .collect();

                // Update progress bar after finish each row
                self.pb.as_ref().map(|pb| pb.inc(1));
                row_pixels
            })
            .collect();
        buffer.extend(colors);
        self.pb.as_ref().map(|pb| pb.finish_with_message("Done!"));
    }

    /// Render the image for given scene and return `RgbImage`.
    pub fn render(&self) -> RgbImage {
        let mut buffer = Buffer::new(self.width, self.height);
        self.sample(self.num_samples, &mut buffer);
        buffer.image()
    }
}
