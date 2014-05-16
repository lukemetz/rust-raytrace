use geometry::{Vec3};
use spectrum::Spectrum;
use std::num::Float;

pub trait BSDF {
  fn f(&self, wo : Vec3, wi: Vec3) -> Spectrum;
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
  fn f(&self, wo : Vec3, wi : Vec3) -> Spectrum {
    self.color.mul_float(Float::frac_1_pi())
  }
}

#[test]
fn test_lambertian() {
  let lam = Lambertian::new(Spectrum::new((0.5, 0.2, 0.1)));
  let wo = Vec3::new(0., 1., 0.);
  let wi = Vec3::new(0., 1., 0.);
  let p = Float::pi();
  assert_eq!(lam.f(wo, wi), Spectrum::new((0.5/p, 0.2/p, 0.1/p)));
}
