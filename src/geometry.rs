extern crate cgmath;

pub use Mat4 = cgmath::matrix::Mat4;

use std::fmt;

struct Mat4 {
  data : [f32, ..16]
}

struct Point {
  x : f32,
  y : f32,
  z : f32
}

struct Vector {
  x : f32,
  y : f32,
  z : f32
}

struct Point {
  x : f32,
  y : f32,
  z : f32
}

pub struct Ray {
  o : Point3<f32>,
  d : Vec3<f32>
}

impl fmt::Show for Ray {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f.buf, "Ray \\{ o: {}, d: {} \\}", self.o, self.d)
  }
}
