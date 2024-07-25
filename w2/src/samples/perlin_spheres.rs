use std::{error::Error, io::Write, rc::Rc};

use crate::{
    camera::CameraOptionsBuilder,
    hittable::{hittable_list::HittableList, sphere::Sphere},
    material::lambertian::Lambertian,
    primitive::{interval::Interval, point3::Point3, vec3::Vec3},
    texture::{noise_texture::NoiseTexture, Texture},
};

pub fn perlin_spheres(out: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    let perlin_texture = Rc::new(NoiseTexture::new(4.));
    let perlin_material = Rc::new(Lambertian::from(perlin_texture.clone() as Rc<dyn Texture>));

    let floor_sphere = Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    ));
    let sphere = Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material.clone(),
    ));

    world.add(floor_sphere);
    world.add(sphere);

    let cam_opts = CameraOptionsBuilder::default()
        .aspect_ratio(16. / 9.)
        .image_width(400)
        .samples_per_pixel(100)
        .max_depth(50)
        .vfov(20.)
        .lookfrom(Point3::new(13., 2., 3.))
        .lookat(Point3::new(0., 0., 0.))
        .vup(Vec3::new(0., 1., 0.))
        .defocus_angle(0.)
        .time_range(Interval::new(0., 1.))
        .build()?;

    let cam = cam_opts.build();

    cam.render(&world, out)?;
    Ok(())
}
