use crate::hittable::{Hittable, HitRecord};
use crate::{Point, Ray};
use crate::materials::*;
use std::rc::Rc;

pub struct Sphere{
    pub center: Point,
    pub radius: f64,
    pub material: Rc<dyn Mat>
}

impl Sphere{
    pub fn new(center: Point, radius: f64, material: Rc<dyn Mat>) -> Self{
        Self { center, radius, material}
    }
}

impl Hittable for Sphere{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitRecord{
        let oc = r.origin - self.center;
        let a = r.dir.l2();
        let half_b = r.dir.dot(&oc);
        let c = oc.l2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return HitRecord::Miss;
        }
    
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) /a;
        //finds root within t_min and t_max
        if root < t_min || root > t_max{
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max{
                return HitRecord::Miss
            }
        }
        let t = root;
        let p = r.at(t);
        let normal = (p - self.center)/self.radius;
        
        HitRecord::new(p, t, normal, &r, self.material.clone())
    }   
}                                                                                       