#[deriving(Eq, Clone, Show)]
pub struct Point {
  x : f32,
  y : f32,
  z : f32
}
impl Point{
  pub fn new(x : f32, y : f32, z : f32) -> Point {
    Point{x : x, y : y, z : z}
  }

  pub fn from_vec(vec : Vec3) -> Point {
    Point::new(vec.x, vec.y, vec.z)
  }
}
impl Add<Point, Point> for Point {
  fn add(&self, other : &Point) -> Point {
    Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
  }
}
impl Sub<Point, Point> for Point {
  fn sub(&self, other : &Point) -> Point {
    Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
  }
}

#[deriving(Eq, Clone, Show)]
pub struct Vec3 {
  x : f32,
  y : f32,
  z : f32
}
impl Vec3 {
  pub fn new(x : f32, y : f32, z : f32) -> Vec3 {
    Vec3{x : x, y : y, z : z}
  }
}
impl Add<Vec3, Vec3> for Vec3 {
  fn add(&self, other : &Vec3) -> Vec3 {
    Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
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

//TODO why u know work strange trait...
impl Scale for Vec3 {
  fn scale(&self, other : f32) -> Vec3{
    Vec3::new(self.x * other, self.y * other, self.z * other)
  }
}
#[test]
fn test_vec3_scale() {
  assert_eq!(Vec3::new(0.,1.,2.).scale(2.), Vec3::new(0., 2., 4.));
}

#[deriving(Eq, Clone, Show)]
pub struct Normal {
  x : f32,
  y : f32,
  z : f32
}

impl Normal {
  pub fn new(x : f32, y : f32, z : f32) -> Normal {
    Normal{x : x, y : y, z : z}
  }
  pub fn from_point(point : Point) -> Normal {
    let mag = (point.x*point.x + point.y*point.y + point.z*point.z).sqrt();
    Normal::new(point.x / mag, point.y / mag, point.z / mag)
  }
}

#[deriving(Eq, Clone, Show)]
pub struct Ray {
  o : Point,
  d : Vec3
}

impl Ray {
  pub fn new(origin : Point, direction : Vec3) -> Ray {
    Ray {o : origin, d : direction}
  }
}
