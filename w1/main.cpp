#include "rtweekend.h"

#include "camera.h"
#include "hittable_list.h"
#include "sphere.h"

int main() {

  // Image
  auto aspect_ratio = 16.0 / 9.0;
  int image_width = 400;

  // World
  hittable_list world;

  world.add(make_shared<sphere>(point3(0, 0, -1), 0.5));
  world.add(make_shared<sphere>(point3(0, -100.5, -1), 100));

  camera cam;

  cam.aspect_ratio = aspect_ratio;
  cam.image_width = image_width;
  cam.samples_per_pixel = 100;
  cam.max_depth = 50;

  cam.render(world);
}