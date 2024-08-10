use std::{error::Error, io::Write, rc::Rc};

use crate::{
    camera::CameraOptionsBuilder,
    hittable::{hittable_list::HittableList, quad::Quad},
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    primitive::{color::Color, point3::Point3, vec3::Vec3},
};

pub fn cornell_box(out: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::from(Color::new(15.0, 15.0, 15.0)));

    world.add(Rc::new(Quad::new(
        Point3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        green.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        red.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Point3::new(343., 554., 332.),
        Vec3::new(-130., 0., 0.),
        Vec3::new(0., 0., 105.),
        light.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 0., 555.),
        white.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Point3::new(555., 555., 555.),
        Vec3::new(-555., 0., 0.),
        Vec3::new(0., 0., -555.),
        white.clone(),
    )));

    world.add(Rc::new(Quad::new(
        Point3::new(0., 0., 555.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        white.clone(),
    )));

    let cam_opts = CameraOptionsBuilder::default()
        .aspect_ratio(1.)
        .image_width(600)
        .samples_per_pixel(200)
        .max_depth(50)
        .background(Color::new(0., 0., 0.))
        .lookfrom(Point3::new(278., 278., -800.))
        .lookat(Point3::new(278., 278., 0.))
        .vfov(40.)
        .vup(Vec3::new(0., 1., 0.))
        .defocus_angle(0.)
        .build()?;

    let cam = cam_opts.build();

    cam.render(&world, out)?;

    Ok(())
}
