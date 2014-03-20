use shape::{Intersect, Shape};
use geometry::{Point, Normal, Vec3};

pub mod shape;
pub mod geometry;

pub mod transform;

fn make_pixels() -> ~[int] {
  let mut pixels = ~[0, ..(300*300*3)];
  for x in range(0, 300) {
    for y in range(0, 300) {
      pixels[x*300*3+y*3] = x % 255;
      pixels[x*300*3+y*3+1] = 128;
      pixels[x*300*3+y*3+2] = y%255;
    }
  }
  pixels
}

//TODO make return Result
fn write_pixels(pixels : &[int]) {
  use std::io::{File, Open, Write};
  let img_path = Path::new("test.ppm");
  let header : ~str = format!("P3\n{:d} {:d}\n{:d}\n", 300, 300, 255);
  //println!( header);
  let mut file = File::open_mode(&img_path, Open, Write).unwrap();

  if file.write_str(header).is_err() {
    println!("Failed to write file");
    return;
  }
  for pixel_idx in range(0, 300*300) {
    let r = pixels[pixel_idx*3+0];
    let g = pixels[pixel_idx*3+1];
    let b = pixels[pixel_idx*3+2];
    if file.write_str(format!("{:d} {:d} {:d} ", r, g, b)).is_err() {
      println!("Failed to write file");
      return;
    }
  }
}

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
