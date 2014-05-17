use primitive::{Primitive, Intersect, Intersection};
use geometry::{Ray};

pub struct Scene {
    aggregate : Box<Primitive>
}

impl Scene {
  pub fn new(aggregate : Box<Primitive>) -> Scene {
    Scene{ aggregate : aggregate }
  }
}

impl Intersect for Scene {
  fn intersect(&self, ray : &Ray) -> Option<Intersection> {
    self.aggregate.intersect(ray)
  }
}
