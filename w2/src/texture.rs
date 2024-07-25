use std::fmt::Debug;

use crate::primitive::{color::Color, point3::Point3};

pub trait Texture: Debug {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub mod checker_texture;
pub mod image_texture;
mod perlin;
pub mod solid_color;
pub mod noise_texture;