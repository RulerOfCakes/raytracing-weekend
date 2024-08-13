use std::rc::Rc;

use crate::primitive::{interval::Interval, ray::Ray, vec3::Vec3};

use super::{aabb::AABB, HitRecord, Hittable};

#[derive(Debug)]
pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        if let Some(rec) = self.object.hit(&offset_r, ray_t) {
            let rec = rec.translate(self.offset);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
