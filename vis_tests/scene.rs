extern crate rt = "raytrace_core";
use rt::transform::{Transform};
use rt::shape::{Sphere};
use rt::primitive;
use rt::spectrum::Spectrum;
use rt::geometry::Vec3;
use rt::aggregator::Aggregator;
use rt::material;

pub fn make_scene() -> Aggregator {
  let t1 = Transform::translate(Vec3::new(0., 2., -2.));
  let s1 = box Sphere::new(2., t1);
  let m1 = box material::Lambertian::new(Spectrum::new((0.3, 0.2, 0.3)));
  let p1 = box primitive::Geometric::new(s1, m1);

  let t2 = Transform::translate(Vec3::new(0., -2., 2.));
  let s2 = box Sphere::new(6., t2);
  let m2 = box material::Lambertian::new(Spectrum::new((0.1, 0.2, 0.1)));
  let p2 = box primitive::Geometric::new(s2, m2);

  let t3 = Transform::translate(Vec3::new(0., 2., -2.));
  let s3 = box Sphere::new(3., t3);
  let m3 = box material::Lambertian::new(Spectrum::new((0.2, 0.1, 0.3)));
  let p3 = box primitive::Geometric::new(s3, m3);

  let mut agg = Aggregator::new();
  agg.push(p1);
  agg.push(p2);
  agg.push(p3);
  agg
}
