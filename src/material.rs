use shape::{DifferentialGeometry};
use bsdf;
use spectrum::Spectrum;

pub trait Material {
  fn get_BSDF(&self, geom_dg : &DifferentialGeometry, shading_dg : &DifferentialGeometry) -> Box<bsdf::BSDF>;
}

pub struct Lambertian {
  bsdf : bsdf::Lambertian
}

impl Lambertian {
  pub fn new(color : Spectrum) -> Lambertian {
    Lambertian{bsdf : bsdf::Lambertian::new(color)}
  }
}

impl Material for Lambertian {
  fn get_BSDF(&self, geom_dg : &DifferentialGeometry, shading_dg : &DifferentialGeometry) -> Box<bsdf::BSDF> {
    box self.bsdf as Box<bsdf::BSDF>
  }
}
