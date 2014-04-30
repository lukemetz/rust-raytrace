
pub struct Spectrum {
  pub rgb : (f32, f32, f32)
}

impl Spectrum {
  pub fn new(rgb : (f32, f32, f32)) -> Spectrum {
    Spectrum { rgb : rgb }
  }
}
