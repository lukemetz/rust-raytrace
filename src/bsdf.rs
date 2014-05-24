use geometry::{Vec3};
use spectrum::Spectrum;
use std::num::Float;
use sample::Sample;

pub trait BSDF {
  fn f(&self, wo : &Vec3, wi: &Vec3) -> Spectrum;
  fn pdf(&self, wo : &Vec3, wi : &Vec3) -> f32;
}

pub struct Lambertian {
  color : Spectrum
}

impl Lambertian {
  pub fn new(color : Spectrum) -> Lambertian {
    Lambertian { color : color }
  }
}

impl BSDF for Lambertian {
  fn f(&self, wo : &Vec3, wi : &Vec3) -> Spectrum {
    self.color.mul_float(Float::frac_1_pi())
  }

  fn pdf(&self, wo : &Vec3, wi : &Vec3) -> f32 {
    //FIXME
    0.
  }
}

pub struct BSDFSample {
  dir : (f32, f32),
  component : f32
}

pub struct BSDFSampleOffsets {
  n_samples : uint,
  component_offset : uint,
  dir_offset : uint
}

impl BSDFSample {
  pub fn from_offset(sample : &Sample, offset : uint) -> BSDFSample{
    let d0 = sample.extra.get(offset);
    let d1 = sample.extra.get(offset + 1);
    let dir = (*d0, *d1);
    let component = sample.extra.get(offset + 2);
    BSDFSample {dir : dir, component : *component}
  }
}

#[test]
fn test_lambertian() {
  let lam = Lambertian::new(Spectrum::new((0.5, 0.2, 0.1)));
  let wo = Vec3::new(0., 1., 0.);
  let wi = Vec3::new(0., 1., 0.);
  let p = Float::pi();
  assert_eq!(lam.f(&wo, &wi), Spectrum::new((0.5/p, 0.2/p, 0.1/p)));
}
