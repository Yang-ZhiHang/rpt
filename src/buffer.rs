use image::{ImageBuffer, RgbImage};

use crate::color::{Color, color_bytes};

/// A buffer to store the result of path tracing.
pub struct Buffer {
    /// The width of image.
    width: u32,

    /// The height of image.
    height: u32,

    /// The sample colors of image.
    samples: Vec<Vec<Color>>,
}

impl Buffer {
    /// Create a empty buffer with width and height.
    pub const fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            samples: Vec::new(),
        }
    }

    /// Push a color into the buffer.
    pub fn push(&mut self, x: u32, y: u32, color: Color) {
        assert!(x < self.width && y < self.height, "Invalid pixel location!");
        let index = (y * self.width + x) as usize;
        self.samples[index].push(color);
    }

    /// Extend a list of colors into the buffer.
    pub fn extend(&mut self, colors: Vec<Vec<Color>>) {
        self.samples.extend(colors);
    }

    /// Transite the buffer into rgb image.
    pub fn image(&self) -> RgbImage {
        let mut buf = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.samples[y as usize][x as usize];
                let [r, g, b] = color_bytes(color);
                buf.push(r);
                buf.push(g);
                buf.push(b);
            }
        }
        ImageBuffer::from_raw(self.width, self.height, buf).expect("Incorrect image size.")
    }
}
