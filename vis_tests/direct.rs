extern crate rt = "raytrace_core";
use rt::sampler::{Sampler, RandomSampler};
use rt::camera::{Camera, OrthographicCamera};
use rt::transform::{Transform};
use rt::primitive::{Intersect};
use rt::film::Film;
use rt::spectrum::Spectrum;
use rt::geometry::Vec3;
use rt::filter;
use rt::integrator::{DirectLightIntegrator, SurfaceIntegrator};
use rt::renderer::{Renderer, SampleRenderer};
use rt::scene::Scene;
use rt::light::{PointLight, Light};

use std::cell::RefCell;

use scene::make_scene;
mod scene;

fn main() {
  let mut scene = Scene::new(box make_scene());
  let transform = Transform::translate(Vec3::new(0., 2., -10.));
  let intensity = Spectrum::new((100., 100., 100.));
  scene.lights.push((box PointLight::new(transform, intensity)) as Box<Light>);

  let triangle_filter = box filter::Triangle::new(1., 1.);
  let mut film = Film::new((100, 100), triangle_filter);
  //TODO fix the mutability of film
  let film_trash = Film::new((100, 100), box filter::Triangle::new(1., 1.));
  let cam_trans = Transform::translate(Vec3::new(0., 0., -10.));
  let camera = OrthographicCamera::new(cam_trans, (-10., 10., -10., 10.), film_trash);

  let mut surf_integrator = DirectLightIntegrator::new();
  let mut sampler = RandomSampler::new(10, (0, 100), (0, 100));
  surf_integrator.request_samples(&mut sampler);
  let renderer = SampleRenderer::new(camera, sampler, surf_integrator);

  let film = renderer.render(&scene, film);

  println!("Writing file");
  let path = Path::new("direct.ppm");
  film.write(&path);
}
