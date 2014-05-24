#[deriving(Eq, Show)]
pub struct Spectrum {
  pub xyz: (f32, f32, f32)
}

impl Spectrum {
  pub fn new(xyz: (f32, f32, f32)) -> Spectrum {
    Spectrum { xyz: xyz }
  }

  pub fn black() -> Spectrum {
    Spectrum {xyz : (0., 0., 0.)}
  }

  pub fn blue() -> Spectrum {
    Spectrum {xyz : (0., 0., 1.)}
  }

  pub fn white() -> Spectrum {
    Spectrum {xyz : (1., 1., 1.)}
  }

  pub fn is_black(&self) -> bool {
    if self.xyz == (0.,0.,0.) {
      true
    } else {
      false
    }
  }

  pub fn mul_float(&self, amt : f32) -> Spectrum {
    match self.xyz {
      (x,y,z) => Spectrum::new((x*amt, y*amt, z*amt))
    }
  }

  pub fn div_float(&self, amt : f32) -> Spectrum {
    match self.xyz {
      (x,y,z) => Spectrum::new((x/amt, y/amt, z/amt))
    }
  }
}

impl Mul <Spectrum, Spectrum> for Spectrum {
  fn mul(&self, other : &Spectrum) -> Spectrum {
    let (x,y,z) = self.xyz;
    let (xx,yy,zz) = other.xyz;
    Spectrum::new((x*xx, y*yy, z*zz))
  }
}

impl Add <Spectrum, Spectrum> for Spectrum {
  fn add(&self, other : &Spectrum) -> Spectrum {
    let (x,y,z) = self.xyz;
    let (xx,yy,zz) = other.xyz;
    Spectrum::new((x+xx, y+yy, z+zz))
  }
}
