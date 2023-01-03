use crate::{Ray, Color, Vec3, Point};

pub trait Mat{
    //returns Some of Color (attenuation) and Ray (scatter dir) or None
    fn scatter(&self, r_in: &Ray, p: &Point, normal: &Vec3) -> Option<(Color, Ray)>; 
}

pub struct Lambertian{
    pub albedo: Vec3,
}

impl Lambertian{
    pub fn new(albedo: Vec3) -> Self{
        Self{albedo}
    }
}

impl Mat for Lambertian{
    fn scatter(&self, _r_in: &Ray, p: &Point, normal: &Vec3) -> Option<(Color, Ray)>{
        let mut scatter_dir = *normal + Vec3::rand_within_unit_sphere().unit();
        //fix degenerate case
        if scatter_dir.near_zero(){
            scatter_dir = *normal
        }
        let scattered = Ray::new(*p, scatter_dir);
        Some((self.albedo, scattered))
    }
}