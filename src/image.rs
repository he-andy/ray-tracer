use std::ops::{AddAssign};
use std::vec::Vec;
use crate::{Vec3, HittableList, Camera};



pub struct Image {
    image: Vec<Vec3>,
    height: i32
}

impl AddAssign<Vec<Vec3>> for Image{
    fn add_assign(&mut self, rhs: Vec<Vec3>) {
        for i in 0..self.image.len(){
            self.image[i] += rhs[i]
        }
    }
}

impl Image{
    fn new(height: i32, aspect_ratio: f64){

    }
}


pub struct Renderer {
    samples: i32,
    samples_remaining: i32,
    camera: Camera,
    scene: HittableList,
    image: Image
}

impl Iterator for Renderer {
    type Item = Image;
    fn next(&mut self) -> Option<Self::Item>{
        if self.samples_remaining > 0{
            self.camera.render(self.image.height, &self.scene, 1);
        }
        return None;
    }   
}