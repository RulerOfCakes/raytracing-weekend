#include "rtweekend.h"

#include "color.h"
#include "hittable.h"
#include "hittable_list.h"
#include "sphere.h"

#include <iostream>

color ray_color(const ray &r, const hittable &world) {
  hit_record rec;
  if (world.hit(r, interval(0, infinity), rec)) {
    // color by unit normals
    return 0.5 * (rec.normal + color(1, 1, 1));
  }

  vec3 unit_direction = unit_vector(r.direction());
  auto a = 0.5 * (unit_direction.y() + 1.0);
  // white to blue based on y
  return (1.0 - a) * color(1.0, 1.0, 1.0) + a * color(0.5, 0.7, 1.0);
}

int main() {

  // Image
  auto aspect_ratio = 16.0 / 9.0;
  int image_width = 400;

  // Calculate the image height, and ensure that it's at least 1.
  int image_height = static_cast<int>(image_width / aspect_ratio);
  image_height = (image_height < 1) ? 1 : image_height;

  // World
  hittable_list world;

  world.add(make_shared<sphere>(point3(0, 0, -1), 0.5));
  world.add(make_shared<sphere>(point3(0, -100.5, -1), 100));

  // Camera
  auto focal_length = 1.0;
  // Viewport widths less than one are ok since they are real valued.
  auto viewport_height = 2.0;
  auto viewport_width =
      viewport_height * (static_cast<double>(image_width) / image_height);
  auto camera_center = point3(0, 0, 0);

  auto viewport_u = vec3(viewport_width, 0, 0);
  auto viewport_v = vec3(0, -viewport_height, 0);

  auto pixel_delta_u = viewport_u / image_width;
  auto pixel_delta_v = viewport_v / image_height;

  auto viewport_upper_left = camera_center - viewport_u / 2 - viewport_v / 2 -
                             vec3(0, 0, focal_length);
  auto pixel00_loc =
      viewport_upper_left + pixel_delta_u / 2 + pixel_delta_v / 2;

  // Render

  std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

  for (int j = 0; j < image_height; ++j) {
    std::clog << "\rScanlines remaining: " << (image_height - j) << ' '
              << std::flush;
    for (int i = 0; i < image_width; ++i) {
      auto pixel_center =
          pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
      auto ray_direction = pixel_center - camera_center;
      ray r(camera_center, ray_direction);

      color pixel_color = ray_color(r, world);
      write_color(std::cout, pixel_color);
    }
  }
  std::clog << "\rDone.                 \n";
}