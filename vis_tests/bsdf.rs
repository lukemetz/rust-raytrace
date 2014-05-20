extern crate rt = "raytrace_core";
use rt::sampler::{Sampler, RandomSampler};
use rt::camera::{Camera, OrthographicCamera};
use rt::transform::{Transform};
use rt::primitive::{Intersect};
use rt::film::Film;
use rt::spectrum::Spectrum;
use rt::geometry::Vec3;
use rt::filter;
use rt::integrator::{BSDFIntegrator, SurfaceIntegrator};
use rt::renderer::{Renderer, SampleRenderer};
use rt::scene::Scene;

use std::cell::RefCell;

use scene::make_scene;
mod scene;

fn main() {
  let sampler = RandomSampler::new(10, (0, 100), (0, 100));
  let scene = Scene::new(box make_scene());
  let triangle_filter = box filter::Triangle::new(1., 1.);
  let mut film = Film::new((100, 100), triangle_filter);
  //TODO fix the mutability of film
  let film_trash = Film::new((100, 100), box filter::Triangle::new(1., 1.));
  let cam_trans = Transform::translate(Vec3::new(0., 0., -10.));
  let camera = OrthographicCamera::new(cam_trans, (-10., 10., -10., 10.), film_trash);

  let surf_integrator = BSDFIntegrator;

  let renderer = SampleRenderer::new(camera, sampler, surf_integrator);

  let film = renderer.render(&scene, film);

  println!("Writing file");
  let path = Path::new("bsdf.ppm");
  film.write(&path);
}
