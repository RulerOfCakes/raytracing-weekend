use std::rc::Rc;

use crate::{
    camera::CameraOptionsBuilder,
    hittable::{bvh::BVHNode, hittable_list::HittableList, sphere::Sphere},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    primitive::{color::Color, interval::Interval, point3::Point3, vec3::Vec3},
    texture::{checker_texture::CheckerTexture, solid_color::SolidColor, Texture},
};

use rand::random;

pub fn bouncing_spheres(out: &mut impl std::io::Write) -> std::io::Result<()> {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::new(
        0.32,
        Rc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Rc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));

    let lambert_ground = Rc::new(Lambertian::from(checker as Rc<dyn Texture>));

    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        lambert_ground.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let mat_choice = random::<f64>();
            let center = Point3::new(
                f64::from(a) + 0.9 * random::<f64>(),
                0.2,
                f64::from(b) + 0.9 * random::<f64>(),
            );
            let mut velocity = Vec3::zero();

            if (center
                - Point3 {
                    x: 4.,
                    y: 0.2,
                    z: 0.,
                })
            .length()
                > 0.9
            {
                let sphere_mat: Rc<dyn Material> = match mat_choice {
                    x if x < 0.8 => {
                        velocity = Vec3::new(0., random::<f64>() * 0.5, 0.);
                        Rc::new(Lambertian::new(Color::random() * Color::random()))
                    }
                    x if x < 0.95 => Rc::new(Metal::new(Color::random(), random::<f64>() * 0.5)),
                    _ => Rc::new(Dielectric::new(1.5)),
                };

                world.add(Rc::new(Sphere::new_moving(
                    center, 0.2, sphere_mat, velocity,
                )))
            }
        }
    }

    world.add(Rc::new(Sphere::new(
        Vec3::new(0., 1., 0.),
        1.0,
        Rc::new(Dielectric::new(1.5)),
    )));

    world.add(Rc::new(Sphere::new(
        Vec3::new(-4., 1., 0.),
        1.0,
        Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.0,
        Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    )));

    // convert world to bvh
    let bvh_node = Rc::new(BVHNode::from(world));

    world = HittableList::new();
    world.add(bvh_node);

    let opts = CameraOptionsBuilder::default()
        .aspect_ratio(16. / 9.)
        .image_width(600)
        .samples_per_pixel(50)
        .max_depth(50)
        .vfov(20.)
        .lookfrom(Point3::new(13., 2., 3.))
        .lookat(Point3::zero())
        .vup(Vec3::new(0., 1., 0.))
        .defocus_angle(0.6)
        .focus_dist(10.)
        .time_range(Interval::new(0., 1.))
        .build()
        .unwrap();

    let camera = opts.build();
    camera.render(&world, out)
}
