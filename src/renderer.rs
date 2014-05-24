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

pub trait Renderer<FilterT> {
  //fn render<FilterT>(&self, scene : &Scene, film : Film<FilterT>) -> Film<FilterT>;
  fn render(&self, scene : &Scene, film : Film<FilterT>) -> Film<FilterT>;
  fn li(&self, scene : &Scene, ray : &Ray, sample : &Sample) -> Spectrum;
}

pub struct SampleRenderer<Camera_T, Sampler_T, SurfaceIntegrator_T, Sample_Iter> {
  camera : Camera_T,
  sampler : Sampler_T,
  surface_integrator : SurfaceIntegrator_T,
}

impl< SampIter : Iterator<Box<Vec<Sample>>>, Cam : Camera<FilterT>, Samp : Sampler<SampIter>, Surf : SurfaceIntegrator, FilterT : Filter>
  SampleRenderer <Cam, Samp, Surf, SampIter> {
  pub fn new (camera : Cam, sampler : Samp, surface_integrator : Surf)
    -> SampleRenderer<Cam, Samp, Surf, SampIter> {
    SampleRenderer {
      camera : camera,
      sampler : sampler,
      surface_integrator : surface_integrator,
    }
  }
}

impl<Cam : Camera<FilterT>, SampIter : Iterator<Box<Vec<Sample>>>, Samp : Sampler<SampIter>, Surf : SurfaceIntegrator, FilterT : Filter>
//impl<Cam, Samp : Sampler<SampIter>, Surf : SurfaceIntegrator, SampIter : Iterator<Box<Vec<Sample>>>>
  Renderer<FilterT> for SampleRenderer<Cam, Samp, Surf, SampIter> {

  fn li(&self, scene : &Scene, ray : &Ray, sample : &Sample) -> Spectrum {
    //Sum the integrators
    let intersect = scene.intersect(ray);
    let surface_li = match intersect {
      Some(intersection) => {
        let render = self as &Renderer<FilterT>;
        self.surface_integrator.li(scene, render, ray, &intersection, sample)
        //self.surface_integrator.li(scene, self, ray, &intersection, sample)
      }
      None => Spectrum::new((0., 0., 0.))
    };
    surface_li
  }

  fn render(&self, scene : &Scene, film : Film<FilterT>) -> Film<FilterT>{
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

