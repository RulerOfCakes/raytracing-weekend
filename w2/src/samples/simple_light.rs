use std::{error::Error, io::Write, rc::Rc};

use crate::{
    camera::CameraOptionsBuilder,
    hittable::{hittable_list::HittableList, quad::Quad, sphere::Sphere},
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    primitive::{color::Color, point3::Point3, vec3::Vec3},
    texture::{noise_texture::NoiseTexture, Texture},
};

pub fn simple_light(out: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    let perlin: Rc<dyn Texture> = Rc::new(NoiseTexture::new(4.));

    let perlin_lamb = Rc::new(Lambertian::from(perlin.clone()));

    world.add(Rc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        perlin_lamb.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        perlin_lamb.clone(),
    )));

    let difflight = Rc::new(DiffuseLight::from(Color::new(4., 4., 4.)));

    world.add(Rc::new(Quad::new(
        Point3::new(3., 1., -2.),
        Vec3::new(2., 0., 0.),
        Vec3::new(0., 2., 0.),
        difflight.clone(),
    )));

    let cam_opts = CameraOptionsBuilder::default()
        .aspect_ratio(16. / 9.)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.)
        .lookfrom(Point3::new(26., 3., 6.))
        .lookat(Point3::new(0., 2., 0.))
        .vup(Point3::new(0., 1., 0.))
        .background(Color::new(0., 0., 0.))
        .defocus_angle(0.)
        .build()?;

    let cam = cam_opts.build();

    cam.render(&world, out)?;

    Ok(())
}
