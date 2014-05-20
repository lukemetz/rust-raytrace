extern crate rt = "raytrace_core";
use rt::sampler::{Sampler, RandomSampler};
use rt::camera::{Camera, OrthographicCamera};
use rt::transform::{Transform};
use rt::primitive::{Intersect};
use rt::film::Film;
use rt::spectrum::Spectrum;
use rt::geometry::Vec3;
use rt::filter;

use scene::make_scene;
mod scene;

fn main() {
  let sampler = RandomSampler::new(10, (0, 100), (0, 100));
  let scene = make_scene();
  let triangle_filter = box filter::Triangle::new(1., 1.);
  let mut film = Film::new((100, 100), triangle_filter);
  //TODO fix the mutability of film
  let film_trash = Film::new((100, 100), box filter::Triangle::new(1., 1.));
  let cam_trans = Transform::translate(Vec3::new(0., 0., -10.));
  let camera = OrthographicCamera::new(cam_trans, (-10., 10., -10., 10.), film_trash);

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
