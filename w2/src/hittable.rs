use std::rc::Rc;

use aabb::AABB;

use crate::{
    material::Material,
    primitive::{interval::Interval, point3::Point3, ray::Ray, vec3::Vec3},
};

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64, // ray parameter
    pub u: f64, // texture coordinate
    pub v: f64, // texture coordinate
    pub front_face: bool,
}

impl HitRecord {
    fn new(p: Point3, ray: &Ray, outward_normal: Vec3, t: f64, material: Rc<dyn Material>) -> Self {
        let (front_face, normal) = Self::face_normal(ray, outward_normal);
        // TODO: properly calculate u/v

        Self {
            p,
            normal,
            material,
            t,
            u: 0.0,
            v: 0.0,
            front_face,
        }
    }
    fn face_normal(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = r.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, normal)
    }
    fn min(self, other: Option<Self>) -> Self {
        match other {
            None => self,
            Some(other) => {
                if self.t < other.t {
                    self
                } else {
                    other
                }
            }
        }
    }
}

pub trait Hittable: std::fmt::Debug {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
    // fn update_bounding_box(&self, time_range: Interval);
}

pub mod aabb;
pub mod bvh;
pub mod hittable_list;
pub mod sphere;
