use std::fs::File;
use std::io::Write;
use std::error::Error;

mod vec3;
mod ray;

pub use vec3::{Vec3, Color, Point};
pub use ray::Ray;

pub fn write_color(file: &mut File, color: Vec3) -> Result<(), Box<dyn Error>>{
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;
    writeln!(file, "{ir} {ig} {ib} ")?;
    Ok(())
}

