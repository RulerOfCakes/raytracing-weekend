use std::ops::{Add, AddAssign, Mul};

use crate::primitive::interval::Interval;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

const INTENSITY: Interval = Interval {
    start: 0.0,
    end: 0.999,
};

impl Color {
    pub fn new<T: Into<f64>>(r: T, g: T, b: T) -> Self {
        Self {
            r: r.into(),
            g: g.into(),
            b: b.into(),
        }
    }
    pub fn linear_to_gamma(&self) -> Self {
        Self {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }
    pub fn write_color(
        &self,
        out: &mut impl std::io::Write,
        samples_per_pixel: usize,
    ) -> std::io::Result<()> {
        let mut color = *self * (1.0 / samples_per_pixel as f64);
        color = color.linear_to_gamma();

        writeln!(
            out,
            "{} {} {}",
            (256.0 * INTENSITY.clamp(color.r)) as u8,
            (256.0 * INTENSITY.clamp(color.g)) as u8,
            (256.0 * INTENSITY.clamp(color.b)) as u8
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
