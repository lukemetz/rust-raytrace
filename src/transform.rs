mod geometry;

#[deriving(Eq, Clone, Show)]
pub struct Transform {
  mat : geometry::Mat4<f32>
}

impl Transform {
  fn apply_Mat4(&self, matrix : &geometry::Mat4<f32>) -> geometry::Mat4<f32> {
    //TODO check order
    self.mat.mul(matrix)
  }

  /*
  fn apply_Vec3(&self, vec : &geometry::Vec3<f32>) -> geometry::Vec3<f32> {
    self.mat.mul_v(vec)
  }
  */
  
  /*
  fn apply_Point3(&self, point : geometry::Point3<f32>) -> geometry::Point3<f32> {
    self.mat * point
  }*/
}
