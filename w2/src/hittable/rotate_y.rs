use std::rc::Rc;

use crate::primitive::{interval::Interval, ray::Ray, vec3::Vec3};

use super::{aabb::AABB, HitRecord, Hittable};

#[derive(Debug)]
pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = object.bounding_box();
        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max() + (1 - i) as f64 * bbox.x.min();
                    let y = j as f64 * bbox.y.max() + (1 - j) as f64 * bbox.y.min();
                    let z = k as f64 * bbox.z.max() + (1 - k) as f64 * bbox.z.min();

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min.set(c, min.get(c).min(tester.get(c)));
                        max.set(c, max.get(c).max(tester.get(c)));
                    }
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: AABB::new_from_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        // change the ray from world space to object space
        origin.x = self.cos_theta * r.origin().x - self.sin_theta * r.origin().z;
        origin.z = self.sin_theta * r.origin().x + self.cos_theta * r.origin().z;

        direction.x = self.cos_theta * r.direction().x - self.sin_theta * r.direction().z;
        direction.z = self.sin_theta * r.direction().x + self.cos_theta * r.direction().z;

        let rotated_r = Ray::new(origin, direction, r.time());

        if let Some(mut rec) = self.object.hit(&rotated_r, ray_t) {
            let mut p = rec.p;

            // change the hit point and normal from object space to world space
            p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
            p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

            let mut normal = rec.normal;
            normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
            normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

            rec.p = p;
            rec.normal = normal;

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
