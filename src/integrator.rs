use scene::Scene;
use renderer::Renderer;
use geometry::Ray;
use primitive::Intersection;
use sample::Sample;
use spectrum::Spectrum;
use std::rc::Rc;

pub trait SurfaceIntegrator<FilmT> {
  fn li(&self, scene : &Scene, renderer : &Renderer<FilmT>, ray : &Ray,
                 intersection : &Intersection, sample : &Sample) -> Spectrum;

}


pub struct BSDFIntegrator;
impl<FilmT> SurfaceIntegrator<FilmT> for BSDFIntegrator {
  fn li(&self, scene : &Scene, renderer : &Renderer<FilmT>, ray : &Ray,
                 intersection : &Intersection, sample : &Sample) -> Spectrum {

    //TODO fix me by sampling output / doing lighting
    let dg = intersection.diff_geom;
    match intersection.prim.get_material() {
      Some(mat) => mat.get_BSDF(&dg, &dg).f(ray.d, ray.d),
      None => Spectrum::new((0.,0.,0.))
    }
  }
}
