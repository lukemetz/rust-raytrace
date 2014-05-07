use geometry::Point;

#[deriving(Eq, Clone, Show)]
pub struct Sample {
  pub point : Point //In raster space
}

impl Sample {
  pub fn new(x : f32, y : f32) -> Sample {
    Sample{point : Point::new(x, y, 0.)}
  }
}

