use crate::rand_range;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn default() -> Self {
        Self::zero()
    }

    pub fn rand(min: f64, max: f64) -> Self {
        Vec3 {
            x: rand_range(min, max),
            y: rand_range(min, max),
            z: rand_range(min, max),
        }
    }

    pub fn rand_within_unit_sphere() -> Self {
        //rejection alg
        loop {
            let res = Self::rand(-1.0, 1.0);
            if res.l2() <= 1.0 {
                return res;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let mut res = Self::rand(-1.0, 1.0);
            res.z = 0.0;
            if res.l2() <= 1.0 {
                return res;
            }
        }
    }

    pub fn length(&self) -> f64 {
        self.l2().sqrt()
    }

    pub fn l2(&self) -> f64 {
        self.dot(self)
    }

    pub fn scale(&self, scalar: f64) -> Self {
        Vec3 {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit(&self) -> Self {
        let len = self.length();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn clamp(&self) -> Self {
        Vec3 {
            x: self.x.clamp(0.0, 0.9999),
            y: self.y.clamp(0.0, 0.9999),
            z: self.z.clamp(0.0, 0.9999),
        }
    }

    pub fn sqrt(&self) -> Self {
        Vec3 {
            x: self.x.sqrt(),
            y: self.y.sqrt(),
            z: self.z.sqrt(),
        }
    }

    pub fn pow(&self, p: f64) -> Self {
        Vec3 {
            x: self.x.powf(p),
            y: self.y.powf(p),
            z: self.z.powf(p),
        }
    }

    pub fn near_zero(&self) -> bool {
        let thresh = 1e-8;
        self.x.abs() < thresh && self.y.abs() < thresh && self.z.abs() < thresh
    }

    pub fn get(&self, i: i32) -> f64 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Out of Bounds"),
        }
    }

    pub fn min(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            x: f64::min(v1.x, v2.x),
            y: f64::min(v1.y, v2.y),
            z: f64::min(v1.z, v2.z),
        }
    }

    pub fn max(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            x: f64::max(v1.x, v2.x),
            y: f64::max(v1.y, v2.y),
            z: f64::max(v1.z, v2.z),
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
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
            y: self * vec.y,
            z: self * vec.z,
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: self / vec.x,
            y: self / vec.y,
            z: self / vec.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

pub type Color = Vec3;
pub type Point = Vec3;
