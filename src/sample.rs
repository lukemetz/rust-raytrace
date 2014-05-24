use geometry::Point;

#[deriving(Eq, Clone, Show)]
pub struct Sample {
  pub point : Point, //In raster space
  pub extra : Vec<f32>
}

impl Sample {
  pub fn new(x : f32, y : f32, extra : Vec<f32>) -> Sample {
    Sample{
      point : Point::new(x, y, 0.),
      extra: extra
    }
  }
}

