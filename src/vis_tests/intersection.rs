extern crate rt = "raytrace_core";
use rt::sampler::{Sampler, RandomSampler};
use rt::camera::{Camera, OrthographicCamera};
use rt::transform::{Transform};
use rt::shape::{Sphere};
use rt::primitive;
use rt::primitive::{Intersect};
use rt::film::Film;
use rt::spectrum::Spectrum;
use rt::geometry::Vec3;
use rt::filter;
use rt::aggregator::Aggregator;

fn make_scene() -> Aggregator {
  let t1 = Transform::translate(Vec3::new(0., 2., -2.));
  let s1 = box Sphere::new(2., t1);
  let p1 = box primitive::Geometric::new(s1);

  let t2 = Transform::translate(Vec3::new(0., -2., 2.));
  let s2 = box Sphere::new(6., t2);
  let p2 = box primitive::Geometric::new(s2);

  let t3 = Transform::translate(Vec3::new(0., 2., -2.));
  let s3 = box Sphere::new(3., t3);
  let p3 = box primitive::Geometric::new(s3);

  let mut agg = Aggregator::new();
  agg.push(p1);
  agg.push(p2);
  agg.push(p3);
  agg
}

fn main() {
  let sampler = RandomSampler::new(10, (0, 100), (0, 100));
  let scene = make_scene();
  let triangle_filter = box filter::Triangle::new(1., 1.);
  let mut film = box Film::new((100, 100), triangle_filter);
  let cam_trans = Transform::translate(Vec3::new(0., 0., -10.));
  let camera = OrthographicCamera::new(cam_trans, (-10., 10., -10., 10.), film);

  for samples in sampler.mut_iter() {
    for sample in samples.iter() {
      let ray = camera.generate_ray(sample);
      let spectrum = match scene.intersect(&ray) {
        None => Spectrum::new((0., 0., 0.)),
        Some(_) => Spectrum::new((1., 1., 1.))
      };
      film.add_sample(sample, spectrum);
    }
  }
  println!("Writing file");
  let path = Path::new("intersection.ppm");
  film.write(&path);
}
