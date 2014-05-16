#[deriving(Eq, Show)]
pub struct Spectrum {
  pub xyz: (f32, f32, f32)
}

impl Spectrum {
  pub fn new(xyz: (f32, f32, f32)) -> Spectrum {
    Spectrum { xyz: xyz }
  }

  pub fn mul_float(&self, amt : f32) -> Spectrum {
    match self.xyz {
      (x,y,z) => Spectrum::new((x*amt, y*amt, z*amt))
    }
  }
}
