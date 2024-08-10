use std::{error::Error, io::Write, rc::Rc};

use crate::{
    camera::CameraOptionsBuilder,
    hittable::{hittable_list::HittableList, quad::Quad, sphere::Sphere},
    material::{lambertian::Lambertian, Material},
    primitive::{color::Color, interval::Interval, point3::Point3, vec3::Vec3},
    texture::{image_texture::ImageTexture, Texture},
};
pub fn quads(out: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    // Materials
    let left_red = Rc::new(Lambertian::new(Color::new(1., 0.2, 0.2)));
    let back_green = Rc::new(Lambertian::new(Color::new(0.2, 1., 0.2)));
    let right_blue = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 1.)));
    let upper_orange = Rc::new(Lambertian::new(Color::new(1., 0.5, 0.0)));
    let lower_teal = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Rc::new(Quad::new(
        Point3::new(-3., -2., 5.),
        Point3::new(0., 0., -4.),
        Point3::new(0., 4., 0.),
        left_red.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2., -2., 0.),
        Point3::new(4., 0., 0.),
        Point3::new(0., 4., 0.),
        back_green.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3., -2., 1.),
        Point3::new(0., 0., 4.),
        Vec3::new(0., 4., 0.),
        right_blue.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2., 3., 1.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., 4.),
        upper_orange.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2., -3., 5.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., -4.),
        lower_teal.clone(),
    )));

    let camera_opts = CameraOptionsBuilder::default()
        .aspect_ratio(1.)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(80.0)
        .lookfrom(Vec3::new(0.0, 0.0, 9.0))
        .lookat(Vec3::zero())
        .vup(Vec3::new(0.0, 1.0, 0.0))
        .defocus_angle(0.)
        .time_range(Interval::new(0., 1.))
        .build()?;
    let camera = camera_opts.build();

    camera.render(&world, out)?;

    Ok(())
}
