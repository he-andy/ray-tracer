use crate::Vec3;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::vec::Vec;

pub struct Image {
    pub image: Vec<Vec3>,
    height: i32,
    width: i32,
}

impl Add for Image {
    type Output = Self;
    fn add(self, other: Image) -> Self {
        let mut v = Vec::with_capacity(self.image.len());
        for i in 0..self.image.len() {
            v.push(self.image[i] + other.image[i])
        }
        Image {
            image: v,
            height: self.height,
            width: self.width,
        }
    }
}

impl Image {
    pub fn new(height: i32, width: i32) -> Self {
        let image = Vec::with_capacity(height as usize * width as usize);
        Image {
            image,
            height,
            width,
        }
    }

    pub fn blank(height: i32, width: i32) -> Self {
        let image = vec![Vec3::zero(); height as usize * width as usize];
        Image {
            image,
            height,
            width,
        }
    }

    pub fn push(&mut self, pixel_color: Vec3) {
        self.image.push(pixel_color);
    }

    pub fn gamma_correction(&self, g: f64) -> Self {
        let p = 1.0 / g;
        Image {
            image: self.image.iter().map(|x| x.pow(p)).collect(),
            height: self.height,
            width: self.width,
        }
    }
    pub fn clamp(&self) -> Self {
        Image {
            image: self.image.iter().map(|x| x.clamp()).collect(),
            height: self.height,
            width: self.width,
        }
    }

    pub fn scale(&self, k: f64) -> Self {
        Image {
            image: self.image.iter().map(|x| k * *x).collect(),
            height: self.height,
            width: self.width,
        }
    }

    pub fn save(&self) {
        let mut file = File::create("out.ppm").unwrap();
        writeln!(&mut file, "P3\n{} {}\n255", self.width, self.height).unwrap();
        for pixel in &self.image {
            Self::write_color(&mut file, pixel);
        }
    }

    fn write_color(file: &mut File, color: &Vec3) {
        let ir = (255.999 * color.x) as i32;
        let ig = (255.999 * color.y) as i32;
        let ib = (255.999 * color.z) as i32;
        writeln!(file, "{ir} {ig} {ib} ").unwrap();
    }
}
