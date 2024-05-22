use std::rc::Rc;

use rand::random;
use w2::{
    camera::Camera,
    hittable::{hittable_list::HittableList, sphere::Sphere},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    primitive::{color::Color, point3::Point3, vec3::Vec3},
};

fn main() {
    let mut world = HittableList::new();

    let lambert_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

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
                    x if x < 0.8 => Rc::new(Lambertian::new(Color::random() * Color::random())),
                    x if x < 0.95 => Rc::new(Metal::new(Color::random(), random::<f64>() * 0.5)),
                    _ => Rc::new(Dielectric::new(1.5)),
                };

                world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat)))
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

    let camera = Camera::new(
        16.0 / 9.0,
        600,
        50,
        50,
        20.0,
        Vec3::new(13., 2., 3.),
        Vec3::new(0., 0., 0.),
        Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );
    let mut outstream = std::io::stdout().lock();
    if let Err(e) = camera.render(&world, &mut outstream) {
        eprintln!("{}", e);
    }
}
