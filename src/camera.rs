use crate::*;
use std::error::Error;
use std::fs::File;
use std::io::Write;

const MAX_DEPTH: i32= 50;

pub struct Camera{
    origin: Point,
    horizontal: Vec3, 
    vertical: Vec3,
    lower_left_corner: Vec3,
    aspect_ratio: f64,
}

impl Camera{
    pub fn new(viewport_height: f64, aspect_ratio: f64, focal_length: f64) -> Self{
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Point::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        Self{ 
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length),
            aspect_ratio
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray{
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }

    fn ray_cast(&self, i: i32, j: i32, height: i32, width: i32, world: &HittableList, n_samples: i32) -> Color{
        let mut color = Color::zero();
        for _ in 0..n_samples{
            let u = (i as f64 + rand()) / (width - 1) as f64;
            let v = (j as f64 + rand()) / (height - 1) as f64;
    
            let r = self.get_ray(u, v);
            color += ray_color(&r, world, MAX_DEPTH)
        }
        (color / n_samples as f64).clamp()
    }

    pub fn render(&self, height: i32, world: &HittableList, n_samples: i32) -> Result<(), Box<dyn Error>>{
        let mut file = File::create("out.ppm")?;
        let width = (height as f64 * self.aspect_ratio) as i32;

        writeln!(&mut file, "P3\n{width} {height}\n255")?;
        for j in (0..height).rev(){
            eprint!("\rScanlines remaining: {j}");
            for i in 0..width{
                let pixel_color = self.ray_cast(i, j, height, width, world, n_samples);
                let gamma_corrected = pixel_color.sqrt();
                write_color(&mut file, gamma_corrected)?;
            }
        }
        Ok(())
    }
}

//Rendering
fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth > 0 {
        match world.hit(r, 0.001, INFINITY) {
            HitRecord::Hit {normal, p, material, ..} => {
                //Lambertian Distribution Approx
                match material.scatter(r, &p, &normal) {
                    Some((attenuation, r_out)) => {
                        attenuation * ray_color(&r_out, world, depth -1)
                    }
                    None => Color::zero()
                }
            }
            HitRecord::Miss =>{
                let unit_dir = r.dir.unit();
                let t = 0.5 *(unit_dir.y + 1.0);
                Color::new(0.5, 0.7, 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + (t * Color::new(0.5, 0.7, 1.0))
            }
        }
    }
    else {
        Color::zero()
    }
}

fn write_color(file: &mut File, color: Vec3) -> Result<(), Box<dyn Error>>{
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;
    writeln!(file, "{ir} {ig} {ib} ")?;
    Ok(())
}