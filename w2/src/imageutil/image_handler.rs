use std::path::Path;

struct ImageHandler {}

impl ImageHandler {
    fn open_image(&self, path: &Path) -> Result<image::DynamicImage, image::ImageError> {
        image::open(path)
    }
}
