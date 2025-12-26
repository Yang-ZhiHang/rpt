use crate::math::{Point3, Ray, Vec3, Vec3Ext};

pub struct Camera {
    /// The original point of camera
    pub origin: Point3,

    /// Width of viewport
    pub horizontal: Vec3,

    /// Height of viewport
    pub vertical: Vec3,

    /// The bottom left corner of viewport which centered on the origin
    pub lower_left: Point3,

    pub u: Vec3,
    pub v: Vec3,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3, // View up vector
        vfov: f32, // Vertical field-of-view in degrees
        aspect_ratio: f32,
        aperture: f32,
        focal_length: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        // Pixel coordinates
        let w: Vec3 = (look_from - look_at).normalize();
        let u: Vec3 = vup.cross(w).normalize();
        let v: Vec3 = w.cross(u);

        let horizontal: Vec3 = focal_length * viewport_width * u;
        let vertical: Vec3 = focal_length * viewport_height * v;
        let lower_left: Point3 = look_from - horizontal / 2.0 - vertical / 2.0 - w * focal_length;
        let lens_radius = aperture / 2.0;

        Self {
            origin: look_from,
            horizontal,
            vertical,
            lower_left,
            u,
            v,
            lens_radius,
        }
    }

    /// Obtain the ray direction of the pixel coordinate uv from the aperture
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let r = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * r.x + self.v * r.y;
        Ray::new(
            self.origin + offset,
            self.lower_left + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
