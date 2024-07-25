use std::{error::Error, rc::Rc};

use crate::{
    camera::CameraOptionsBuilder,
    hittable::{hittable_list::HittableList, sphere::Sphere},
    material::lambertian::Lambertian,
    primitive::{color::Color, interval::Interval, point3::Point3},
    texture::{checker_texture::CheckerTexture, solid_color::SolidColor, Texture},
};

pub fn checkered_spheres(out: &mut impl std::io::Write) -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    let checker: Rc<dyn Texture> = Rc::new(CheckerTexture::new(
        0.32,
        Rc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Rc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));

    world.add(Rc::new(Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        Rc::new(Lambertian::from(checker.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        Rc::new(Lambertian::from(checker.clone())),
    )));

    let opts = CameraOptionsBuilder::default()
        .aspect_ratio(16. / 9.)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .lookfrom(Point3::new(13., 2., 3.))
        .lookat(Point3::new(0., 0., 0.))
        .vup(Point3::new(0., 1., 0.))
        .vfov(20.)
        .defocus_angle(0.)
        .focus_dist(10.)
        .time_range(Interval::new(0., 1.))
        .build()?;
    let cam = opts.build();

    cam.render(&world, out)?;
    Ok(())
}
