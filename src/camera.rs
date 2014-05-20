use geometry::{Vec3, Ray};
use transform::Transform;
use sample::Sample;
use film::Film;
use filter::Filter;
use std::rc::Rc;

#[test]
use geometry::{Point};
#[test]
use filter;

pub trait Camera<T> {
  fn generate_ray(&self, sample : &Sample) -> Ray;
  fn get_film<'a> (&'a mut self) -> Rc<Box<Film<T>>>;
}

#[deriving(Eq, Clone)]
pub struct OrthographicCamera<T> {
  raster_to_screen : Transform,
  screen_to_camera : Transform,
  camera_to_world : Transform,
  pub film : Rc<Box<Film<T>>>
}

impl<T : Filter> OrthographicCamera<T> {
  pub fn new(camera_to_world: Transform, window : (f32, f32, f32, f32), film: Film<T>) -> OrthographicCamera<T> {
    let clipping = (0., 1000.);
    let (znear, zfar) = clipping;
    let camera_to_screen = Transform::scale(1., 1., 1. / (zfar - znear))
      * Transform::translate(Vec3::new(0., 0., -znear));
    let (sx, sy) = match film.size { (x, y) => (x as f32, y as f32) };
    let (min_x, max_x, min_y, max_y) = window;
    let screen_to_raster = Transform::scale(sx, sy, 1f32) *
                           Transform::scale(1./(max_x - min_x), 1./(max_y - min_y), 1.) *
                           Transform::translate(Vec3::new(-min_x, -min_y, 0.));
    OrthographicCamera {
      raster_to_screen : screen_to_raster.inverse(),
      screen_to_camera : camera_to_screen.inverse(),
      camera_to_world : camera_to_world,
      film : Rc::new(box film)
    }
  }
}

impl<T : Filter> Camera<T> for OrthographicCamera<T> {
  fn generate_ray(&self, sample : &Sample) -> Ray {
    let direction = self.camera_to_world.apply_Vec3(&Vec3::new(0., 0., 1.));
    let raster_to_world = self.camera_to_world * self.screen_to_camera * self.raster_to_screen;
    let origin = raster_to_world.apply_Point(&sample.point);
    Ray::new(origin, direction)
  }

  fn get_film<'a>(&'a mut self) -> Rc<Box<Film<T>>> {
    self.film.clone()
  }
}


#[test]
fn test_OrthographicCamera_generate_ray() {
  let trans = Transform::translate(Vec3::new(0., 0., -2.));
  let filter = box filter::Box::new(0.5, 0.5);
  let film = Film::new((10, 10), filter);
  let camera = box OrthographicCamera::new(trans, (-10., 10., -10., 10.), film);

  let sample = Sample::new(7.5, 7.5);
  let ray = camera.generate_ray(&sample);
  let true_ray = Ray::new(Point::new(5.,5.,-2.), Vec3::new(0.,0.,1.));
  assert_eq!(ray, true_ray)

  let sample = Sample::new(2.5, 2.5);
  let ray = camera.generate_ray(&sample);
  let true_ray = Ray::new(Point::new(-5.,-5.,-2.), Vec3::new(0.,0.,1.));
  assert_eq!(ray, true_ray)
}
