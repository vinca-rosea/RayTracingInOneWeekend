use super::*;
use std::sync::Arc;

pub struct HitableList {
    pub objects: Vec<Arc<dyn Hitable + Send + Sync>>,
}

impl HitableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hitable + Send + Sync>) {
        self.objects.push(object);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        self.objects.iter().for_each(|object| {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.assign(&temp_rec);
            }
        });
        hit_anything
    }
}

unsafe impl Send for HitableList {}
unsafe impl Sync for HitableList {}