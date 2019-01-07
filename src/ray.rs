use crate::vector::Vec3;

pub struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {
    pub fn new(a: &Vec3, b: &Vec3) -> Ray {
        Ray {
            a: *a,
            b: *b
        }
    }

    #[inline(always)]
    pub fn origin(&self) -> Vec3 {
        self.a
    }

    #[inline(always)]
    pub fn direction(&self) -> Vec3 {
        self.b
    }

    #[inline(always)]
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + self.b * t
    }
}