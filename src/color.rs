use glam::Vec3A;

// Offer a more friendly alias for using color
pub type Color = Vec3A;

pub trait ColorExt {
    fn rgb(r: f32, g: f32, b: f32) -> Self;
    fn r(&self) -> f32;
    fn g(&self) -> f32;
    fn b(&self) -> f32;
}

impl ColorExt for Vec3A {
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
