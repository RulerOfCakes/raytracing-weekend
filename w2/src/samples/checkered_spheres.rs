use std::rc::Rc;

use crate::{
    camera::Camera,
    hittable::{hittable_list::HittableList, sphere::Sphere},
    material::lambertian::Lambertian,
    primitive::{color::Color, interval::Interval, point3::Point3},
    texture::{checker_texture::CheckerTexture, solid_color::SolidColor, Texture},
};

pub fn checkered_spheres(out: &mut impl std::io::Write) -> std::io::Result<()> {
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

    let cam = Camera::new(
        16. / 9.,
        400,
        100,
        50,
        20.,
        Point3::new(13., 2., 3.),
        Point3::new(0., 0., 0.),
        Point3::new(0., 1., 0.),
        0.,
        10.,
        Interval::new(0., 1.),
    );

    cam.render(&world, out)
}
