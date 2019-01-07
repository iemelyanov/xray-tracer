use crate::ray::Ray;
use crate::hitable::{Hitable, HitableRecord};

pub struct World {
    objects: Vec<Box<Hitable>>
}

impl Hitable for World {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &mut HitableRecord) -> bool {
        let mut temp_rec = HitableRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = tmax;
        for obj in self.objects.iter() {
            if obj.hit(r, tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec!()
        }
    }

    pub fn add<H: Hitable + 'static>(&mut self, object: H) {
        self.objects.push(Box::new(object));
    }
}