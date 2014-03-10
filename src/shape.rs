use geometry::{Point3, Normal, Vec3};
use transform;
use geometry;
use std::fmt;
use std::rc::Rc;

//use differential_geometry::DifferentialGeometry;
#[deriving(Show)]
pub struct IntersectionResult {
  diff_geom : DifferentialGeometry,
  tHit : f32,
  rayEpsilon : f32
}

/*impl fmt::Show for IntersectionResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f.buf, "IntersectionResult \\{ diff_geom: {}, tHit: {}, rayEpsilon: {} \\}",
    self.diff_geom, self.tHit, self.rayEpsilon)
  }
} */

pub trait Intersect {
  fn intersect(&self, ray : &geometry::Ray) -> Option<IntersectionResult>;
  fn inter(&self) {
    println!("HERE");
  }
}

#[deriving(Show)]
struct DifferentialGeometry {
  p : Point3<f32>,
  n : Normal<f32>,
  u : f32, v : f32,
  dpdu : Vec3<f32>, dpdv : Vec3<f32>,
  dndu : Normal<f32>, dndv : Normal<f32>,
  //shape : ~Shape
}

/*impl fmt::Show for DifferentialGeometry{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f.buf, "DifferentialGeometry\\{ p: {}, n: {}, shape: {} \\}",
    self.p, self.n, &self.shape)
  }
}*/

pub trait Shape : Intersect {
}

impl fmt::Show for ~Shape {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //TODO fix me so polymorphism works correctly
    write!(f.buf, "Undefined Shape print")
  }
}



//#[deriving( Show)]
pub struct Sphere {
  trans : transform::Transform,
  r : f32
}
impl Shape for Sphere {}

impl Intersect for Sphere {
  fn intersect(&self, ray : &geometry::Ray) -> Option<IntersectionResult> {
    let diff_geom = DifferentialGeometry{
      p: Point3::new(0f32,0f32,0f32),
      n: Normal::new(0f32,1f32,0f32),
      u : 0f32, v : 0f32,
      dpdu : Vec3::new(0f32,0f32,0f32), dpdv : Vec3::new(0f32,0f32,0f32),
      dndu : Vec3::new(0f32,0f32,0f32), dndv : Vec3::new(0f32,0f32,0f32)
    };

    let intersect = IntersectionResult{
      diff_geom : diff_geom,
      tHit : 1.0,
      rayEpsilon : 1.0
    };

    Some(intersect)
  }
}
