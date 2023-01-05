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
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64
}

impl Camera{
    pub fn new(vfov: f64, aspect_ratio: f64, lookfrom: &Point, lookat: &Point, vup: &Vec3, focal_dist: f64, aperture: f64) -> Self{
        let w = (*lookfrom - *lookat).unit();
        let u = vup.cross(&w);
        let v = w.cross(&u);

        let theta = vfov.to_radians();
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = lookfrom.clone();
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focal_dist * w;
        
        Self{ 
            origin: *lookfrom,
            horizontal,
            vertical,
            lower_left_corner,
            aspect_ratio,
            u,
            v,
            w,
            lens_radius: aperture/2.0
        }
    }

    fn get_ray(&self, s: f64, t: f64) -> Ray{
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(self.origin + offset, self.lower_left_corner + s * self.horizontal + t* self.vertical - self.origin - offset)
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
            HitRecord::Hit {normal, p, material, front_face, ..} => {
                match material.scatter(r, &p, &normal, front_face) {
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