use std::rc::Rc;

use crate::primitive::{interval::Interval, ray::Ray};

use super::{
    aabb::{AABB, EMPTY_AABB},
    HitRecord, Hittable,
};

#[derive(Debug)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: EMPTY_AABB,
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        if self.bbox == EMPTY_AABB {
            self.bbox = object.bounding_box();
        } else {
            self.bbox = AABB::surrounding_box(&self.bbox, &object.bounding_box());
        }
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        self.objects
            .iter()
            .fold((None, ray_t.end), |(ret, mut closest_so_far), object| {
                if let Some(rec) = object.hit(
                    r,
                    Interval {
                        start: ray_t.start,
                        end: closest_so_far,
                    },
                ) {
                    if rec.t < closest_so_far {
                        closest_so_far = rec.t;
                        return (Some(rec), closest_so_far);
                    }
                    (ret, closest_so_far)
                } else {
                    (ret, closest_so_far)
                }
            })
            .0
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
