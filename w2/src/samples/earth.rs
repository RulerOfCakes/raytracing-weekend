use std::{error::Error, io::Write, rc::Rc};

use crate::{
    camera::CameraOptionsBuilder,
    hittable::{hittable_list::HittableList, sphere::Sphere},
    material::{lambertian::Lambertian, Material},
    primitive::{interval::Interval, vec3::Vec3},
    texture::{image_texture::ImageTexture, Texture},
};

pub fn earth(out: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg")?);
    let earth_surface = Rc::new(Lambertian::from(earth_texture as Rc<dyn Texture>));

    let globe = Rc::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface as Rc<dyn Material>,
    ));

    let camera_opts = CameraOptionsBuilder::default()
        .aspect_ratio(16.0 / 9.0)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.0)
        .lookfrom(Vec3::new(0.0, 0.0, 12.0))
        .lookat(Vec3::zero())
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.)
        .time_range(Interval::new(0., 1.))
        .build()?;
    let camera = camera_opts.build();

    let mut world = HittableList::new();
    world.add(globe);
    camera.render(&world, out)?;
    Ok(())
}
