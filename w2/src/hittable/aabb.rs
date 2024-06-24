use crate::primitive::{
    interval::{Interval, EMPTY_INTERVAL},
    point3::Point3,
    ray::Ray,
};

// Axis-Aligned Bounding Box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }
    pub fn new_from_points(p0: Point3, p1: Point3) -> Self {
        let x = Interval::new(p0.x, p1.x).reorder();
        let y = Interval::new(p0.y, p1.y).reorder();
        let z = Interval::new(p0.z, p1.z).reorder();
        Self { x, y, z }
    }
    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> Self {
        let x = box0.x.merge(&box1.x);
        let y = box0.y.merge(&box1.y);
        let z = box0.z.merge(&box1.z);
        Self { x, y, z }
    }
    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis index"),
        }
    }
    pub fn longest_axis(&self) -> usize {
        let x_size = self.x.size();
        let y_size = self.y.size();
        let z_size = self.z.size();

        if x_size > y_size {
            if x_size > z_size {
                0
            } else {
                2
            }
        } else if y_size > z_size {
            1
        } else {
            2
        }
    }
    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_origin: Point3 = r.origin();
        let ray_dir = r.direction();

        for i in 0..3 {
            let axis = self.axis_interval(i);
            // this can be NaN if the ray_dir component is 0
            let adinv = 1.0 / ray_dir.get(i);

            let mut t0 = (axis.start - ray_origin.get(i)) * adinv;
            let mut t1 = (axis.end - ray_origin.get(i)) * adinv;

            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            ray_t.start = ray_t.start.max(t0);
            ray_t.end = ray_t.end.min(t1);

            if ray_t.start >= ray_t.end {
                return false;
            }
        }
        true
    }
}

pub static EMPTY_AABB: AABB = AABB {
    x: EMPTY_INTERVAL,
    y: EMPTY_INTERVAL,
    z: EMPTY_INTERVAL,
};
