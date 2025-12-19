use glam::Vec3A;
use simple_rpt::color;
use simple_rpt::color::Color;
use simple_rpt::config::load_config;
use simple_rpt::ray::Ray;
use simple_rpt::{camera::Camera, color::ColorExt};
use std::{thread::sleep, time::Duration};

#[allow(dead_code, unused, non_snake_case)]
fn hit_sphere(center: [f32; 3], radius: f32, r: &Ray) -> bool {
    let [x, y, z] = center;
    // dst = A - C
    let dst = r.origin() - Vec3A::new(x, y, z);
    let a = r.direction().dot(r.direction());
    let b = 2.0 * r.direction().dot(dst);
    let c = (dst).dot(dst) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere([0.0, 0.0, 0.0], 0.5, r) {
        return Color::rgb(255.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().normalize();
    let t: f32 = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let config = load_config("../config.toml");
    let aspect_ratio = config.aspect_ratio();
    let image_width = config.image_width();
    let image_height = config.image_height();

    // Camera
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect_ratio as f32 * viewport_height;
    let cm = Camera::new([0.0, 0.0, 0.0], 1.0, viewport_height, viewport_width);

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for row in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", row);
        sleep(Duration::from_millis(1));
        for col in 0..image_width {
            let u = col as f32 / (image_width - 1) as f32;
            let v = row as f32 / (image_height - 1) as f32;
            let r = Ray::new(
                cm.origin(),
                cm.lower_left_corner() + u * cm.horizontal_vector() + v * cm.vertical_vector()
                    - cm.origin(),
            );
            let pixel_color = ray_color(&r);
            color::write_color(&mut std::io::stdout(), &pixel_color);
        }
    }
    eprint!("\nDone.\n");
}
