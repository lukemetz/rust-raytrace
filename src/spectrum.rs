#[deriving(Eq, Show)]
pub struct Spectrum {
  pub xyz: (f32, f32, f32)
}

impl Spectrum {
  pub fn new(xyz: (f32, f32, f32)) -> Spectrum {
    Spectrum { xyz: xyz}
  }
}
