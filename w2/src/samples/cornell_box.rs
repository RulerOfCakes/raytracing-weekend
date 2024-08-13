use std::{error::Error, io::Write, rc::Rc};

use crate::{
    camera::CameraOptionsBuilder,
    hittable::{
        hittable_list::HittableList,
        quad::{box_from_quads, Quad},
        rotate_y::RotateY,
        translate::Translate,
    },
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

    // Boxes

    let box1 = Rc::new(box_from_quads(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white.clone(),
    ));
    let box1 = Rc::new(RotateY::new(box1, 15.));
    let box1 = Rc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));
    world.add(box1);

    let box2 = Rc::new(box_from_quads(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white.clone(),
    ));
    let box2 = Rc::new(RotateY::new(box2, -18.));
    let box2 = Rc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));
    world.add(box2);

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
