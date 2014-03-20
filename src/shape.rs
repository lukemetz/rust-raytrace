use geometry::{Point, Normal, Vec3, Ray, Scale};
use transform::{Transform, Mat4};
use std::fmt;
pub mod transform;
pub mod geometry;

//use differential_geometry::DifferentialGeometry;
#[deriving(Show)]
pub struct IntersectionResult {
  diff_geom : DifferentialGeometry,
  t_hit : f32,
  rayEpsilon : f32
}

pub trait Intersect {
  fn intersect(&self, ray : &Ray) -> Option<IntersectionResult>;
}

#[deriving(Show)]
struct DifferentialGeometry {
  p : Point,
  n : Normal,
  //u : f32, v : f32,
  //dpdu : Vec3, dpdv : Vec3,
  //dndu : Normal, dndv : Normal,
  //shape : ~Shape
}

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
  trans : Transform,
  radius : f32
}
impl Sphere {
  fn new(radius :f32, t : Transform) -> Sphere {
    Sphere{trans:t, radius:radius}
  }
}

impl Shape for Sphere {}


fn quadratic(a : f32, b : f32, c : f32) -> Option<(f32, f32)> {
  let inner = b*b - 4.*a*c;
  if inner > 0. {
    let t0 = (-b + inner.sqrt()) / (2.0 * a);
    let t1 = (-b - inner.sqrt()) / (2.0 * a);
    Some((t0, t1))
  } else {
    None
  }
}

impl Intersect for Sphere {
  fn intersect(&self, ray: &Ray) -> Option<IntersectionResult> {
    let tray = self.trans.apply_inv_Ray(ray);
    
    //Quadratic constants
    let a = tray.d.x*tray.d.x + tray.d.y*tray.d.y + tray.d.z*tray.d.z;
    let b = 2. * (tray.d.x*tray.o.x + tray.d.y*tray.o.y + tray.d.z*tray.o.z);
    let c = tray.o.x*tray.o.x + tray.o.y*tray.o.y +
            tray.o.z*tray.o.z - self.radius*self.radius;
    match quadratic(a, b, c) {
      None => None,
      Some((t0, t1)) => {
        let mut thit = t0;
        if t0 > 0. {
          if t1 > 0. {
            thit = t0.min(t1);
          }
        } else {
          thit = t1;
        }
        if thit > 0. {
          let point = tray.o + Point::from_vec(tray.d.scale(thit));
          let normal = Normal::from_point(tray.o - point);
          println!("THit: {}", thit);
          let diff_geom = DifferentialGeometry{
            p: self.trans.apply_Point(&point),
            n: self.trans.apply_Normal(&normal)
            //u : 0f32, v : 0f32,
            //dpdu : Vec3::new(0f32,0f32,0f32), dpdv : Vec3::new(0f32,0f32,0f32),
            //dndu : Normal::new(0f32,0f32,0f32), dndv : Normal::new(0f32,0f32,0f32)
          };
          let intersect = IntersectionResult{
            diff_geom : diff_geom,
            t_hit : t1,
            rayEpsilon : 1.0
          };
          Some(intersect)
        } else {
          None
        }
      }
    }
  }
}

#[test]
fn test_Sphere_intersect() {
  let trans = Transform::translate(Vec3::new(0., 2., 0.));
  let sphere = Sphere::new(4., trans);
  let ray = Ray::new(Point::new(0.,10.,0.), Vec3::new(0., -1., 0.));
  let result = sphere.intersect(&ray).unwrap();
  assert_eq!(result.diff_geom.p, Point::new(0., 6., 0.));
  assert_eq!(result.diff_geom.n, Normal::new(0., 1., 0.));
  assert_eq!(result.t_hit, 4.);
}
