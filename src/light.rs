use spectrum::Spectrum;
use geometry::{Point, Vec3, Ray};
use std::num::Float;
use transform::Transform;
use scene::Scene;
use sample::Sample;
use primitive::Intersect;


pub trait Light {
  fn sample_l(&self, p : &Point, epsilon : f32, light_sample : &LightSample) -> (Spectrum, Vec3, f32, VisibilityTester);
  fn is_delta_light(&self) -> bool;
}

pub struct LightSample {
  pos : (f32, f32),
  component : f32
}

pub struct LightSampleOffsets {
  n_samples : uint,
  component_offset : uint,
  pos_offset : uint
}

pub struct VisibilityTester {
  ray : Ray
}

impl LightSample {
  pub fn from_offset(sample : &Sample, offset : uint) -> LightSample{
    let p0 = sample.extra.get(offset);
    let p1 = sample.extra.get(offset+1);
    let pos = (*p0, *p1);
    let component = sample.extra.get(offset+2);
    LightSample {pos : pos, component : *component}
  }
}

impl VisibilityTester {
  pub fn from_segment(p1 : &Point, eps1 : f32, p2 : &Point, eps2 : f32) -> VisibilityTester {
    let dist = p1.distance(p2);
    let r = Ray::new_bounded(*p1, (*p2-*p1).div_float(dist), eps1, dist * (1. - eps2));
    VisibilityTester{ray : r}
  }
  pub fn from_ray(p : &Point, eps : f32, w : Vec3) -> VisibilityTester {
    let r = Ray::new_bounded(*p, w, eps, Float::infinity());
    VisibilityTester{ray : r}
  }
  pub fn unoccluded(&self, scene : &Scene) -> bool {
    match scene.intersect(&self.ray) {
      None => true,
      Some(_) => false
    }
  }
}


pub struct PointLight {
  light_to_world : Transform,
  pos : Point,
  intensity : Spectrum
}

impl PointLight {
  pub fn new(transform : Transform, intensity : Spectrum) -> PointLight {
    let pos = transform.apply_point(&Point::new(0., 0., 0.));
    PointLight {light_to_world : transform, intensity : intensity, pos : pos}
  }
}

impl Light for PointLight {
  fn sample_l(&self, p : &Point, p_epsilon : f32, light_sample : &LightSample) -> (Spectrum, Vec3, f32, VisibilityTester) {
    let wi = (self.pos - *p).normalize();
    let pdf = 1.;
    let visibility = VisibilityTester::from_segment(p, p_epsilon, &self.pos, 0.);
    let spectrum = self.intensity.div_float((self.pos - *p).length_squared());
    (spectrum, wi, pdf, visibility)
  }
  fn is_delta_light(&self) -> bool {
    true
  }
}
