use indicatif::{ProgressBar, ProgressStyle};
use simple_rpt::camera::Camera;
use simple_rpt::config::load_config;
use simple_rpt::math::{Color, ColorExt, Ray, write_color};
use simple_rpt::object::hittable::{HitRecord, Hitable};
use simple_rpt::object::sphere::Sphere;

fn ray_color(r: &Ray, obj: &Sphere) -> Color {
    let mut rec = HitRecord::new();
    if obj.hit(r, 0.0, std::f32::INFINITY, &mut rec) {
        return Color::rgb(255.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().normalize();
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::rgb(1.0, 1.0, 1.0) + t * Color::rgb(0.2, 0.5, 1.0)
}

fn main() {
    // Image
    let config = load_config("config.toml");
    let aspect_ratio = config.aspect_ratio();
    let image_width = config.image_width();
    let image_height = config.image_height();

    // Camera
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio as f32 * viewport_height;
    let cm = Camera::new([0.0, 0.0, 0.0], 1.0, viewport_height, viewport_width);

    // World
    let sp = Sphere::new([0.5, 0.5, -1.0], 0.5);

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    let pb = ProgressBar::new(image_height as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );

    for row in (0..image_height).rev() {
        for col in 0..image_width {
            let u = col as f32 / (image_width - 1) as f32;
            let v = row as f32 / (image_height - 1) as f32;
            let r = Ray::new(
                cm.origin(),
                cm.lower_left_corner() + u * cm.horizontal_vector() + v * cm.vertical_vector()
                    - cm.origin(),
            );
            let pixel_color = ray_color(&r, &sp);
            write_color(&mut std::io::stdout(), &pixel_color);
        }
        pb.inc(1);
    }
    pb.finish_with_message("Done!");
}
