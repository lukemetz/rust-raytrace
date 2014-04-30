use geometry::{Point, Vec3, Ray};
use transform::Transform;
use sample::Sample;
pub mod transform;
pub mod geometry;
pub mod sample;

pub trait Camera {
  fn generate_ray(&self, sample : &Sample) -> Ray;
}

#[deriving(Eq, Clone, Show)]
pub struct OrthographicCamera {
  transform : Transform
}

impl OrthographicCamera {
  pub fn new(trans : Transform) -> OrthographicCamera {
    OrthographicCamera { transform : trans }
  }
}

impl Camera for OrthographicCamera {
  fn generate_ray(&self, sample : &Sample) -> Ray {
    let origin = self.transform.apply_Point(&sample.point);
    let direction = Vec3::new(0.,0.,1.);
    Ray::new(origin, direction)
  }
}

#[test]
fn test_OrthographicCamera_generate_ray() {
  let trans = Transform::scale(10.);
  let camera = ~OrthographicCamera::new(trans);
  println!("{}", camera);
  let sample = Sample::new(0.5, 0.5);
  let ray = camera.generate_ray(&sample);
  let true_ray = Ray::new(Point::new(5.,5.,0.), Vec3::new(0.,0.,1.));
  assert_eq!(ray, true_ray)
}
