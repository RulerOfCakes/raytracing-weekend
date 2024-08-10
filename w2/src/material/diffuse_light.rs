use std::rc::Rc;

use crate::{
    primitive::{color::Color, point3::Point3},
    texture::{solid_color::SolidColor, Texture},
};

use super::Material;

#[derive(Debug)]
pub struct DiffuseLight {
    texture: Rc<dyn Texture>,
}

impl From<Color> for DiffuseLight {
    fn from(color: Color) -> Self {
        Self {
            texture: Rc::new(SolidColor::new(color)),
        }
    }
}

impl DiffuseLight {
    pub fn new(texture: Rc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.texture.value(u, v, p)
    }
}
