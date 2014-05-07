pub trait Filter {
  fn evaluate(&self, x : f32, y : f32) -> f32;
}

#[deriving(Eq, Clone, Show)]
pub struct Box {
  x_width : f32,
  y_width : f32
}

#[deriving(Eq, Clone, Show)]
pub struct Triangle {
  x_width : f32,
  y_width : f32
}

impl Box {
  pub fn new(x_width : f32, y_width : f32) -> Box {
    Box {x_width : x_width, y_width : y_width}
  }
}

impl Filter for Box {
  fn evaluate(&self, x : f32, y : f32) -> f32 {
    if x > self.x_width/2. || x < -self.x_width/2. {
      0.
    } else if y > self.y_width/2. || y < -self.y_width/2. {
      0.
    } else {
      1.
    }
  }
}

impl Triangle {
  pub fn new(x_width : f32, y_width : f32) -> Triangle {
    Triangle {x_width : x_width, y_width : y_width}
  }
}

impl Filter for Triangle {
  fn evaluate(&self, x : f32, y : f32) -> f32 {
    (self.x_width/2. - x.abs()).max(0.) *
    (self.y_width/2. - y.abs()).max(0.)
  }
}

#[test]
fn test_Box() {
  let box_filter = Box::new(1., 1.);
  assert_eq!(box_filter.evaluate(0.,0.), 1.);
  assert_eq!(box_filter.evaluate(0.4,-0.1), 1.);
  assert_eq!(box_filter.evaluate(0.6,0.), 0.);
}

#[test]
fn test_Triangle() {
  let triangle_filter = Triangle::new(2., 2.);
  assert_eq!(triangle_filter.evaluate(0., 0.), 1.);
  assert_eq!(triangle_filter.evaluate(0.5, 0.), 0.5);
  assert_eq!(triangle_filter.evaluate(0., 0.5), 0.5);
  assert_eq!(triangle_filter.evaluate(0., 1.), 0.);
}
