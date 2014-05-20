use sample::Sample;
use spectrum::Spectrum;
use std::io::{File, Open, Write};
use filter::Filter;
use std;
use std::default::Default;

#[test]
use filter;

#[deriving(Clone, Show, Eq)]
pub struct Pixel {
  xyz : (f32, f32, f32),
  weight_sum : f32
}

impl Default for Pixel {
  fn default() -> Pixel {
    Pixel { xyz : (0., 0., 0.), weight_sum : 0. }
  }
}

#[deriving(Clone, Show, Eq)]
pub struct Film<T> {
  pub size : (uint, uint),
  pub data : Vec<Pixel>,
  pub filter : Box<T>
}

impl<T : Filter> Film<T> {
  pub fn new (size : (uint, uint), filter : Box<T>) -> Film<T> {
    match size {
      (x, y) => {
        let data = Vec::from_elem(x * y, Default::default());
        Film {size : size, data : data, filter : filter}
      }
    }
  }

  //TODO speed up with lifetimes
  pub fn get<'a>(&'a self, x : uint, y : uint) -> &'a Pixel {
    let (sx, _) = self.size;
    self.data.get(y * sx + x)
  }

  //TODO add docs
  pub fn add_sample(&mut self, sample : &Sample, spectrum: Spectrum) {
    //FIXME do i need to subtract 0.5 to convert to discrete? see page 408
    let discrete_x = sample.point.x - 0.5;
    let discrete_y = sample.point.y - 0.5;
    let (x_width, y_width) = self.filter.get_extent();
    let x0 = (discrete_x - x_width).ceil() as int;
    let x1 = (discrete_x + x_width).floor() as int;
    let y0 = (discrete_y - y_width).ceil() as int;
    let y1 = (discrete_y + y_width).floor() as int;

    let (sx, sy) = self.size;
    let x0 = std::cmp::max(0, x0) as uint;
    let x1 = std::cmp::min(x1, (sx-1) as int) as uint;
    let y0 = std::cmp::max(0, y0) as uint;
    let y1 = std::cmp::min(y1, (sy-1) as int) as uint;

    for x in range(x0, x1+1) {
      for y in range(y0, y1+1) {
        let weight = self.filter.evaluate((x as f32)-discrete_x, (y as f32)-discrete_y);
        let index = y * sx + x;
        let p = self.data.get_mut(index);
        (*p).xyz = ((*p).xyz.val0() + weight*spectrum.xyz.val0(),
                    (*p).xyz.val1() + weight*spectrum.xyz.val1(),
                    (*p).xyz.val2() + weight*spectrum.xyz.val2());
        (*p).weight_sum += weight;
      }
    }
  }

  //TODO add docs
  pub fn splat(&mut self, sample : &Sample, spectrum: Spectrum) {
    unimplemented!();
  }

  pub fn write(&self, path : &Path) {

    let (xs, ys) = self.size;
    let header = format!("P3\n{:u} {:u}\n{:d}\n", xs, ys, 255);
    let mut file = File::open_mode(path, Open, Write).unwrap();

    if file.write_str(header).is_err() {
      println!("Failed to write file");
      return;
    }

    for x in range(0, xs) {
      for y in range(0, ys) {
        //TODO omake the correct conversion
        let (r, g, b) = self.get(x, y).xyz;
        let w = self.get(x,y).weight_sum;
        let (r, g, b) = (r/w, g/w, b/w);
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
fn test_Film_add_sample() {
  let filter = filter::Triangle::new(1., 1.);
  let mut film = box Film::new((5, 5), box filter);
  let sample = Sample::new(2., 2.);
  let spectrum = Spectrum::new((1., 1., 0.));
  film.add_sample(&sample, spectrum);

  let pixel = Pixel { xyz : (0.25, 0.25, 0.), weight_sum : 0.25 };
  assert_eq!(*film.get(2, 2), pixel);
  let pixel = Pixel { xyz : (0., 0., 0.), weight_sum : 0. };
  assert_eq!(*film.get(3, 2), pixel);

  let sample = Sample::new(2.5, 2.5);
  let spectrum = Spectrum::new((1., 1., 1.));
  film.add_sample(&sample, spectrum);

  let pixel = Pixel { xyz : (1.25, 1.25, 1.), weight_sum : 1.25 };
  assert_eq!(*film.get(2, 2), pixel);
}

#[test]
fn test_Film_write() {
  let filter = filter::Triangle::new(1., 1.);
  let mut film = box Film::new((100, 100), box filter);
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
