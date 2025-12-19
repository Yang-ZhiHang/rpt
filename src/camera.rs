use glam::Vec3A;

pub struct Camera {
    origin: Vec3A,
    focal_length: f32,
    viewport_height: f32,
    viewport_width: f32,
}

impl Camera {
    pub fn new(
        origin: [f32; 3],
        focal_length: f32,
        viewport_height: f32,
        viewport_width: f32,
    ) -> Self {
        Self {
            origin: Vec3A::from_array(origin),
            focal_length,
            viewport_height,
            viewport_width,
        }
    }

    pub fn origin(&self) -> Vec3A {
        self.origin
    }

    pub fn focal_length(&self) -> f32 {
        self.focal_length
    }

    pub fn viewport_height(&self) -> f32 {
        self.viewport_height
    }

    pub fn viewport_width(&self) -> f32 {
        self.viewport_width
    }

    pub fn horizontal_vector(&self) -> Vec3A {
        Vec3A::new(self.viewport_width, 0.0, 0.0)
    }

    pub fn vertical_vector(&self) -> Vec3A {
        Vec3A::new(0.0, self.viewport_height, 0.0)
    }

    pub fn lower_left_corner(&self) -> Vec3A {
        self.origin
            - self.horizontal_vector() / 2.0
            - self.vertical_vector() / 2.0
            - Vec3A::new(0.0, 0.0, self.focal_length)
    }
}
