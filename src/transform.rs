//TODO don't re export here?
use geometry::{Vec3, Point, Ray, Normal};
pub mod geometry;

#[deriving(Eq, Clone, Show)]
pub struct Mat4 {
  pub data : ~[f32]
}

impl Mat4 {
  pub fn diag(d : f32) -> Mat4 {
    let mut data = ~[0.0f32, ..16];
    data[0] = d;
    data[5] = d;
    data[10] = d;
    data[15] = d;
    Mat4 { data: data }
  }

  pub fn identity() -> Mat4 {
    Mat4::diag(1.0f32)
  }

  pub fn raw(a : f32, b : f32, c : f32, d : f32,
         e : f32, f : f32, g : f32, h : f32,
         i : f32, j : f32, k : f32, l : f32,
         m : f32, n : f32, o : f32, p : f32) -> Mat4 {
    Mat4 {data : ~[a, b, c, d,
                   e, f, g, h,
                   i, j, k, l,
                   m, n, o, p]}
  }
  
  fn apply_Point(&self, other : &Point) -> Point{
    let x = self.data[0] * other.x + self.data[1] * other.y + self.data[2] * other.z + self.data[3];
    let y = self.data[4] * other.x + self.data[5] * other.y + self.data[6] * other.z + self.data[7];
    let z = self.data[8] * other.x + self.data[9] * other.y + self.data[10] * other.z + self.data[11];
    Point::new(x, y, z)
  }
  
  fn apply_Vec3(&self, other : &Vec3) -> Vec3{
    let x = self.data[0] * other.x + self.data[1] * other.y + self.data[2] * other.z;
    let y = self.data[4] * other.x + self.data[5] * other.y + self.data[6] * other.z;
    let z = self.data[8] * other.x + self.data[9] * other.y + self.data[10]* other.z;
    Vec3::new(x, y, z)
  }
  
  fn apply_Normal(&self, other : &Normal) -> Normal{
    let x = self.data[0] * other.x + self.data[1] * other.y + self.data[2] * other.z;
    let y = self.data[4] * other.x + self.data[5] * other.y + self.data[6] * other.z;
    let z = self.data[8] * other.x + self.data[9] * other.y + self.data[10]* other.z;
    Normal::new(x, y, z)
  }

  fn apply_Ray(&self, other : &Ray) -> Ray{
    Ray{o:self.apply_Point(&other.o),
        d:self.apply_Vec3(&other.d)}
  }

  //TODO make this more functional
  //Implementation ported from pbrt
  fn inverse(&self) -> Option<Mat4> {
    let mut indxc = [0u, ..4];
    let mut indxr = [0u, ..4];
    let mut ipiv = [0u, ..4];
    let mut minv = self.data.to_owned();
    let mut fail = false;
    for i in range(0u, 4u) {
      let mut irow = -1;
      let mut icol = -1;
      let mut big = 0.;
      // choose pivot
      for j in range(0u, 4u) {
        if ipiv[j] != 1 {
          for k in range(0u, 4u) {
            if ipiv[k] == 0 {
              if minv[j*4 + k].abs() >= big {
                big = minv[j*4 + k].abs();
                irow = j;
                icol = k;
              }
            }
            else if ipiv[k] > 1 {
              fail = true;
            }
          }
          if fail { break; }
        }
      }
      if fail { break; }
      ipiv[icol] += 1;
      // swap rows _irow_ and _icol_ for pivot
      if irow != icol {
        for k in range(0u, 4u) {
          let tmp = minv[irow*4 + k];
          minv[irow*4 + k] = minv[icol*4 + k];
          minv[icol*4 + k] = tmp;
        }
      }
      indxr[i] = irow;
      indxc[i] = icol;
      if minv[icol*4 + icol] == 0. {
        fail = true;
        break
      }
      // set $m[icol][icol]$ to one by scaling row _icol_ appropriately
      let pivinv = 1.0f32 / minv[icol*4 + icol];
      minv[icol*4 + icol] = 1.;
      for j in range(0u, 4u) {
        minv[icol*4 + j] *= pivinv;
      }
      // subtract this row from others to zero out their columns
      for j in range(0u, 4u) {
        if j != icol {
          let save = minv[j*4 + icol];
          minv[j*4 + icol] = 0.0;
          for k in range(0u, 4u) {
            minv[j*4 + k] -= minv[icol*4 + k]*save;
          }
        }
      }
    }
    // swap columns to reflect permutation
    for jj in range(0u, 4u) {
      let j = 3-jj;
      if indxr[j] != indxc[j] {
        for k in range(0u, 4u) {
          let tmp = minv[k*4 + indxr[j]];
          minv[k*4 + indxr[j]] = minv[k*4 + indxc[j]];
          minv[k*4 + indxc[j]] = tmp;
        }
      }
    }
    if fail {
      None
    } else {
      Some(Mat4{data:minv})
    }
  }
}

#[test]
fn test_Mat4_diag() {
  let iden = Mat4 {data : ~[2f32, 0f32, 0f32, 0f32,
                            0f32, 2f32, 0f32, 0f32,
                            0f32, 0f32, 2f32, 0f32,
                            0f32, 0f32, 0f32, 2f32]};
  assert_eq!(Mat4::diag(2.0), iden);
}

#[test]
fn test_Mat4_identity() {
  let iden = Mat4 {data : ~[1f32, 0f32, 0f32, 0f32,
                            0f32, 1f32, 0f32, 0f32,
                            0f32, 0f32, 1f32, 0f32,
                            0f32, 0f32, 0f32, 1f32]};
  assert_eq!(Mat4::identity(), iden);
}

#[test]
fn test_Mat4_inverse() {
  let iden = Mat4 {data : ~[2f32, 0f32, 0f32, 0f32,
                            0f32, 2f32, 0f32, 0f32,
                            0f32, 0f32, 2f32, 0f32,
                            0f32, 0f32, 0f32, 2f32]};
  let inv = Mat4 {data : ~[0.5f32, 0f32, 0f32, 0f32,
                            0f32, 0.5f32, 0f32, 0f32,
                            0f32, 0f32, 0.5f32, 0f32,
                            0f32, 0f32, 0f32, 0.5f32]};
  assert_eq!(iden.inverse().unwrap(), inv);

  let iden2 = Mat4 {data : ~[1f32, 0f32, 0f32, 4f32,
                            0f32, 1f32, 0f32, 2f32,
                            0f32, 0f32, 1f32, 0f32,
                            0f32, 0f32, 0f32, 1f32]};
  let inv2 = Mat4 {data : ~[1f32, 0f32, 0f32, -4f32,
                            0f32, 1f32, 0f32, -2f32,
                            0f32, 0f32, 1f32, 0f32,
                            0f32, 0f32, 0f32, 1f32]};
  assert_eq!(iden2.inverse().unwrap(), inv2);
  let failInv = Mat4 {data : ~[1f32, 1f32, 1f32, 1f32,
                                1f32, 1f32, 1f32, 1f32,
                                1f32, 1f32, 1f32, 1f32,
                                1f32, 1f32, 1f32, 1f32]};
  assert_eq!(failInv.inverse(), None);
}

impl Add<Mat4, Mat4> for Mat4 {
  fn add(&self, other : &Mat4) -> Mat4 {
    let mut sum_data = ~[0.0f32, ..16];
    for i in range(0u, 16u) {
      sum_data[i] = other.data[i] + self.data[i];
    }
    Mat4 { data : sum_data }
  }
}
#[test]
fn test_Mat4_add() {
  let m1 = Mat4::raw(1.,  2.,  3.,  4.,
                     5.,  6.,  7.,  8.,
                     9.,  10., 11., 12.,
                     13., 14., 15., 16.);
  let res = Mat4::raw(2.,  4.,  6.,  8.,
                      10., 12., 14., 16.,
                      18., 20., 22., 24.,
                      26., 28., 30., 32.);
  assert_eq!(m1 + m1, res);
}

impl Mul<Mat4, Mat4> for Mat4 {
  fn mul(&self, other : &Mat4) -> Mat4 {
    let mut sum_data = ~[0.0f32, ..16];
    for i in range(0u, 4u) {
      for j in range(0u, 4u) {
        sum_data[i*4+j] = self.data[i*4 + 0] * other.data[0 + j] +
                          self.data[i*4 + 1] * other.data[4 + j] +
                          self.data[i*4 + 2] * other.data[8 + j] +
                          self.data[i*4 + 3] * other.data[12 + j];
      }
    }
    Mat4 { data : sum_data }
  }
}
#[test]
fn test_Mat4_mul() {
  let m1 = Mat4::raw(1.,  2.,  3.,  4.,
                     5.,  6.,  7.,  8.,
                     9.,  10., 11., 12.,
                     13., 14., 15., 16.);
  let res = Mat4::raw(90.,  100., 110., 120.,
                      202., 228., 254., 280.,
                      314., 356., 398., 440.,
                      426., 484., 542., 600.);
  assert_eq!(m1 * m1, res);
}

#[test]
fn test_Mat4_apply_Vec3() {
  let m = Mat4::raw(1.,  2.,  3.,  4.,
                     5.,  6.,  7.,  8.,
                     9.,  10., 11., 12.,
                     13., 14., 15., 16.);
  let v = Vec3::new(1., 2., 3.);
  let res = Vec3::new(14., 38., 62.);
  assert_eq!(m.apply_Vec3(&v), res);
}

#[test]
fn test_Mat4_apply_Point() {
  let m = Mat4::raw(1.,  2.,  3.,  4.,
                     5.,  6.,  7.,  8.,
                     9.,  10., 11., 12.,
                     13., 14., 15., 16.);
  let v = Point::new(1., 2., 3.);
  let res = Point::new(18., 46., 74.);
  assert_eq!(m.apply_Point(&v), res);
}


#[deriving(Eq, Clone, Show)]
pub struct Transform {
  mat : Mat4
}

impl Transform {
  pub fn translate(d : Vec3) -> Transform{
    Transform{ mat : Mat4::raw(1., 0., 0., d.x,
                               0., 1., 0., d.y,
                               0., 0., 1., d.z,
                               0., 0., 0., 1.)
    }
  }

  pub fn apply_Mat4(&self, matrix : Mat4) -> Mat4 {
    self.mat * matrix
  }

  pub fn apply_Vec3(&self, vec : &Vec3) -> Vec3 {
    self.mat.apply_Vec3(vec)
  }

  pub fn apply_Point(&self, point : &Point) -> Point {
    self.mat.apply_Point(point)
  }

  pub fn apply_Normal(&self, point : &Normal) -> Normal{
    self.mat.apply_Normal(point)
  }

  pub fn apply_Ray(&self, ray : &Ray) -> Ray {
    self.mat.apply_Ray(ray)
  }

  pub fn apply_inv_Ray(&self, ray : &Ray) -> Ray {
    self.mat.inverse().unwrap().apply_Ray(ray)
  }

  pub fn scale(amt : f32) -> Transform {
    Transform {mat : Mat4::diag(amt)}
  }
  pub fn identity() -> Transform {
    Transform {mat : Mat4::identity()}
  }
}
