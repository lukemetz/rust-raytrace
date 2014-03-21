use camera::CameraSample;
use std::io::{File, Open, Write};

pub mod camera;
pub mod geometry;
//Why is this needed?
pub mod transform;

pub struct Film {
  size : (uint, uint),
  data : ~Vec<(f32, f32, f32)>
}

impl Film {
  pub fn new(size : (uint, uint)) -> Film {
    match size {
      (x, y) => {
        let data = ~Vec::with_capacity(x * y);
        Film {size : size, data : data}
      }
    }
  }

  //TODO speed up with lifetimes
  pub fn get(&self, x : uint, y : uint) -> (f32, f32, f32) {
    let (sx, _) = self.size;
    *self.data.get(y * sx + x)
  }

  pub fn add_sample(&mut self, sample : &CameraSample, color : (f32, f32, f32)) {
    match self.size {
      (x, y) => {
        let nearest_x = ((x as f32) * sample.point.x) as uint;
        let nearest_y = ((y as f32) * sample.point.y) as uint;
        let index = nearest_y * x + nearest_x;
        //TODO fix me to not grow
        self.data.grow_set(index, &(0.,0.,0.), color)
      }
    }
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
        let (r, g, b) = self.get(x, y);
        let (ir, ig, ib) = ((r*255.) as int, (g*255.) as int, (b*255.) as int);
        println!("{}, {}, {}", ir, ig, ib);
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
      let sample = CameraSample::new((x as f32 + 0.5) / 100., (y as f32 + 0.5) / 100.);
      film.add_sample(&sample, (x as f32 / 100., y as f32 / 100., 0.5));
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
