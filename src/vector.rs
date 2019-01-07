use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

#[derive(PartialEq, Copy, Clone)]
pub struct Vec3 (f32, f32, f32);

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.1)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {

    #[inline(always)]
    fn add_assign(&mut self, other: Vec3) {
        *self = *self + other;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vec3 {

    #[inline(always)]
    fn sub_assign(&mut self, other: Vec3) {
        *self = *self - other;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, t: f32) -> Vec3 {
        Vec3(self.0 * t, self.1 * t, self.2 * t)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3(self * v.0, self * v.1, self * v.2)
    }
}

impl MulAssign<Vec3> for Vec3 {

    #[inline(always)]
    fn mul_assign(&mut self, other: Vec3) {
        *self = *self * other;
    }
}

impl MulAssign<f32> for Vec3 {

    #[inline(always)]
    fn mul_assign(&mut self, t: f32) {
        *self = *self * t;
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, other: Vec3) -> Vec3 {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, t: f32) -> Vec3 {
        Vec3(self.0 / t, self.1 / t, self.2 / t)
    }
}

impl DivAssign<Vec3> for Vec3 {

    #[inline(always)]
    fn div_assign(&mut self, other: Vec3) {
        *self = *self / other;
    }
}

impl DivAssign<f32> for Vec3 {

    #[inline(always)]
    fn div_assign(&mut self, t: f32) {
        let k = 1.0 / t;
        *self = *self * k;
    }
}

impl Vec3 {
    #[inline(always)]
    pub fn new(a: f32, b: f32, c: f32) -> Vec3 {
        Vec3(a, b, c)
    }

    #[inline(always)]
    pub fn x(&self) -> f32 { self.0 }

    #[inline(always)]
    pub fn y(&self) -> f32 { self.1 }

    #[inline(always)]
    pub fn z(&self) -> f32 { self.2 }

    #[inline(always)]
    pub fn r(&self) -> f32 { self.0 }

    #[inline(always)]
    pub fn g(&self) -> f32 { self.1 }

    #[inline(always)]
    pub fn b(&self) -> f32 { self.2 }

    #[inline(always)]
    pub fn length(&self) -> f32 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2)).sqrt()
    }

    #[inline(always)]
    pub fn squared_length(&self) -> f32 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    #[inline(always)]
    pub fn make_unit_vec(&mut self) {
        let k: f32 = 1.0 / self.length();
        self.0 *= k;
        self.1 *= k;
        self.2 *= k;
    }

    #[inline(always)]
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
    }

    #[inline(always)]
    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3(
            v1.1 * v2.2 - v1.2 * v2.1,
            -(v1.0 * v2.2 - v1.2 * v2.0),
            v1.0 * v2.1 - v1.1 * v2.0
        )
    }

    #[inline(always)]
    pub fn unit_vec(v: Vec3) -> Vec3 {
        v / v.length()
    }
}