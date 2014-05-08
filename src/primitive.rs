use shape::{Shape, DifferentialGeometry};
use geometry::{Ray};
#[test]
use shape;
#[test]
use geometry::{Vec3, Point, Normal};
#[test]
use transform::Transform;

//use differential_geometry::DifferentialGeometry;
#[deriving(Show, Eq)]
pub struct Intersection {
  pub t_hit : f32,
  pub diff_geom : DifferentialGeometry,
  pub ray_epsilon : f32
}

pub trait Intersect {
  fn intersect(&self, ray : &Ray) -> Option<Intersection>;
}


pub trait Primitive : Intersect {
  fn can_intersect(&self) -> bool;
}

pub struct Geometric {
  shape : ~Shape
}


impl Primitive for Geometric {
  fn can_intersect(&self) -> bool {
    true
  }
}

impl Geometric {
  pub fn new(shape : ~Shape) -> Geometric {
    Geometric { shape : shape }
  }
}

impl Intersect for Geometric {
  fn intersect(&self, ray : &Ray) -> Option<Intersection> {
    match self.shape.intersect(ray) {
      None => None,
      Some((t_hit, ray_epsilon, diff_geom)) => {
        Some(
          Intersection {
            t_hit : t_hit,
            ray_epsilon : ray_epsilon,
            diff_geom : diff_geom
        })
      }
    }
  }
}

#[test]
fn test_Geometric_Intersect() {
  let trans = Transform::translate(Vec3::new(0., 2., 0.));
  let shape = box shape::Sphere::new(4., trans);
  let geometric_prim = Geometric::new(shape);
  let ray = Ray::new(Point::new(0.,10.,0.), Vec3::new(0., -1., 0.));
  let result = geometric_prim.intersect(&ray).unwrap();
  assert_eq!(result.diff_geom.p, Point::new(0., 6., 0.));
  assert_eq!(result.diff_geom.n, Normal::new(0., 1., 0.));
  assert_eq!(result.t_hit, 4.);
}
