use rand_distr::{Distribution, Uniform};

use crate::{
    hittable::Hittable,
    primitive::{color::Color, interval::Interval, point3::Point3, ray::Ray, vec3::Vec3},
};

#[derive(Debug)]
pub struct Camera {
    aspect_ratio: f64,
    image_width: usize,
    image_height: usize,

    samples_per_pixel: usize,
    pixel_samples_scale: f64,
    max_depth: usize, // recursion depth for shadow rays

    vfov: f64,
    lookfrom: Point3, // Camera center
    lookat: Vec3,     // Camera target
    vup: Vec3,        // viewport orientation vector(world up)

    pixel00_loc: Point3, // location of the top-left corner of the viewport
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    // Depth of field effect
    defocus_angle: f64, // angle of camera lens cone
    focus_dist: f64,    // distance of lookfrom to the focus plane
    defocus_u: Vec3,
    defocus_v: Vec3,

    // time range of the current frame
    time_step: Uniform<f64>,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: usize,
        max_depth: usize,
        vfov: f64,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
        time_range: Interval,
    ) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as usize;
        if image_height < 1 {
            image_height = 1;
        }

        let lookfrom: Point3 = Point3::from(lookfrom);

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = (image_width as f64 / image_height as f64) * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left = lookfrom - viewport_u / 2.0 - viewport_v / 2.0 - w * focus_dist;
        let pixel00_loc = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_u = u * defocus_radius;
        let defocus_v = v * defocus_radius;

        let sampler = Uniform::new(time_range.start, time_range.end);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            pixel_samples_scale: 1.0 / (samples_per_pixel as f64),
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_angle,
            focus_dist,
            defocus_u,
            defocus_v,
            time_step: sampler,
        }
    }

    pub fn render(
        &self,
        world: &dyn Hittable,
        out: &mut impl std::io::Write,
    ) -> std::io::Result<()> {
        writeln!(out, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0, 0, 0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&r, world, self.max_depth);
                }
                pixel_color *= self.pixel_samples_scale;
                pixel_color.write_color(out)?;
            }
        }
        Result::Ok(())
    }

    fn pixel_sample(&self) -> Vec3 {
        let px = -0.5 + rand::random::<f64>();
        let py = -0.5 + rand::random::<f64>();
        self.pixel_delta_u * px + self.pixel_delta_v * py
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_disk(1.0);
        self.lookfrom + self.defocus_u * p.x + self.defocus_v * p.y
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let pixel_center =
            self.pixel00_loc + self.pixel_delta_u * (i as f64) + self.pixel_delta_v * (j as f64);
        let pixel_sample = pixel_center + self.pixel_sample();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.lookfrom
        } else {
            self.defocus_disk_sample()
        };

        let ray_dir = pixel_sample - ray_origin;
        let time = self.time_step.sample(&mut rand::thread_rng());

        Ray::new(ray_origin, ray_dir, time)
    }

    fn ray_color(r: &Ray, world: &dyn Hittable, max_depth: usize) -> Color {
        if max_depth == 0 {
            return Color::new(0, 0, 0);
        }

        if let Some(rec) = world.hit(
            r,
            Interval {
                start: 0.001,
                end: f64::INFINITY,
            },
        ) {
            let mut scattered = Ray::new(Vec3::zero(), Vec3::zero(), r.time());
            let mut attenuation = Color::new(0, 0, 0);
            if rec
                .material
                .scatter(r, &rec, &mut attenuation, &mut scattered)
                && max_depth > 0
            {
                attenuation * Camera::ray_color(&scattered, world, max_depth - 1)
            } else {
                Color::new(0, 0, 0)
            }
        } else {
            let unit_direction = r.direction().unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            Color::new(1, 1, 1) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}
