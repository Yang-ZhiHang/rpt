use glam::Vec3A;

// Offer more friendly alias
pub type Vec3 = Vec3A;
pub type Point3 = Vec3A;
pub type Color = Vec3A;

pub trait ColorExt {
    fn rgb(r: f32, g: f32, b: f32) -> Self;
    fn r(&self) -> f32;
    fn g(&self) -> f32;
    fn b(&self) -> f32;
}

impl ColorExt for Color {
    #[inline]
    fn rgb(r: f32, g: f32, b: f32) -> Self {
        Vec3A::new(r, g, b)
    }

    #[inline]
    fn r(&self) -> f32 {
        self.x
    }

    #[inline]
    fn g(&self) -> f32 {
        self.y
    }

    #[inline]
    fn b(&self) -> f32 {
        self.z
    }
}

pub fn write_color(io: &mut impl std::io::Write, color: &Color) {
    let ir = (color.r().clamp(0.0, 0.999) * 255.99) as u8;
    let ig = (color.g().clamp(0.0, 0.999) * 255.99) as u8;
    let ib = (color.b().clamp(0.0, 0.999) * 255.99) as u8;
    io.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())
        .expect("Failed to write color");
}

#[derive(Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Point3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Point3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
