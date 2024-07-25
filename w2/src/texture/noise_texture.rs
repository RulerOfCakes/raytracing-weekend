use crate::primitive::{color::Color, point3::Point3};

use super::{perlin::Perlin, Texture};

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64, // scale increases the pattern frequency
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self::new(1.)
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5)
            * (1. + (self.noise.turbulence(p, 7) * 10. + self.scale * p.z).sin())
    }
}
