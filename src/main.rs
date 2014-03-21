use shape::{Intersect, Shape};
use geometry::{Point, Normal, Vec3};

pub mod shape;
pub mod geometry;

pub mod transform;

//TODO make return Result

fn main() {
  let trans = transform::Transform { mat : transform::Mat4::identity() };
  let sphere = ~shape::Sphere{ trans : trans, radius : 1.0} as ~Shape;

  let ray = ~geometry::Ray { o:Point::new(10.0f32, 0f32, 0f32), d:Vec3::new(-1f32, 0f32, 0f32)};
  let k = sphere.intersect(ray);

  println!("{:?}", k);
  println!("{:?}", sphere);
  println!("{:?}", ray);

  let pixels : ~[int] = make_pixels();
  /*println!("Starting writing");
  write_pixels(pixels);*/

  let point = geometry::Point::new(1.,2.,3.);
  println!("{}", point);
}
