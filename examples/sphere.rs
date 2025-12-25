use indicatif::{ProgressBar, ProgressStyle};
use simple_rpt::camera::Camera;
use simple_rpt::config::load_config;
use simple_rpt::material::dieletrics::Dielectric;
use simple_rpt::material::lambertian::Lambertian;
use simple_rpt::material::metal::Metal;
use simple_rpt::math::{Color, ColorExt};
use simple_rpt::object::Object;
use simple_rpt::renderer::Renderer;
use simple_rpt::scene::Scene;
use simple_rpt::shape::sphere::Sphere;

fn main() {
    // Image
    let config = load_config("config.toml");
    let aspect_ratio = config.aspect_ratio();
    let image_width = config.image_width();
    let image_height = config.image_height();
    let samples_per_pixel = config.samples_per_pixel;
    let depth = config.depth;

    // Camera
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio as f32 * viewport_height;
    let cam = Camera::new([0.0, 0.0, 0.0], 1.0, viewport_height, viewport_width);

    // Scene
    let sp_lambert = Object::new(Sphere::from_array([0.0, 0.0, -1.0], 0.5))
        .material(Lambertian::new(Color::rgb(0.8, 0.1, 0.1)));
    let sp_metal = Object::new(Sphere::from_array([1.0, 0.0, -1.0], 0.5))
        .material(Metal::new(Color::splat(0.8), 0.8));
    let sp_dieletrics_outer =
        Object::new(Sphere::from_array([-1.0, 0.0, -1.0], 0.5)).material(Dielectric::new(1.8));
    let sp_dieletrics_inner =
        Object::new(Sphere::from_array([-1.0, 0.0, -1.0], -0.4)).material(Dielectric::new(1.8));
    let ground = Object::new(Sphere::from_array([0.0, -100.5, -1.0], 100.0))
        .material(Lambertian::new(Color::rgb(0.1, 0.8, 0.1)));
    let mut scene = Scene::new();
    scene.add_list([
        sp_lambert,
        sp_metal,
        sp_dieletrics_outer,
        sp_dieletrics_inner,
        ground,
    ]);

    // Render
    let pb = ProgressBar::new(image_height as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );
    let r = Renderer::new(cam, scene)
        .width(image_width)
        .height(image_height)
        .progress_bar(pb);
    r.render(image_width, image_height, samples_per_pixel, depth);
}
