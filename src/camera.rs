use crate::*;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::iter::{ParallelIterator};
use rayon::prelude::*;

const MAX_DEPTH: i32 = 50;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    aspect_ratio: f64,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        vfov: f64,
        aspect_ratio: f64,
        lookfrom: &Point,
        lookat: &Point,
        vup: &Vec3,
        focal_dist: f64,
        aperture: f64,
    ) -> Self {
        let w = (*lookfrom - *lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let theta = vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = lookfrom.clone();
        let horizontal = focal_dist * viewport_width * u;
        let vertical = focal_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focal_dist * w;

        Self {
            origin: *lookfrom,
            horizontal,
            vertical,
            lower_left_corner,
            aspect_ratio,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }

    fn ray_cast(&self, i: i32, j: i32, height: i32, width: i32, world: &dyn Hittable) -> Color {
        let u = (i as f64 + rand()) / (width - 1) as f64;
        let v = (j as f64 + rand()) / (height - 1) as f64;

        let r = self.get_ray(u, v);
        ray_color(r, world, MAX_DEPTH)
    }

    pub fn render(&self, height: i32, world: &dyn Hittable, n_samples: i32) {
        let width = (height as f64 * self.aspect_ratio) as i32;
        let dims = (height, width);

        let style = ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:100.cyan/blue} {pos:>7}/{len:7} ({eta_precise})",
        )
        .unwrap()
        .progress_chars("##-");

        let res: Vec<Image> = (0..n_samples)
            .into_par_iter()
            .progress_with_style(style)
            .map(|_| self.render_helper(dims, world))
            .collect();
        let img = res
            .into_iter()
            .fold(Image::blank(height, width), |acc, x| acc + x)
            .scale(1.0 / n_samples as f64)
            .clamp()
            .gamma_correction(2.0);
        img.save();
    }

    pub fn render_helper(&self, dims: (i32, i32), world: &dyn Hittable) -> Image {
        let (height, width) = dims;
        let mut img = Image::new(height, width);

        for j in (0..height).rev() {
            for i in 0..width {
                let pixel_color = self.ray_cast(i, j, height, width, world);
                img.push(pixel_color);
            }
        }
        img
    }
}

//Rendering
fn ray_color(r: Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth > 0 {
        match world.hit(&r, 0.001, INFINITY) {
            HitRecord::Hit {
                normal,
                p,
                material,
                front_face,
                ..
            } => match material.scatter(r, &p, &normal, front_face) {
                Some((attenuation, r_out)) => attenuation * ray_color(r_out, world, depth - 1),
                None => Color::zero(),
            },
            HitRecord::Miss => {
                let unit_dir = r.dir.unit();
                let t = 0.5 * (unit_dir.y + 1.0);
                Color::new(0.5, 0.7, 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + (t * Color::new(0.5, 0.7, 1.0))
            }
        }
    } else {
        Color::zero()
    }
}
