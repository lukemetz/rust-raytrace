use shape::Shape;
use geometry::Ray;
use primitive::{Primitive, Intersect, Intersection};
use std::rc::Rc;
use material::Material;

#[test]
use geometry::{Vec3, Point};
#[test]
use transform::Transform;
#[test]
use primitive;
#[test]
use material;
#[test]
use spectrum::Spectrum;

pub struct Aggregator {
  prims: Vec<Box<Primitive>>
}

impl Aggregator {
  pub fn new() -> Aggregator{
    Aggregator {prims: Vec::new()}
  }
  pub fn push(&mut self, obj : Box<Primitive>) {
    self.prims.push(obj);
  }
}

fn closest_intersection(intersections : &Vec<Intersection>) -> Intersection {
  let mut closest = intersections.get(0);
  for intersection in intersections.iter() {
    if closest.t_hit > intersection.t_hit {
      closest = intersection;
    }
  }
  let nonmut : Intersection = *closest;
  nonmut
}

impl Intersect for Aggregator {
  fn intersect(&self, ray : &Ray) -> Option<Intersection> {
    //TODO look into folds here
    //TODO make always return first
    let mut intersections : Vec<Intersection> = Vec::new();
    for prim in self.prims.iter() {
      match prim.intersect(ray) {
        None => (),
        Some(intersect) => intersections.push(intersect)
      }
    }
    match intersections.len() {
       0 => None,
       _ => Some(closest_intersection(&intersections))
    }
  }
}

impl Primitive for Aggregator {
  fn can_intersect(&self) -> bool {
    true
  }

  fn get_shape(&self) -> Option<Rc<Box<Shape>>> {
    None
  }

  fn get_material(&self) -> Option<Rc<Box<Material>>> {
    None
  }
}


#[test]
fn test_Scene() {
  use shape::Sphere;

  let t1 = Transform::translate(Vec3::new(0., 2., 0.));
  let s1 = box Sphere::new(2., t1);
  let m1 = box material::Lambertian::new(Spectrum::new((0.1, 0.2, 0.4)));
  let p1 = box primitive::Geometric::new(s1, m1);

  let t2 = Transform::translate(Vec3::new(0., -2., 0.));
  let s2 = box Sphere::new(2., t2);
  let m2 = box material::Lambertian::new(Spectrum::new((0.1, 0.2, 0.4)));
  let p2 = box primitive::Geometric::new(s2, m2);

  let t3 = Transform::translate(Vec3::new(0., 2., 0.));
  let s3 = box Sphere::new(3., t3);
  let m3 = box material::Lambertian::new(Spectrum::new((0.1, 0.2, 0.4)));
  let p3 = box primitive::Geometric::new(s3, m3);

  let mut agg = Aggregator::new();
  agg.push(p1);
  agg.push(p2);
  agg.push(p3);

  let ray = Ray::new(Point::new(0.,10.,0.), Vec3::new(0., -1., 0.));
  let result = agg.intersect(&ray);
  assert!(result.is_some());
  assert_eq!(result.unwrap().diff_geom.p, Point::new(0., 5., 0.));
}
