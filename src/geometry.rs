use std::num::Float;

#[deriving(Eq, Clone, Show)]
pub struct Point {
  pub x : f32,
  pub y : f32,
  pub z : f32
}
impl Point{
  pub fn new(x : f32, y : f32, z : f32) -> Point {
    Point{x : x, y : y, z : z}
  }

  pub fn from_vec(vec : Vec3) -> Point {
    Point::new(vec.x, vec.y, vec.z)
  }

  pub fn distance(&self, other : &Point) -> f32 {
    let new = (self - *other);
    let x = new.x;
    let y = new.y;
    let z = new.z;
    (x*x + y*y + z*z).sqrt()
  }

  pub fn normalize(&self) -> Vec3 {
    let scale = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    Vec3::new(self.x / scale, self.y / scale, self.z / scale)
  }
}
impl Add<Point, Point> for Point {
  fn add(&self, other : &Point) -> Point {
    Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
  }
}
impl Sub<Point, Vec3> for Point {
  fn sub(&self, other : &Point) -> Vec3 {
    Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
  }
}

impl Mul<Point, Point> for Point {
  fn mul(&self, other : &Point) -> Point {
    Point::new(self.x * other.x, self.y * other.y, self.z * other.z)
  }
}

#[deriving(Eq, Clone, Show)]
pub struct Vec3 {
  pub x : f32,
  pub y : f32,
  pub z : f32
}
impl Vec3 {
  pub fn new(x : f32, y : f32, z : f32) -> Vec3 {
    Vec3{x : x, y : y, z : z}
  }

  pub fn length_squared(&self) -> f32{
    (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
  }

  pub fn dot_norm(&self, norm : &Normal) -> f32 {
    self.x * norm.x + self.y * norm.y + self.z * norm.z
  }

  pub fn abs_dot_norm(&self, norm : &Normal) -> f32 {
    self.dot_norm(norm).abs()
  }

  pub fn div_float(&self, other : f32) -> Vec3 {
    Vec3::new(self.x/other, self.y/other, self.z/other)
  }

  pub fn normalize(&self) -> Vec3 {
    let scale = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    Vec3::new(self.x / scale, self.y / scale, self.z / scale)
  }
}

impl Add<Vec3, Vec3> for Vec3 {
  fn add(&self, other : &Vec3) -> Vec3 {
    Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
  }
}

impl Neg<Vec3> for Vec3 {
  fn neg(&self) -> Vec3 {
    Vec3::new(-self.x, -self.y, -self.z)
  }
}

#[test]
fn test_vec3_add() {
  assert_eq!(Vec3::new(0.,1.,2.) + Vec3::new(2., 3., 4.), Vec3::new(2., 4., 6.));
}

impl Sub<Vec3, Vec3> for Vec3 {
  fn sub(&self, other : &Vec3) -> Vec3 {
    Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
  }
}
#[test]
fn test_vec3_sub() {
  assert_eq!(Vec3::new(0.,1.,2.) + Vec3::new(2., 3., 4.), Vec3::new(2., 4., 6.));
}

pub trait Scale {
  fn scale(&self, other : f32) -> Self;
}

impl Scale for Vec3 {
  fn scale(&self, other : f32) -> Vec3 {
    Vec3::new(self.x * other, self.y * other, self.z * other)
  }
}

impl Scale for Point {
  fn scale(&self, other : f32) -> Point {
    Point::new(self.x * other, self.y * other, self.z * other)
  }
}

#[deriving(Eq, Clone, Show)]
pub struct Normal {
  pub x : f32,
  pub y : f32,
  pub z : f32
}

impl Normal {
  pub fn new(x : f32, y : f32, z : f32) -> Normal {
    Normal{x : x, y : y, z : z}
  }
  pub fn from_point(point : Point) -> Normal {
    let mag = (point.x*point.x + point.y*point.y + point.z*point.z).sqrt();
    Normal::new(point.x / mag, point.y / mag, point.z / mag)
  }
  pub fn from_vec(vec : Vec3) -> Normal {
    let mag = (vec.x*vec.x + vec.y*vec.y + vec.z*vec.z).sqrt();
    Normal::new(vec.x / mag, vec.y / mag, vec.z / mag)
  }
}

#[deriving(Eq, Clone, Show)]
pub struct Ray {
  pub o : Point,
  pub d : Vec3,
  pub min_t : f32,
  pub max_t : f32
}

impl Ray {
  pub fn new(origin : Point, direction : Vec3) -> Ray {
    Ray {o : origin, d : direction, min_t : 0., max_t : Float::infinity()}
  }
  pub fn new_bounded(origin : Point, direction : Vec3, min_t : f32, max_t : f32) -> Ray {
    Ray {o : origin, d : direction, min_t : min_t, max_t : max_t}
  }
}
