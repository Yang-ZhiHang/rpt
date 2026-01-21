use crate::math::Vec3;

///  Ortho-Normal Basis
pub struct ONB {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl ONB {
    /// Create a new `ONB` struct from given z-axis vector.
    pub fn new(vec: Vec3) -> Self {
        let w = vec.normalize();
        let r = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(r).normalize();
        let u = w.cross(v);
        Self { u, v, w }
    }

    /// Transform the coordinates of vec to `ONB`'s coordinates.
    pub fn transform(&self, vec: Vec3) -> Vec3 {
        vec.x * self.u + vec.y * self.v + vec.z * self.w
    }
}
