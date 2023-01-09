mod bvh;
mod camera;
mod hittable;
mod image;
mod ray;
mod sphere;
mod vec3;

pub mod materials;
pub mod scenes;
pub mod texture;

pub use bvh::{AABB, BVH};
pub use camera::Camera;
pub use hittable::{HitRecord, Hittable, HittableList};
pub use image::Image;
pub use ray::Ray;
pub use sphere::Sphere;
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;
pub use vec3::{Color, Point, Vec3};

use rand::Rng;
pub fn rand() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
