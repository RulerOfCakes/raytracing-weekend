#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub start: f64,
    pub end: f64,
}

impl Interval {
    pub fn new(start: f64, end: f64) -> Interval {
        Interval { start, end }
    }
    pub fn reorder(&self) -> Interval {
        if self.start > self.end {
            Interval {
                start: self.end,
                end: self.start,
            }
        } else {
            *self
        }
    }
    // returns the minimum enclosing interval.
    pub fn merge(&self, other: &Interval) -> Interval {
        Interval {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
    pub fn size(&self) -> f64 {
        self.end - self.start
    }
    pub fn contains(&self, x: f64) -> bool {
        x >= self.start && x <= self.end
    }
    pub fn surrounds(&self, x: f64) -> bool {
        x > self.start && x < self.end
    }
    pub fn contains_interval(&self, other: &Interval) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    pub fn clamp(&self, x: f64) -> f64 {
        x.max(self.start).min(self.end)
    }
    pub fn expand(&self, delta: f64) -> Interval {
        Interval {
            start: self.start - delta,
            end: self.end + delta,
        }
    }
}

pub static EMPTY_INTERVAL: Interval = Interval {
    start: f64::INFINITY,
    end: f64::NEG_INFINITY,
};
pub static MAX_INTERVAL: Interval = Interval {
    start: f64::MIN,
    end: f64::MAX,
};
