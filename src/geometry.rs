#[deriving(Eq, Clone, Show)]
struct Point {
  x : f32,
  y : f32,
  z : f32
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

#[deriving(Eq, Clone, Show)]
struct Normal {
  x : f32,
  y : f32,
  z : f32
}

#[deriving(Eq, Clone, Show)]
pub struct Ray {
  o : Point,
  d : Vec3
}

