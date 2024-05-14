use std::rc::Rc;

use w2::{
    camera::Camera,
    hittable::{hittable_list::HittableList, sphere::Sphere},
    material::lambertian::Lambertian,
    primitive::{color::Color, vec3::Vec3},
};

fn main() {
    let mut world = HittableList::new();

    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, 0.0, -5.0),
        1.7,
        Rc::new(Lambertian::new(Color::new(0.8, 0.3, 0.3))),
    )));
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -100.5, -5.0),
        100.0,
        Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
    )));

    let camera = Camera::new(
        16.0 / 9.0,
        384,
        50,
        50,
        45.0,
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.1,
        1.0,
    );
    let mut outstream = std::io::stdout().lock();
    if let Err(e) = camera.render(&world, &mut outstream) {
        eprintln!("{}", e);
    }
}
