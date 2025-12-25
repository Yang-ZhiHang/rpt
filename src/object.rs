use crate::{
    material::{Material, lambertian::Lambertian},
    shape::Shape,
};
use std::sync::Arc;

pub struct Object {
    /// The shape of object
    pub shape: Arc<dyn Shape>,

    /// The material of object
    pub material: Arc<dyn Material>,
}

impl Object {
    pub fn new<T>(shape: T) -> Object
    where
        T: Shape + 'static,
    {
        Self {
            shape: Arc::new(shape),
            material: Arc::new(Lambertian::default()),
        }
    }

    pub fn material<T>(mut self, material: T) -> Object
    where
        T: Material + 'static,
    {
        self.material = Arc::new(material);
        self
    }
}
