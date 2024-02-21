#ifndef COLOR_H
#define COLOR_H

#include "vec3.h"

#include <iostream>

using color = vec3;

void write_color(std::ostream &out, color pixel_color, int samples_per_pixel) {
  pixel_color *= 1.0 / samples_per_pixel;

  // Write the translated [0,255] value of each color component.
  static const interval intensity(0.0, 0.999);
  out << static_cast<int>(256 * intensity.clamp(pixel_color.x())) << ' '
      << static_cast<int>(256 * intensity.clamp(pixel_color.y())) << ' '
      << static_cast<int>(256 * intensity.clamp(pixel_color.z())) << '\n';
}

#endif