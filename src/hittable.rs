use crate::{Point, Vec3, Ray, Sphere};
use crate::materials::*;
use std::process;
use std::rc::Rc;

pub enum HitRecord{
    Hit {
        normal: Vec3,
        p: Point,
        t: f64,
        front_face: bool,
        material: Rc<dyn Mat>
    },
    Miss
}
pub trait Hittable{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitRecord;
}

impl HitRecord{
    pub fn new(p: Point, t: f64, outward_norm: Vec3, r: &Ray, material: Rc<dyn Mat>) -> HitRecord{
        let mut res = HitRecord::Hit { normal :Vec3::zero(), p, t, front_face: false, material};
        if let Result::Err(str) = res.set_face_normal(r, outward_norm){
            eprintln!("{str}");
            process::exit(1);
        };
        res
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_norm: Vec3) -> Result<(), &'static str>{
        match self {
            HitRecord::Hit {front_face, normal, ..}  => {
                *front_face = r.dir.dot(&outward_norm) < 0.0; 
                *normal = if *front_face{
                    outward_norm
                } else{
                    -outward_norm
                };
                Ok(())
            },
            HitRecord::Miss => Err("Cannot set face normal for HitRecord::Miss")
        }
    }
}


#[derive(Default)]
pub struct HittableList{
    pub list: Vec<Box<dyn Hittable>>
}

impl HittableList{
    pub fn add(&mut self, h: Box<dyn Hittable>){
        self.list.push(h);
    }

    pub fn add_sphere(&mut self, center: Point, r: f64, mat: Rc::<dyn Mat>) {
        self.list.push(
            Box::new(
                Sphere::new(
                    center,
                    r,
                    mat
                )
            )
        );
    }

    pub fn clear(&mut self){
        self.list = Vec::new()
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> HitRecord{
        let mut closest_so_far = t_max;
        let mut obj_hit = HitRecord::Miss;

        for obj in self.list.iter(){
            let res = obj.hit(r, t_min, closest_so_far);
            if let HitRecord::Hit{t, ..} = res{
                closest_so_far = t;
                obj_hit = res;
            }
        };

        obj_hit
    }
}