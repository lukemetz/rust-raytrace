pub trait Filter : Clone{
  fn evaluate(&self, x : f32, y : f32) -> f32;
  fn get_extent(&self) -> (f32, f32);
}

#[deriving(Eq, Clone, Show)]
pub struct Box {
  extent : (f32, f32)
}

#[deriving(Eq, Clone, Show)]
pub struct Triangle {
  extent : (f32, f32)
}

impl Box {
  pub fn new(x_width : f32, y_width : f32) -> Box {
    Box {extent : (x_width, y_width)}
  }
}

impl Filter for Box {
  fn evaluate(&self, x : f32, y : f32) -> f32 {
    let (x_width, y_width) = self.extent;
    if x > x_width || x < -x_width {
      0.
    } else if y > y_width || y < -y_width {
      0.
    } else {
      1.
    }
  }

  fn get_extent(&self) -> (f32, f32) {
    self.extent
  }
}

impl Triangle {
  pub fn new(x_width : f32, y_width : f32) -> Triangle {
    Triangle { extent : (x_width, y_width) }
  }
}

impl Filter for Triangle {
  fn evaluate(&self, x : f32, y : f32) -> f32 {
    let (x_width, y_width) = self.extent;
    (x_width - x.abs()).max(0.) *
    (y_width - y.abs()).max(0.)
  }

  fn get_extent(&self) -> (f32, f32) {
    self.extent
  }
}

#[test]
fn test_Box() {
  let box_filter = Box::new(0.5, 0.5);
  assert_eq!(box_filter.evaluate(0.,0.), 1.);
  assert_eq!(box_filter.evaluate(0.4,-0.1), 1.);
  assert_eq!(box_filter.evaluate(0.6,0.), 0.);
}

#[test]
fn test_Triangle() {
  let triangle_filter = Triangle::new(1., 1.);
  assert_eq!(triangle_filter.evaluate(0., 0.), 1.);
  assert_eq!(triangle_filter.evaluate(0.5, 0.), 0.5);
  assert_eq!(triangle_filter.evaluate(0., 0.5), 0.5);
  assert_eq!(triangle_filter.evaluate(0., 1.), 0.);
}
