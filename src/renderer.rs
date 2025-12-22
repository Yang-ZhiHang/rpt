use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use indicatif::ProgressBar;
use rayon::prelude::*;

use crate::camera::Camera;
use crate::common::random;
use crate::math::{Color, ColorExt, Ray, Vec3, Vec3Ext};
use crate::shape::{HitRecord, Hitable, Scene};

pub struct Renderer {
    /// The camera to use
    pub cm: Camera,

    /// The scene to render
    pub scene: Scene,

    /// The width of output image
    pub width: u32,

    /// The height of output image
    pub height: u32,

    /// The progress bar to show
    pub pb: Option<ProgressBar>,

    /// The path to save image
    pub output_path: PathBuf,
}

impl Renderer {
    pub fn new(cm: Camera, scene: Scene) -> Self {
        Self {
            cm,
            scene,
            width: 800,
            height: 600,
            pb: None,
            output_path: PathBuf::from("output/image.ppm"),
        }
    }

    /// Set the width of output image
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Set the height of output image
    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Set progress bar for renderer
    pub fn progress_bar(mut self, pb: ProgressBar) -> Self {
        self.pb = Some(pb);
        self
    }

    /// Set the output path for output image
    pub fn output_path(mut self, path: PathBuf) -> Self {
        self.output_path = path;
        self
    }

    pub fn sample(r: &Ray, s: &Scene, depth: u32, rec: &mut HitRecord) -> Color {
        if depth <= 0 {
            return Color::black();
        }
        if s.hit(r, 1e-5, f32::INFINITY, rec) {
            let direction = rec.normal + Vec3::random_in_unit_sphere();
            return 0.3 * Self::sample(&Ray::new(rec.p, direction), s, depth - 1, rec);
        }
        let unit_direction = r.direction.normalize();
        let t: f32 = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::rgb(1.0, 1.0, 1.0) + t * Color::rgb(0.5, 0.7, 1.0)
    }

    pub fn write_color(io: &mut impl std::io::Write, color: &Color, samples_per_pixel: u32) {
        let mut r = color.r();
        let mut g = color.g();
        let mut b = color.b();

        let scale = 1.0 / samples_per_pixel as f32;
        // sqrt for gamma-correct
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();

        // Translate [0, 1) values to [0, 255]
        io.write_all(
            format!(
                "{} {} {}\n",
                (256.0 * r.clamp(0.0, 0.999)) as u8,
                (256.0 * g.clamp(0.0, 0.999)) as u8,
                (256.0 * b.clamp(0.0, 0.999)) as u8
            )
            .as_bytes(),
        )
        .expect("Failed to write color");
    }

    pub fn render(&self, image_width: u32, image_height: u32, samples_per_pixel: u32, depth: u32) {
        let file = File::create(&self.output_path).expect("Create image file failed.");
        let mut writer = BufWriter::with_capacity(1024 * 1024, file);

        // Header of image which format in PPM
        writeln!(writer, "P3\n{} {}\n255", image_width, image_height)
            .expect("Failed to write PPM header");

        let rows: Vec<Vec<Color>> = (0..image_height)
            .into_par_iter()
            .rev()
            .map(|row| {
                let row_pixels: Vec<Color> = (0..image_width)
                    .map(|col| {
                        let mut pixel_color = Color::default();
                        let mut rec = HitRecord::new();
                        for _ in 0..samples_per_pixel {
                            let u = (col as f32 + random()) / (image_width - 1) as f32;
                            let v = (row as f32 + random()) / (image_height - 1) as f32;
                            let r = self.cm.get_ray(u, v);
                            pixel_color += Self::sample(&r, &self.scene, depth, &mut rec);
                        }
                        pixel_color
                    })
                    .collect();
                // Update progress bar after finish each row
                self.pb.as_ref().map(|pb| pb.inc(1));
                row_pixels
            })
            .collect();

        for row_pixels in rows {
            for pixel_color in row_pixels {
                Renderer::write_color(&mut writer, &pixel_color, samples_per_pixel);
            }
        }

        writer.flush().expect("Failed to flush buffer");
        self.pb.as_ref().map(|pb| pb.finish_with_message("Done!"));
    }
}
