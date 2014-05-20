use scene::Scene;
use sample::Sample;
use geometry::Ray;

use camera::Camera;
use sampler::Sampler;
use integrator::SurfaceIntegrator;
use primitive::Intersect;

use spectrum::Spectrum;
use film::Film;
use filter::Filter;

pub trait Renderer<T> {
  fn render(&self, scene : &Scene, film : T) -> T;
  fn li(&self, scene : &Scene, ray : &Ray, sample : &Sample) -> Spectrum;
}

pub struct SampleRenderer<Camera_T, Sampler_T, SurfaceIntegrator_T, Filt, Sample_Iter> {
  camera : Camera_T,
  sampler : Sampler_T,
  surface_integrator : SurfaceIntegrator_T,
}

impl<Cam, Samp, Surf, Filt, SampIter> SampleRenderer <Cam, Samp, Surf, Filt, SampIter> {
  pub fn new (camera : Cam, sampler : Samp, surface_integrator : Surf)
    -> SampleRenderer<Cam, Samp, Surf, Filt, SampIter> {
    SampleRenderer {
      camera : camera,
      sampler : sampler,
      surface_integrator : surface_integrator,
    }
  }
}

impl<Cam : Camera<Filt>, Samp : Sampler<SampIter>, Surf : SurfaceIntegrator<Film<Filt>>,
  Filt : Filter, SampIter : Iterator<Box<Vec<Sample>>>> Renderer<Film<Filt>>
  for SampleRenderer<Cam, Samp, Surf, Filt, SampIter> {

  fn li(&self, scene : &Scene, ray : &Ray, sample : &Sample) -> Spectrum {
    //Sum the integrators
    let intersect = scene.intersect(ray);
    let surface_li = match intersect {
      Some(intersection) => {
        self.surface_integrator.li(scene, self as &Renderer<Film<Filt>>, ray, &intersection, sample)
      }
      None => Spectrum::new((0., 0., 0.))
    };
    surface_li
  }

  fn render(&self, scene : &Scene, film : Film<Filt>) -> Film<Filt>{
    let mut film = film.clone();
    for samples in self.sampler.mut_iter() {
      for sample in samples.iter() {
        let ray = self.camera.generate_ray(sample);
        let li = self.li(scene, &ray, sample);
        film.add_sample(sample, li);
      }
    }
    film
  }
}

