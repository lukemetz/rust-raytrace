use geometry::Point;
pub mod geometry;

#[deriving(Eq, Clone, Show)]
pub struct Sample {
  pub point : Point //In screen space
}

impl Sample {
  pub fn new(x : f32, y : f32) -> Sample {
    Sample{point : Point::new(x, y, 0.)}
  }
}

