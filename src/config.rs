use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use toml;

const DEFAULT_WIDTH: u32 = 400;
const DEFAULT_ASPECT_RATIO: f32 = 16.0 / 9.0;
const DEFAULT_SAMPLES_PER_PIXEL: u32 = 100;
const DEFAULT_MAX_DEPTH: u32 = 50;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    #[serde(default)]
    pub image: ImageConfig,
    #[serde(default)]
    pub render: RenderConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImageConfig {
    #[serde(default = "default_width", alias = "image_width", alias = "width")]
    pub width: u32,
    #[serde(default, alias = "image_height", alias = "height")]
    pub height: Option<u32>,
    #[serde(default, alias = "aspect_ratio")]
    pub aspect_ratio: Option<f32>,
    #[serde(default, alias = "aspect_width")]
    aspect_width: Option<u32>,
    #[serde(default, alias = "aspect_height")]
    aspect_height: Option<u32>,
    #[serde(default = "default_output_path", alias = "output_path")]
    pub output_path: String,
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            width: DEFAULT_WIDTH,
            height: None,
            aspect_ratio: None,
            aspect_width: None,
            aspect_height: None,
            output_path: default_output_path(),
        }
    }
}

impl ImageConfig {
    /// Resolve the image configuration into concrete values
    pub fn resolved(&self) -> ResolvedImage {
        let aspect = self
            .aspect_ratio
            .or_else(|| match (self.aspect_width, self.aspect_height) {
                (Some(w), Some(h)) if h != 0 => Some(w as f32 / h as f32),
                _ => None,
            })
            .unwrap_or(DEFAULT_ASPECT_RATIO);

        let width = self.width;
        let height = self
            .height
            .unwrap_or_else(|| (width as f32 / aspect).round() as u32);

        ResolvedImage {
            width,
            height,
            aspect_ratio: aspect,
            output_path: PathBuf::from(&self.output_path),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedImage {
    /// The width of output image
    pub width: u32,

    /// The height of output image
    pub height: u32,

    /// The aspect ratio of output image
    pub aspect_ratio: f32,

    /// The path to save image
    pub output_path: PathBuf,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RenderConfig {
    #[serde(default = "default_samples_per_pixel", alias = "samples_per_pixel")]
    /// The number of samples per pixel which affects the quality of image
    pub samples_per_pixel: u32,

    #[serde(default = "default_max_depth", alias = "depth", alias = "max_depth")]
    /// The maximum depth of ray tracing recursion
    pub max_depth: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL,
            max_depth: DEFAULT_MAX_DEPTH,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    /// The resolved image configuration
    pub image: ResolvedImage,

    /// The number of samples per pixel which affects the quality of image
    pub samples_per_pixel: u32,

    /// The maximum depth of ray tracing recursion
    pub max_depth: u32,
}

impl Config {
    /// Resolve the config into concrete values
    pub fn resolve(&self) -> ResolvedConfig {
        ResolvedConfig {
            image: self.image.resolved(),
            samples_per_pixel: self.render.samples_per_pixel,
            max_depth: self.render.max_depth,
        }
    }
}

fn default_width() -> u32 {
    DEFAULT_WIDTH
}

fn default_output_path() -> String {
    "output/image.ppm".to_string()
}

fn default_samples_per_pixel() -> u32 {
    DEFAULT_SAMPLES_PER_PIXEL
}

fn default_max_depth() -> u32 {
    DEFAULT_MAX_DEPTH
}

/// Load configuration from a TOML file
pub fn load_config(path_str: &str) -> Config {
    let path = Path::new(path_str);
    let content = fs::read_to_string(&path).expect("The config.toml is not exist.");
    toml::from_str(&content).unwrap()
}
