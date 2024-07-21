use image::{DynamicImage, GenericImageView};

use crate::primitive::{color::Color, point3::Point3};

use super::Texture;

#[derive(Debug)]
pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new(image_path: &str) -> Result<Self, image::ImageError> {
        let image = image::open(image_path)?;
        let image = Self::linearize(&image).unwrap_or(image);
        Ok(Self { image })
    }

    // We use the following formula to linearize it, as we apply gamma correction at render time.
    fn linearize(image: &DynamicImage) -> Option<DynamicImage> {
        let img_bytes: Vec<u8> = image
            .as_bytes()
            .iter()
            .cloned()
            .map(|x| ((x as f64 / 255.0).powf(2.2) * 255.0) as u8)
            .collect();

        Some(DynamicImage::ImageRgb8(image::ImageBuffer::from_raw(
            image.width(),
            image.height(),
            img_bytes,
        )?))
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        if (self.image.height() == 0) || (self.image.width() == 0) {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // flip V to iamge coordinates

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;
        let pixel = self.image.get_pixel(i, j);

        let color_scale = 1. / 255.; // RGB values are in [0, 255]

        Color::new(
            color_scale * pixel.0[0] as f64,
            color_scale * pixel.0[1] as f64,
            color_scale * pixel.0[2] as f64,
        )
    }
}
