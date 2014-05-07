use scene::Scene;
use sample::Sample;
use geometry::Ray;

pub trait Renderer {
  fn render(scene : &Scene);
  fn li(scene : &Scene, ray : &Ray, sample : &Sample) -> Spectrum;
}

pub struct SimpleRenderer;
impl SimpleRender {
  fn new() -> Simple
}

impl Renderer for SimpleRenderer {
  fn li(scene : &Scene, ray : &Ray, sample : &Sample) -> Spectrum {

  }

  fn render(scene : &Scene) {

  }
}

