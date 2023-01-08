mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod image;
mod bvh;

pub mod materials;

pub use vec3::{Vec3, Color, Point};
pub use ray::Ray;
pub use hittable::{Hittable, HitRecord, HittableList};
pub use sphere::Sphere;
pub use camera::Camera;
pub use std::f64::INFINITY;
pub use std::f64::consts::PI;
pub use image::{Image};
pub use bvh::{BVH, AABB};


use rand::Rng;
pub fn rand() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}