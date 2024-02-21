#ifndef COLOR_H
#define COLOR_H

#include "vec3.h"

#include <iostream>

using color = vec3;

inline double linear_to_gamma(double linear_component) {
  return sqrt(linear_component);
}

inline vec3 linear_to_gamma(const vec3 &linear_color) {
  return vec3(linear_to_gamma(linear_color.x()),
              linear_to_gamma(linear_color.y()),
              linear_to_gamma(linear_color.z()));
}

void write_color(std::ostream &out, color pixel_color, int samples_per_pixel) {
  pixel_color *= 1.0 / samples_per_pixel;

  pixel_color = linear_to_gamma(pixel_color);

  // Write the translated [0,255] value of each color component.
  static const interval intensity(0.0, 0.999);
  out << static_cast<int>(256 * intensity.clamp(pixel_color.x())) << ' '
      << static_cast<int>(256 * intensity.clamp(pixel_color.y())) << ' '
      << static_cast<int>(256 * intensity.clamp(pixel_color.z())) << '\n';
}

#endif