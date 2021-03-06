use shape::{Shape, DifferentialGeometry};
use geometry::{Ray};
use std::rc::Rc;
use material::Material;

#[test]
use shape;
#[test]
use geometry::{Vec3, Point, Normal};
#[test]
use transform::Transform;
#[test]
use material;
#[test]
use spectrum::Spectrum;

//use differential_geometry::DifferentialGeometry;
pub trait Intersect {
  fn intersect(&self, ray : &Ray) -> Option<Intersection>;
}


pub trait Primitive : Intersect{
  fn can_intersect(&self) -> bool;
  fn get_shape(&self) -> Option<Rc<Box<Shape>>>;
  fn get_material(&self) -> Option<Rc<Box<Material>>>;
}

pub struct Intersection<'a> {
  pub t_hit : f32,
  pub diff_geom : DifferentialGeometry,
  pub ray_epsilon : f32,
  pub prim : &'a Primitive
}


pub struct Geometric {
  shape : Rc<Box<Shape>>,
  material : Rc<Box<Material>>
}


impl Primitive for Geometric {
  fn can_intersect(&self) -> bool {
    true
  }
  fn get_shape(&self) -> Option<Rc<Box<Shape>>> {
    Some(self.shape.clone())
  }
  fn get_material(&self) -> Option<Rc<Box<Material>>> {
    Some(self.material.clone())
  }
}

impl Geometric {
  pub fn new(shape : Box<Shape>, material : Box<Material>) -> Geometric {
    Geometric {
      shape : Rc::new(shape),
      material : Rc::new(material)
    }
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
            diff_geom : diff_geom,
            prim : self
        })
      }
    }
  }
}

#[test]
fn test_Geometric_Intersect() {
  let trans = Transform::translate(Vec3::new(0., 2., 0.));
  let shape = box shape::Sphere::new(4., trans);
  let material = box material::Lambertian::new(Spectrum::new((0.1, 0.2, 0.3)));
  let geometric_prim = Geometric::new(shape,material);
  let ray = Ray::new(Point::new(0.,10.,0.), Vec3::new(0., -1., 0.));
  let result = geometric_prim.intersect(&ray).unwrap();
  assert_eq!(result.diff_geom.p, Point::new(0., 6., 0.));
  assert_eq!(result.diff_geom.n, Normal::new(0., 1., 0.));
  assert_eq!(result.t_hit, 4.);
}
