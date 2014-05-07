extern crate rt = "raytrace_core";
use rt::sampler::{Sampler, RandomSampler};
use rt::camera::{Camera, OrthographicCamera};
use rt::transform::{Transform};
use rt::scene::Scene;
use rt::shape::{Sphere, Intersect};
use rt::film::Film;
use rt::spectrum::Spectrum;
use rt::geometry::Vec3;
use rt::filter;

fn make_scene() -> Scene {
  let t1 = Transform::translate(Vec3::new(0., 2., -2.));
  let s1 = ~Sphere::new(2., t1);

  let t2 = Transform::translate(Vec3::new(0., -2., 2.));
  let s2 = ~Sphere::new(6., t2);

  let t3 = Transform::translate(Vec3::new(0., 2., -2.));
  let s3 = ~Sphere::new(3., t3);

  let mut scene = Scene::new();
  scene.push(s1);
  scene.push(s2);
  scene.push(s3);
  scene
}

fn main() {
  let sampler = RandomSampler::new(10, (0, 100), (0, 100));
  let scene = make_scene();
  let triangle_filter = ~filter::Triangle::new(1., 1.);
  let mut film = ~Film::new((100, 100), triangle_filter);
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
