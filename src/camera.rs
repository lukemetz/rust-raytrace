use geometry::{Point, Vec3, Ray};
use transform::Transform;
pub mod transform;
pub mod geometry;

pub struct CameraSample {
  point : Point //In screen space
}

impl CameraSample {
  pub fn new(x : f32, y : f32) -> CameraSample {
    CameraSample{point : Point::new(x, y, 0.)}
  }
}

pub trait Camera {
  fn generate_ray(&self, sample : &CameraSample) -> Ray;
}

#[deriving(Eq, Clone, Show)]
pub struct OrthographicCamera {
  transform : Transform
}

impl OrthographicCamera {
  fn new(trans : Transform) -> OrthographicCamera {
    OrthographicCamera { transform : trans }
  }
}

impl Camera for OrthographicCamera {
  fn generate_ray(&self, sample : &CameraSample) -> Ray {
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
  let sample = CameraSample::new(0.5, 0.5);
  let ray = camera.generate_ray(&sample);
  let true_ray = Ray::new(Point::new(5.,5.,0.), Vec3::new(0.,0.,1.));
  assert_eq!(ray, true_ray)
}
