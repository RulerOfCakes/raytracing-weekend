use std::rc::Rc;

use crate::{
    material::Material,
    primitive::{interval::Interval, point3::Point3, ray::Ray, vec3::Vec3},
};

use super::{aabb::AABB, hittable_list::HittableList, HitRecord, Hittable};

#[derive(Debug)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Rc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,  // d = normal * q
    w: Vec3, // w = n / (n * n), used for uv calculation
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let bbox = Quad::set_bounding_box(q, u, v);
        let n = u.cross(&v);
        let normal = n.unit();
        let d = normal.dot(&q);
        let w = n / n.length_squared();

        Self {
            q,
            u,
            v,
            mat,
            bbox,
            normal,
            d,
            w,
        }
    }
    fn set_bounding_box(q: Point3, u: Vec3, v: Vec3) -> AABB {
        let bbox_diagonal1 = AABB::new_from_points(q, q + u + v);
        let bbox_diagonal2 = AABB::new_from_points(q + u, q + v);
        AABB::surrounding_box(&bbox_diagonal1, &bbox_diagonal2)
    }
    fn get_uv(&self, p: Point3) -> (f64, f64) {
        let q = p - self.q;
        let u = self.w.dot(&q.cross(&self.v));
        let v = self.w.dot(&self.u.cross(&q));
        (u, v)
    }
    fn is_interior(&self, p: Point3) -> Option<(f64, f64)> {
        let (u, v) = self.get_uv(p);
        if (0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v) {
            Some((u, v))
        } else {
            None
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&r.direction());

        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(&r.origin())) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        let (u, v) = self.is_interior(intersection)?;

        let rec = HitRecord::new(intersection, r, self.normal, t, u, v, self.mat.clone());

        Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub fn box_from_quads(a: Point3, b: Point3, mat: Rc<dyn Material>) -> HittableList {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    sides.add(Rc::new(Quad::new(min, dx, dy, mat.clone())));
    sides.add(Rc::new(Quad::new(min, dx, dz, mat.clone())));
    sides.add(Rc::new(Quad::new(min, dy, dz, mat.clone())));
    sides.add(Rc::new(Quad::new(max, -dx, -dy, mat.clone())));
    sides.add(Rc::new(Quad::new(max, -dx, -dz, mat.clone())));
    sides.add(Rc::new(Quad::new(max, -dy, -dz, mat.clone())));

    sides
}
