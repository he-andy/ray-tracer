//module for implementing bounding volume hierarchies
use crate::{HitRecord, Hittable, HittableList, Point, Ray, Vec3};
use std::mem;

pub enum BVHContents {
    Node { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>),
}
pub struct BVH {
    pub contents: BVHContents,
    bounding_box: AABB,
}

impl Hittable for BVH {
    //#[timed::timed(tracing(enabled = true), duration(disabled = true))]
    fn hit(&self, r: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        if self.bounding_box().hit(r, t_min, t_max) {
            match &self.contents {
                BVHContents::Leaf(obj) => obj.hit(r, t_min, t_max),
                BVHContents::Node { left, right } => {
                    let left = left.hit(r, t_min, t_max);
                    if let Some(HitRecord { t, .. }) = &left {
                        t_max = *t;
                    }
                    let right = right.hit(r, t_min, t_max);
                    if right.is_some() {
                        right
                    } else {
                        left
                    }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}

impl BVH {
    pub fn new(mut objs: Vec<Box<dyn Hittable>>) -> Self {
        fn axis_range(objs: &Vec<Box<dyn Hittable>>, dim: i32) -> f64 {
            let range = objs.iter().fold(f64::MAX..f64::MIN, |acc, x| {
                let bb = x.bounding_box();
                f64::min(acc.start, bb.min.get(dim))..f64::min(acc.end, bb.max.get(dim))
            });
            range.start - range.end
        }

        match objs.len() {
            0 => panic!("cannot create bvh from empty list"),
            1 => BVH {
                bounding_box: objs[0].bounding_box(),
                contents: BVHContents::Leaf(objs.pop().unwrap()),
            },
            _ => {
                //select axis to split on by max range
                let axis = (0..3).into_iter().fold(0, |acc, x| {
                    if axis_range(&objs, x) > axis_range(&objs, acc) {
                        x
                    } else {
                        acc
                    }
                });

                objs.sort_unstable_by(|a, b| {
                    let abb = a.bounding_box();
                    let bbb = b.bounding_box();
                    let abb_centroid = abb.min.get(axis) + abb.max.get(axis);
                    let bbb_centroid = bbb.min.get(axis) + bbb.max.get(axis);
                    abb_centroid.partial_cmp(&bbb_centroid).unwrap()
                });

                let right = Box::new(BVH::new(objs.drain(objs.len() / 2..).collect()));

                let left = Box::new(BVH::new(objs));

                BVH {
                    bounding_box: AABB::merge(&left.bounding_box(), &right.bounding_box()),
                    contents: BVHContents::Node { left, right },
                }
            }
        }
    }

    pub fn from_hittable_list(h: HittableList) -> BVH {
        BVH::new(h.list)
    }

    pub fn height(&self) -> i32 {
        match &self.contents {
            BVHContents::Leaf(..) => 1,
            BVHContents::Node { left, right } => left.height() + right.height() + 1,
        }
    }
}
//Axis-Aligned Bounding Boxes
#[derive(Copy, Clone, Debug, Default)]
pub struct AABB {
    pub min: Point,
    pub max: Point,
}

impl AABB {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.dir.get(a);
            let mut t0 = (self.min.get(a) - r.origin.get(a)) * inv_d;
            let mut t1 = (self.max.get(a) - r.origin.get(a)) * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t1, &mut t0);
            }

            let t_min = f64::max(t_min, t0);
            let t_max = f64::min(t_max, t1);

            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn expand(&mut self, other: &AABB) {
        self.min = Vec3::min(&self.min, &other.min);
        self.max = Vec3::max(&self.min, &other.min);
    }

    pub fn merge(bb1: &AABB, bb2: &AABB) -> AABB {
        AABB {
            min: Vec3::min(&bb1.min, &bb2.min),
            max: Vec3::max(&bb1.max, &bb2.max),
        }
    }
}
