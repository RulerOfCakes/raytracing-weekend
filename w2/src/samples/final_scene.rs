use std::{error::Error, io::Write, rc::Rc};

use crate::{
    camera::CameraOptionsBuilder,
    hittable::{
        bvh::BVHNode,
        constant_medium::ConstantMedium,
        hittable_list::HittableList,
        quad::{box_from_quads, Quad},
        rotate_y::RotateY,
        sphere::{self, Sphere},
        translate::Translate,
    },
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    primitive::{color::Color, point3::Point3, vec3::Vec3},
    texture::{image_texture::ImageTexture, noise_texture::NoiseTexture, Texture},
};

pub fn final_scene(
    out: &mut impl Write,
    image_width: u32,
    samples_per_pixel: u32,
    max_depth: u32,
) -> Result<(), Box<dyn Error>> {
    let mut world = HittableList::new();

    let ground = Rc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    let mut displaced_ground = HittableList::new();

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let x0 = -1000. + i as f64 * w;
            let z0 = -1000. + j as f64 * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = 100. * (rand::random::<f64>() + 0.01);
            let z1 = z0 + w;

            displaced_ground.add(Rc::new(box_from_quads(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    world.add(Rc::new(BVHNode::from(displaced_ground)));

    let light = DiffuseLight::from(Color::new(7., 7., 7.));
    world.add(Rc::new(Quad::new(
        Point3::new(123., 554., 147.),
        Vec3::new(300., 0., 0.),
        Vec3::new(0., 0., 265.),
        Rc::new(light),
    )));

    let center1 = Point3::new(400., 400., 200.);
    let sphere_material = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Rc::new(Sphere::new_moving(
        center1,
        50.,
        sphere_material.clone(),
        Vec3::new(30., 0., 0.),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(260., 150., 45.),
        50.,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0., 150., 145.),
        50.,
        Rc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.)),
    )));

    let boundary = Rc::new(Sphere::new(
        Point3::new(360., 150., 145.),
        70.,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Rc::new(ConstantMedium::new_from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Rc::new(Sphere::new(
        Point3::zero(),
        5000.,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.add(Rc::new(ConstantMedium::new_from_color(
        boundary,
        0.0001,
        Color::new(1., 1., 1.),
    )));

    let emat = Rc::new(Lambertian::from(
        Rc::new(ImageTexture::new("earthmap.jpg")?) as Rc<dyn Texture>,
    ));
    world.add(Rc::new(Sphere::new(
        Point3::new(400., 200., 400.),
        100.,
        emat,
    )));
    let pertext = Rc::new(NoiseTexture::new(0.2));
    world.add(Rc::new(Sphere::new(
        Point3::new(220., 280., 300.),
        80.,
        Rc::new(Lambertian::from(pertext as Rc<dyn Texture>)),
    )));

    let mut boxes2 = HittableList::new();
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Rc::new(Sphere::new(
            Point3::random_uniform(0., 165.),
            10.,
            white.clone(),
        )));
    }

    world.add(Rc::new(Translate::new(
        Rc::new(RotateY::new(Rc::new(BVHNode::from(boxes2)), 15.0)),
        Vec3::new(-100., 270., 395.),
    )));

    let cam_opts = CameraOptionsBuilder::default()
        .aspect_ratio(1.)
        .image_width(image_width)
        .samples_per_pixel(samples_per_pixel)
        .max_depth(max_depth)
        .background(Color::new(0., 0., 0.))
        .lookfrom(Point3::new(478., 278., -600.))
        .lookat(Point3::new(278., 278., 0.))
        .vfov(40.)
        .vup(Vec3::new(0., 1., 0.))
        .defocus_angle(0.)
        .build()?;

    let cam = cam_opts.build();

    cam.render(&world, out)?;

    Ok(())
}
