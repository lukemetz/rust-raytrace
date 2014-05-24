use primitive::{Primitive, Intersect, Intersection};
use geometry::{Ray};
use light::Light;

pub struct Scene {
    pub aggregate : Box<Primitive>,
    pub lights : Vec<Box<Light>>
}

impl Scene {
  pub fn new(aggregate : Box<Primitive>) -> Scene {
    Scene{ aggregate : aggregate,
           lights : vec!() }
  }
}

impl Intersect for Scene {
  fn intersect(&self, ray : &Ray) -> Option<Intersection> {
    self.aggregate.intersect(ray)
  }
}
