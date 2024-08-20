use core::f64;
use std::rc::Rc;

use crate::{
    material::{isotropic::Isotropic, Material},
    primitive::{
        color::Color,
        interval::{Interval, MAX_INTERVAL},
        ray::Ray,
        vec3::Vec3,
    },
};

use super::{aabb::AABB, HitRecord, Hittable};

#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    phase_function: Rc<dyn Material>,
    neg_inv_density: f64, // density kept as -1/d for optimization
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, density: f64, phase_function: Rc<dyn Material>) -> Self {
        Self {
            boundary,
            phase_function,
            neg_inv_density: -1.0 / density,
        }
    }
    pub fn new_from_color(boundary: Rc<dyn Hittable>, density: f64, color: Color) -> Self {
        Self {
            boundary,
            phase_function: Rc::new(Isotropic::new(color)),
            neg_inv_density: -1.0 / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // get 2 hit records - entry, exit
        let mut rec1 = self.boundary.hit(r, MAX_INTERVAL)?;
        let mut rec2 = self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, f64::MAX))?;

        // clamp the intervals
        if rec1.t < ray_t.start {
            rec1.t = ray_t.start;
        }
        if rec2.t > ray_t.end {
            rec2.t = ray_t.end;
        }

        // degenerate case
        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0. {
            rec1.t = 0.;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::ln(rand::random::<f64>());

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let true_t = rec1.t + hit_distance / ray_length;

        Some(HitRecord::new(
            r.at(true_t),
            r,
            Vec3::new(1., 0., 0.), // arbitrary normal
            true_t,
            0.,
            0.,
            self.phase_function.clone(),
        ))
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
