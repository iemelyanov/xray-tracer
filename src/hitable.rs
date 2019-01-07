use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Copy, Clone)]
pub struct HitableRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

pub trait Hitable {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &mut HitableRecord) -> bool;
}

impl HitableRecord {
    pub fn new() -> Self {
        HitableRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}