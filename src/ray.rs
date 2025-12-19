use glam::Vec3A;

#[derive(Default)]
pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
}

impl Ray {
    pub fn new(origin: Vec3A, direction: Vec3A) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3A {
        self.origin
    }

    pub fn direction(&self) -> Vec3A {
        self.direction
    }
}
