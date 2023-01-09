use crate::materials::*;
use crate::{Point, Ray, Vec3, AABB};

pub struct HitRecord<'a> {
    pub normal: Vec3,
    pub p: Point,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: &'a dyn Mat,
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point,
        t: f64,
        outward_norm: Vec3,
        r: &Ray,
        material: &'a dyn Mat,
        uv: &dyn Fn(&Point) -> (f64, f64),
    ) -> HitRecord<'a> {
        let mut res = HitRecord {
            normal: Vec3::zero(),
            p,
            t,
            u: 0.0,
            v: 0.0,
            front_face: false,
            material,
        };
        res.set_face_normal(r, outward_norm);
        let (u, v) = uv(&res.normal);
        res.u = u;
        res.v = v;
        res
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_norm: Vec3) {
        match self {
            HitRecord {
                front_face, normal, ..
            } => {
                *front_face = r.dir.dot(&outward_norm) < 0.0;
                *normal = if *front_face {
                    outward_norm
                } else {
                    -outward_norm
                };
            }
        }
    }
}

#[derive(Default)]
pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, h: impl Hittable + 'static) {
        self.list.push(Box::new(h));
    }

    pub fn clear(&mut self) {
        self.list = Vec::new()
    }
}

impl Hittable for HittableList {
    //#[timed::timed(tracing(enabled = true), duration(disabled = true))]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut obj_hit = None;

        for obj in self.list.iter() {
            let res = obj.hit(r, t_min, closest_so_far);
            if let Some(HitRecord { t, .. }) = res {
                closest_so_far = t;
                obj_hit = res;
            }
        }

        obj_hit
    }

    fn bounding_box(&self) -> AABB {
        if self.list.is_empty() {
            return AABB::default();
        }

        let mut bounding_box = self.list[0].bounding_box();

        for obj in self.list[1..].iter() {
            bounding_box.expand(&obj.bounding_box());
        }

        bounding_box
    }
}
