use super::*;
use std::sync::Arc;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ptr: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Default::default(),
            normal: Default::default(),
            mat_ptr: Arc::new(UninitMaterial {}),
            t: Default::default(),
            front_face: Default::default(),
        }
    }
}

impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, mat_ptr: Arc<dyn Material>, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            mat_ptr,
            t,
            front_face,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal
        };
    }

    pub fn assign(&mut self, rec: &HitRecord) {
        self.p = rec.p.clone();
        self.normal = rec.normal.clone();
        self.mat_ptr = rec.mat_ptr.clone();
        self.t = rec.t;
        self.front_face = rec.front_face;
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
