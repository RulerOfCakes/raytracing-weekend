#ifndef RTWEEKEND_H
#define RTWEEKEND_H

#include <cmath>
#include <limits>
#include <memory>
#include <random>

// Usings

using std::make_shared;
using std::shared_ptr;
using std::sqrt;

// Constants

const double infinity = std::numeric_limits<double>::infinity();
const double pi = 3.1415926535897932385;

// Utility Functions

inline double degrees_to_radians(double degrees) {
  return degrees * pi / 180.0;
}

// [0, 1)
inline double random_double() {
  static std::uniform_real_distribution<double> distribution(0.0, 1.0);
  static std::mt19937 generator;
  return distribution(generator);
}

// [min, max)
inline double random_double(double min, double max) {
  return min + (max - min) * random_double();
}

inline double random_gaussian() {
  static std::random_device rd;
  static std::mt19937 gen(rd());
  static std::normal_distribution<double> distribution(0.0, 1.0);
  return distribution(gen);
}

inline double random_gaussian(double mean, double stddev) {
  return mean + stddev * random_gaussian();
}

// Common Headers

#include "interval.h"
#include "ray.h"
#include "vec3.h"

#endif
