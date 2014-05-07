use sample::Sample;
use spectrum::Spectrum;
use std::io::{File, Open, Write};

#[test]
use std;

pub struct Film {
  pub size : (uint, uint),
  pub data : Vec<(f32, f32, f32)>
}

impl Film {
  pub fn new(size : (uint, uint)) -> Film {
    match size {
      (x, y) => {
        let data = Vec::from_elem(x * y, (0f32, 0f32, 0f32));
        Film {size : size, data : data}
      }
    }
  }

  //TODO speed up with lifetimes
  pub fn get<'a>(&'a self, x : uint, y : uint) -> &'a (f32, f32, f32) {
    let (sx, _) = self.size;
    self.data.get(y * sx + x)
  }

  //TODO add docs
  pub fn add_sample(&mut self, sample : &Sample, spectrum: Spectrum) {
    match self.size {
      (x, _) => {
        let nearest_x = sample.point.x as uint;
        let nearest_y = sample.point.y as uint;
        let index = nearest_y * x + nearest_x;
        *self.data.get_mut(index) = spectrum.rgb;
      }
    }
  }

  //TODO add docs
  pub fn splat(&mut self, sample : &Sample, spectrum: Spectrum) {
    unimplemented!();
  }

  pub fn write(&self, path : &Path) {

    let (xs, ys) = self.size;
    let header : ~str = format!("P3\n{:u} {:u}\n{:d}\n", xs, ys, 255);
    let mut file = File::open_mode(path, Open, Write).unwrap();

    if file.write_str(header).is_err() {
      println!("Failed to write file");
      return;
    }

    for x in range(0, xs) {
      for y in range(0, ys) {
        let &(r, g, b) = self.get(x, y);
        let (ir, ig, ib) = ((r*255.) as int, (g*255.) as int, (b*255.) as int);
        if file.write_str(format!("{:d} {:d} {:d} ", ir, ig, ib)).is_err() {
          println!("Failed to write file");
          return;
        }
      }
    }
  }
}

#[test]
fn test_Film_write() {
  let mut film = ~Film::new((100, 100));
  for x in range(0, 100) {
    for y in range(0, 100) {
      let sample = Sample::new((x as f32 + 0.5) / 100., (y as f32 + 0.5) / 100.);
      let spectrum = Spectrum::new((x as f32 / 100., y as f32 / 100., 0.5));
      film.add_sample(&sample, spectrum);
    }
  }
  let path = Path::new("unitTestppmWithUniqueName123.ppm");
  film.write(&path);
  let mut file = File::open(&path).unwrap();
  match file.read_to_str() {
    Ok(contents) => {
      assert!(contents.contains("P3\n100 100\n255\n"))
    },
    Err(_) => fail!("Cannot read")
  }
  match std::io::fs::unlink(&path) {
    Ok(_) => {},
    Err(_) => fail!("Cannot remove")
  }
}
