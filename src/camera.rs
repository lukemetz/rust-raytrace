use geometry::{Point, Vec3, Ray};
use transform::Transform;
use sample::Sample;
use film::Film;
use spectrum::Spectrum;
use std::io::fs::unlink;

pub mod spectrum;
pub mod transform;
pub mod geometry;
pub mod sample;
pub mod film;

pub trait Camera {
  fn generate_ray(&self, sample : &Sample) -> Ray;
}

#[deriving(Eq, Clone, Show)]
pub struct OrthographicCamera {
  transform : Transform,
  film_size : (uint, uint)
}

impl OrthographicCamera {
  pub fn new(trans : Transform, film : &Film) -> OrthographicCamera {
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
  let film = Film::new((10, 10));
  let camera = ~OrthographicCamera::new(trans, &film);
  println!("{}", camera);
  let sample = Sample::new(0.5, 0.5);
  let ray = camera.generate_ray(&sample);
  let true_ray = Ray::new(Point::new(5.,5.,0.), Vec3::new(0.,0.,1.));
  assert_eq!(ray, true_ray)
}
