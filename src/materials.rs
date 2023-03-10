use crate::texture::Texture;
use crate::{Color, HitRecord, Ray, Vec3};

pub trait Mat: Sync {
    //returns Some of Color (attenuation) and Ray (scatter dir) or None
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

#[derive(Copy, Clone)]
pub struct Lambertian<T: Texture> {
    pub albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl<T: Texture> Mat for Lambertian<T> {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_dir = rec.normal + Vec3::rand_within_unit_sphere().unit();
        //fix degenerate case
        if scatter_dir.near_zero() {
            scatter_dir = rec.normal
        }
        let scattered = Ray::new(rec.p, scatter_dir);
        Some((self.albedo.value(rec.u, rec.v, &rec.p), scattered))
    }
}

//Metals
fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - (2.0 * v.dot(n) * *n)
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        Self {
            albedo: Vec3::new(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0)),
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }

    pub fn from_vec(albedo: Vec3, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Mat for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&r_in.dir.unit(), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::rand_within_unit_sphere(),
        );
        if scattered.dir.dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

//Dielectrics
fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min((-*uv).dot(n), 1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -((1.0 - r_out_perp.l2()).abs()).sqrt() * *n;
    r_out_perp + r_out_parallel
}

fn reflectance(cos: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Mat for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let att = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = r_in.dir.unit();

        let cos_theta = f64::min((-unit_dir).dot(&rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let dir = if cannot_refract || reflectance(cos_theta, refraction_ratio) > crate::rand() {
            reflect(&unit_dir, &rec.normal)
        } else {
            refract(&unit_dir, &rec.normal, refraction_ratio)
        };
        let scattered = Ray::new(rec.p, dir);
        Some((att, scattered))
    }
}
