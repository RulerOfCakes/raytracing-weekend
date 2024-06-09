use std::rc::Rc;

use crate::primitive::{interval::Interval, ray::Ray};

use super::{aabb::AABB, hittable_list::HittableList, HitRecord, Hittable};

#[derive(Debug)]
struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl From<HittableList> for BVHNode {
    fn from(list: HittableList) -> Self {
        unimplemented!()
    }
}

impl From<&[Rc<dyn Hittable>]> for BVHNode {
    fn from(list: &[Rc<dyn Hittable>]) -> Self {
        unimplemented!()
    }
}

impl BVHNode {
    pub fn new(left: Rc<dyn Hittable>, right: Rc<dyn Hittable>) -> Self {
        let bounding_box = AABB::surrounding_box(&left.bounding_box(), &right.bounding_box());
        Self {
            left,
            right,
            bbox: bounding_box,
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, mut ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }
        let mut return_rec: Option<HitRecord> = None;
        let hit_left = self.left.hit(r, ray_t);

        if let Some(rec) = hit_left {
            ray_t.end = rec.t;
            return_rec = Some(rec);
        }

        let hit_right = self.right.hit(r, ray_t);

        if let Some(rec) = return_rec {
            return_rec = Some(rec.min(hit_right));
        }

        return_rec
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
