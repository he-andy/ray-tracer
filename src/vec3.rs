use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3{
    pub x: f64, 
    pub y: f64,
    pub z: f64
}

impl Vec3{
    pub fn new(x: f64, y: f64, z:f64) -> Self{
        Vec3{
            x, y, z
        }
    }
    pub fn zero() -> Self{
        Vec3{
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn length(&self) -> f64{
        self.l2().sqrt()
    }

    pub fn l2(&self) -> f64{
        self.dot(self)
    }

    pub fn scale(&self, scalar: f64) -> Self{
        Vec3 { x: scalar * self.x, y: scalar * self.y, z: scalar * self.z }
    }

    pub fn dot(&self, other: &Self) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self{
        Vec3{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn unit(&self) -> Self{
        let len = self.length();
        self.scale(1.0/len)
    }

}

impl Neg for Vec3{
    type Output = Self;
    fn neg(self) -> Self{
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Add for Vec3{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self.add(-other)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, f: f64) -> Vec3 {
        Vec3 {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, f: f64) -> Vec3 {
        Vec3 {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self * vec.x,
            y: self * vec.x,
            z: self * vec.x,
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self / vec.x,
            y: self / vec.x,
            z: self / vec.x,
        }
    }
}

impl Mul for Vec3{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for Vec3{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
impl AddAssign for Vec3{
    fn add_assign(&mut self, other: Self) {
        *self = Self{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.y + other.z,
        }
    }
}

impl SubAssign for Vec3{
    fn sub_assign(&mut self, other: Self) {
        *self = Self{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.y - other.z,
        }
    }
}

impl MulAssign for Vec3{
    fn mul_assign(&mut self, other: Self) {
        *self = Self{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.y + other.z,
        }
    }
}

impl DivAssign for Vec3{
    fn div_assign(&mut self, other: Self) {
        *self = Self{
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.y / other.z,
        }
    }
}

pub type Color = Vec3;
pub type Point = Vec3;
