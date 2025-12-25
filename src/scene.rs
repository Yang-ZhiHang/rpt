use crate::{math::Ray, object::Object, shape::HitRecord};

#[derive(Default)]
pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    /// Create a empty Object list for Scene
    pub fn new() -> Self {
        Default::default()
    }

    /// Add a Object to Scene
    pub fn add(&mut self, obj: Object) {
        self.objects.push(obj);
    }

    /// Add a list of Object to Scene
    pub fn add_list<I>(&mut self, obj_list: I)
    where
        I: IntoIterator<Item = Object>,
    {
        self.objects.extend(obj_list);
    }

    pub fn get_closest_intersect(
        &self,
        r: &Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut HitRecord,
    ) -> bool {
        let mut obj_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in &self.objects {
            obj_rec.material = Some(obj.material.clone());
            if obj.shape.intersect(r, t_min, closest_so_far, &mut obj_rec) {
                hit_anything = true;
                closest_so_far = obj_rec.t;
                *rec = obj_rec.clone();
            }
        }
        hit_anything
    }
}
