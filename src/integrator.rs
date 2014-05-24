use scene::Scene;
use renderer::Renderer;
use geometry::{Ray, Point, Normal, Vec3};
use primitive::Intersection;
use sample::Sample;
use spectrum::Spectrum;
use std::rc::Rc;
use shape::DifferentialGeometry;
use light::{LightSample, LightSampleOffsets, Light};
use bsdf::{BSDFSample, BSDFSampleOffsets};
use montecarlo::power_heuristic;
use bsdf::BSDF;
use sampler::Sampler;

pub trait SurfaceIntegrator {
  fn li<FilmT>(&self, scene : &Scene, renderer : &Renderer<FilmT>, ray : &Ray,
                 intersection : &Intersection, sample : &Sample) -> Spectrum;

  fn request_samples<IterT : Iterator<Box<Vec<Sample>>>, SampT : Sampler<IterT>>(&mut self, sampler : &mut SampT) -> ();
}


pub struct BSDFIntegrator;
impl SurfaceIntegrator for BSDFIntegrator {
  fn li<FilmT>(&self, scene : &Scene, renderer : &Renderer<FilmT>, ray : &Ray,
                 intersection : &Intersection, sample : &Sample) -> Spectrum {

    //TODO fix me by sampling output / doing lighting
    let dg = intersection.diff_geom;
    match intersection.prim.get_material() {
      Some(mat) => mat.get_BSDF(&dg, &dg).f(&ray.d, &ray.d),
      None => Spectrum::black()
    }
  }
  fn request_samples<IterT : Iterator<Box<Vec<Sample>>>, SampT : Sampler<IterT>>(&mut self, sampler : &mut SampT) {
  }
}


pub fn estimate_direct<FilmT> (scene : &Scene, renderer : &Renderer<FilmT>, light : &Light, p : &Point, n : &Normal,
                       wo : &Vec3, ray_epsilon : f32, bsdf : &BSDF, light_sample : &LightSample, bsdf_sample : &BSDFSample) -> Spectrum {

  let (li, wi, light_pdf, visibility) = light.sample_l(p, ray_epsilon, light_sample);
  if light_pdf > 0. && !li.is_black() {
    let f = bsdf.f(wo, &wi);
    if !f.is_black() && visibility.unoccluded(scene) {
      if light.is_delta_light() {
        (f * li).mul_float(wi.abs_dot_norm(n)/light_pdf)
      } else {
        let bsdf_pdf = bsdf.pdf(wo, &wi);
        let weight = power_heuristic(1, light_pdf, 1, bsdf_pdf);
        (f * li).mul_float(wi.abs_dot_norm(n) * weight / light_pdf)
      }

    } else {
      Spectrum::black()
    }
  } else {
    Spectrum::black()
  }


}

//TODO possibly bsdf_sample_offset is vector
pub fn uniform_sample_one_light<FilmT> (scene : &Scene, renderer : &Renderer<FilmT>, p : &Point, n : &Normal,
                                wo : &Vec3, ray_epsilon : f32, bsdf : &BSDF, sample : &Sample,
                                light_num_offset : uint, light_sample_offset :uint,
                                bsdf_sample_offset : uint) -> Spectrum {
  let n_lights = scene.lights.len();
  if n_lights > 0 {
    //Last get needed?
    let light_num = (sample.extra.get(light_num_offset) * (n_lights as f32)).floor() as uint;
    let light = scene.lights.get(light_num);
    let light_sample = LightSample::from_offset(sample, light_sample_offset);
    let bsdf_sample = BSDFSample::from_offset(sample, bsdf_sample_offset);
    let spec = estimate_direct(scene, renderer, *light, p, n, wo, ray_epsilon, bsdf, &light_sample, &bsdf_sample);
    spec.mul_float(n_lights as f32)
  } else {
    Spectrum::black()
  }
}


pub struct DirectLightIntegrator {
  light_num_offset : uint,
  light_sample_offset: uint,
  bsdf_sample_offset : uint,
}

impl DirectLightIntegrator {
  pub fn new() -> DirectLightIntegrator {
    DirectLightIntegrator {
      light_num_offset : 0,
      light_sample_offset : 0,
      bsdf_sample_offset : 0,
    }
  }
}

impl SurfaceIntegrator for DirectLightIntegrator {
  fn li<FilmT>(&self, scene : &Scene, renderer : &Renderer<FilmT>, ray : &Ray,
                 intersection : &Intersection, sample : &Sample) -> Spectrum {
    //TODO clean up
    let mut L = Spectrum::new((0., 0., 0.));
    match intersection.prim.get_material() {
      Some(mat) => {
        let dg = intersection.diff_geom;
        let wo = -ray.d;
        let p = dg.p;
        let n = dg.n;
        //TODO fixme
        let bsdf = mat.get_BSDF(&dg, &dg);
        let light_l = uniform_sample_one_light(scene, renderer, &p, &n, &wo, intersection.ray_epsilon,
          bsdf, sample, self.light_num_offset, self.light_sample_offset, self.bsdf_sample_offset);
        light_l
      },
      None => L
    }
  }

  fn request_samples<IterT : Iterator<Box<Vec<Sample>>>, SampT : Sampler<IterT>> (&mut self, sampler : &mut SampT) -> (){
    self.light_num_offset = sampler.add_extra(1);
    self.light_sample_offset = sampler.add_extra(3);
    self.bsdf_sample_offset = sampler.add_extra(3);
  }
}
