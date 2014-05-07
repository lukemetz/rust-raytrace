use shape::{Intersect, Intersection};
use geometry::Ray;
#[test]
use geometry::{Vec3, Point};
#[test]
use transform::Transform;

pub struct Scene {
  shapes : Vec<~Intersect>
}

impl Scene {
  pub fn new() -> Scene {
    Scene {shapes : Vec::new()}
  }
  pub fn push(&mut self, obj : ~Intersect) {
    self.shapes.push(obj);
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

impl Intersect for Scene {
  fn intersect(&self, ray : &Ray) -> Option<Intersection> {
    //TODO look into folds here
    //TODO make always return first
    let mut intersections : Vec<Intersection> = Vec::new();
    for shape in self.shapes.iter() {
      match shape.intersect(ray) {
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


#[test]
fn test_Scene() {
  use shape::Sphere;

  let t1 = Transform::translate(Vec3::new(0., 2., 0.));
  let s1 = ~Sphere::new(2., t1);

  let t2 = Transform::translate(Vec3::new(0., -2., 0.));
  let s2 = ~Sphere::new(2., t2);

  let t3 = Transform::translate(Vec3::new(0., 2., 0.));
  let s3 = ~Sphere::new(3., t3);

  let mut scene = Scene::new();
  scene.push(s1);
  scene.push(s2);
  scene.push(s3);

  let ray = Ray::new(Point::new(0.,10.,0.), Vec3::new(0., -1., 0.));
  let result = scene.intersect(&ray);
  assert!(result.is_some());
  assert_eq!(result.unwrap().diff_geom.p, Point::new(0., 5., 0.));
}
