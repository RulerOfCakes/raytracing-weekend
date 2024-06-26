use std::rc::Rc;

use crate::{
    material::Material,
    primitive::{interval::Interval, point3::Point3, ray::Ray, vec3::Vec3},
};

use super::{aabb::AABB, HitRecord, Hittable};

#[derive(Debug)]
pub struct Sphere {
    center0: Point3,
    radius: f64,
    material: Rc<dyn Material>,
    velocity: Vec3,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center0: center,
            radius: radius.max(0.0),
            material,
            velocity: Vec3::zero(),
            bbox: AABB::new_from_points(
                center - Vec3::new(radius, radius, radius),
                center + Vec3::new(radius, radius, radius),
            ),
        }
    }

    /// Create a moving sphere.
    ///
    /// The sphere will move with a constant velocity.
    /// The velocity is the distance the sphere moves in one second.
    pub fn new_moving(
        center0: Vec3,
        radius: f64,
        material: Rc<dyn Material>,
        velocity: Vec3,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::new_from_points(center0 - rvec, center0 + rvec);
        let box2 = AABB::new_from_points(center0 + velocity - rvec, center0 + velocity + rvec);
        let bbox = AABB::surrounding_box(&box1, &box2);
        Self {
            center0,
            radius: radius.max(0.0),
            material,
            velocity,
            bbox,
        }
    }

    fn center(&self, time: f64) -> Point3 {
        self.center0 + self.velocity * time
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let center = self.center(r.time());

        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }
        let t = root;
        let p = r.at(t);
        let outward_normal = (p - center) / self.radius;
        Some(HitRecord::new(
            p,
            r,
            outward_normal,
            t,
            self.material.clone(),
        ))
    }
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
