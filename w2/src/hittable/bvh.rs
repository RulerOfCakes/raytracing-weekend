use std::rc::Rc;

use crate::primitive::{interval::Interval, ray::Ray};

use super::{
    aabb::{AABB, EMPTY_AABB},
    hittable_list::HittableList,
    HitRecord, Hittable,
};

#[derive(Debug)]
pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl From<HittableList> for BVHNode {
    fn from(list: HittableList) -> Self {
        Self::from(list.objects.as_slice())
    }
}

impl From<&[Rc<dyn Hittable>]> for BVHNode {
    fn from(list: &[Rc<dyn Hittable>]) -> Self {
        match list {
            [] => panic!("Empty hittable list passed to BVHNode::from()"),
            [a] => Self {
                left: a.clone(),
                right: a.clone(),
                bbox: a.bounding_box(),
            },
            [a, b] => {
                let left = a.clone();
                let right = b.clone();
                let bbox = AABB::surrounding_box(&left.bounding_box(), &right.bounding_box());
                Self { left, right, bbox }
            }
            _ => {
                let mut bbox = EMPTY_AABB;
                list.iter()
                    .for_each(|h| bbox = AABB::surrounding_box(&h.bounding_box(), &bbox));

                let axis = bbox.longest_axis();
                let mut sorted_list = list.to_vec();
                sorted_list.sort_by(|a, b| BVHNode::compare_on_axis(a, b, axis));

                let mid = sorted_list.len() / 2;
                let (left, right) = sorted_list.split_at(mid);

                let left = Rc::new(BVHNode::from(left));
                let right = Rc::new(BVHNode::from(right));

                Self { left, right, bbox }
            }
        }
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

    fn compare_on_axis(
        a: &Rc<dyn Hittable>,
        b: &Rc<dyn Hittable>,
        axis: usize,
    ) -> std::cmp::Ordering {
        let box_a = a.bounding_box();
        let box_b = b.bounding_box();

        box_a
            .axis_interval(axis)
            .start
            .partial_cmp(&box_b.axis_interval(axis).start)
            .unwrap()
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

        if let Some(rec) = hit_right {
            return_rec = Some(rec);
        }

        return_rec
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
