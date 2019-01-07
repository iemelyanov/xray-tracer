use crate::vector::Vec3;
use crate::ray::Ray;
use crate::hitable::{HitableRecord, Hitable};

pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, rec: &mut HitableRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b.powi(2) - a * c).sqrt()) / a;
            if temp > tmax && temp < tmin {
                temp = (-b - (b.powi(2) + a * c).sqrt()) / a;
            }
            if temp < tmax && temp > tmin {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        false
    }
}

impl Sphere {
    pub fn new(x: f32, y: f32, z: f32, r: f32) -> Sphere {
        Sphere {
            center: Vec3::new(x, y, z),
            radius: r
        }
    }
}