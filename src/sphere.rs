use crate::hittable::{HitRecord, Hittable};
use crate::materials::*;
use crate::{Point, Ray, AABB};

pub struct Sphere<M: Mat> {
    pub center: Point,
    pub radius: f64,
    pub material: M,
}

impl<M: Mat> Sphere<M> {
    pub fn new(center: Point, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn get_uv(p: &Point) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + crate::PI;

        (phi / (2.0 * crate::PI), theta / crate::PI)
    }
}

impl<M: Mat> Hittable for Sphere<M> {
    //#[timed::timed(tracing(enabled = true), duration(disabled = true))]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.dir.l2();
        let half_b = r.dir.dot(&oc);
        let c = oc.l2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        //finds root within t_min and t_max
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let normal = (p - self.center) / self.radius;
        Some(HitRecord::new(
            p,
            t,
            normal,
            &r,
            &self.material,
            &Self::get_uv,
        ))
    }

    fn bounding_box(&self) -> AABB {
        AABB {
            min: self.center - Point::new(self.radius, self.radius, self.radius),
            max: self.center + Point::new(self.radius, self.radius, self.radius),
        }
    }
}
