extern crate rand;
use sample::Sample;

pub trait Sampler<T : Iterator<Box<Vec<Sample>>>> {
  fn mut_iter(&self) -> T;
  fn add_extra(&mut self, num : uint) -> uint;
}

pub struct RandomSampler {
  max_samples : uint,
  x_range : (int, int),
  y_range : (int, int),
  num_extra : uint
}

pub struct RandomSamplerIter {
  max_samples : uint,
  on_sample: (int, int),
  x_range : (int, int),
  y_range : (int, int),
  num_extra : uint
}

impl RandomSampler {
  pub fn new(max_samples : uint, x_range : (int, int), y_range : (int, int)) -> RandomSampler {
    RandomSampler {
      max_samples: max_samples,
      x_range: x_range,
      y_range: y_range,
      num_extra: 0
    }
  }
}

impl Sampler<RandomSamplerIter> for RandomSampler {
  fn mut_iter(&self) -> RandomSamplerIter {
    RandomSamplerIter {
      on_sample: (self.x_range.val0() - 1, self.y_range.val0()),
      max_samples: self.max_samples,
      x_range: self.x_range,
      y_range: self.y_range,
      num_extra: self.num_extra
    }
  }

  fn add_extra(&mut self, num : uint) -> uint {
    let ret = self.num_extra;
    self.num_extra += num;
    ret
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
impl Iterator<Box<Vec<Sample>>> for RandomSamplerIter {
  fn next(&mut self) -> Option<Box<Vec<Sample>>> {
    match self.next_sample_loc() {
      None => None,
      Some((on_x, on_y)) => {
        let sample_vec = box Vec::from_fn(self.max_samples, |_| -> Sample {
            let delta_x : f32 = rand::random();
            let delta_y : f32 = rand::random();
            let extra = Vec::from_fn(self.num_extra, |x| -> f32 { rand::random() });
            Sample::new((on_x as f32) + delta_x, (on_y as f32) + delta_y, extra)
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

#[test]
fn test_RandomSampler_extras() {
  let mut sampler = RandomSampler::new(3, (0, 10), (0, 10));
  let off1 = sampler.add_extra(3);
  let off2 = sampler.add_extra(2);
  let mut iter = sampler.mut_iter();
  let first = iter.next().unwrap();
  assert_eq!(first.len(), 3);
  assert_eq!(first.get(0).extra.len(), 5);
  assert_eq!(first.get(1).extra.len(), 5);

  for j in range(0u, 3u) {
    for i in range(0u, 5u) {
      assert!(*first.get(j).extra.get(i) < 1. && *first.get(j).extra.get(i) < 1.)
    }
  }
}

