extern crate rand;
use sample::Sample;

pub trait Sampler<T : Iterator<~Vec<Sample>>> {
  fn mut_iter(&self) -> T;
}

pub struct RandomSampler {
  max_samples : uint,
  x_range : (int, int),
  y_range : (int, int)
}

pub struct RandomSamplerIter {
  max_samples : uint,
  on_sample: (int, int),
  x_range : (int, int),
  y_range : (int, int)
}

impl RandomSampler {
  pub fn new(max_samples : uint, x_range : (int, int), y_range : (int, int)) -> RandomSampler {
    RandomSampler {
      max_samples: max_samples,
      x_range: x_range,
      y_range: y_range,
    }
  }
}

impl Sampler<RandomSamplerIter> for RandomSampler {
  fn mut_iter(&self) -> RandomSamplerIter {
    RandomSamplerIter {
      on_sample: (self.x_range.val0() - 1, self.y_range.val0()),
      max_samples: self.max_samples,
      x_range: self.x_range,
      y_range: self.y_range
    }
  }
}

impl RandomSamplerIter {
  fn next_sample_loc(&mut self) -> Option<(int, int)> {
    match self.on_sample {
      (on_x, on_y) => {
        if on_x + 1 == self.x_range.val1() {
          if on_y + 1 == self.y_range.val1() {
            None
          } else {
            self.on_sample = (self.x_range.val0(), on_y + 1);
            Some(self.on_sample)
          }
        } else {
          self.on_sample = (on_x + 1, on_y);
          Some(self.on_sample)
        }
      }
    }
  }
}

//TODO make return reference as to not reuse memory
impl Iterator<~Vec<Sample>> for RandomSamplerIter {
  fn next(&mut self) -> Option<~Vec<Sample>> {
    match self.next_sample_loc() {
      None => None,
      Some((on_x, on_y)) => {
        let sample_vec = ~Vec::from_fn(self.max_samples, |_| -> Sample {
            let delta_x : f32 = rand::random();
            let delta_y : f32 = rand::random();
            Sample::new((on_x as f32) + delta_x, (on_y as f32) + delta_y)
        });
        Some(sample_vec)
      }
    }
  }
}

#[test]
fn test_RandomSampler() {
  let sampler = RandomSampler::new(3, (0, 10), (0, 10));
  assert_eq!(sampler.mut_iter().len(), 100);
  let mut iter = sampler.mut_iter();

  let first = iter.next().unwrap();
  assert_eq!(first.len(), 3);
  assert!(first.get(0).point.x < 1. && first.get(0).point.y < 1. && first.get(0).point.z == 0.);
  assert!(first.get(0).point.x > 0. && first.get(0).point.y > 0. && first.get(0).point.z == 0.);

  assert!(first.get(1).point.x < 1. && first.get(1).point.y < 1. && first.get(1).point.z == 0.);
  assert!(first.get(1).point.x > 0. && first.get(1).point.y > 0. && first.get(1).point.z == 0.);

  assert!(first.get(0).point != first.get(1).point);
}

