use std::ops::{
  Add,
  Sub,
  Mul,
  Div,
  Neg,
  AddAssign,
  MulAssign,
  DivAssign
};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vec3 {
  pub fn new() -> Self {
    Vec3 { x:0.0, y:0.0, z:0.0 }
  }

  pub fn from(x: f64, y: f64, z: f64) -> Self {
    Vec3 { x, y, z }
  }

  pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    (u.x * v.x ) +
    (u.y * v.y ) +
    (u.z * v.z )
  }

  pub fn cross(u: &Vec3, v: &Vec3) -> Self {
    Self {
      x: (u.y * u.z - u.z * u.y),
      y: (u.z * u.x - u.x * u.z),
      z: (u.x * u.y - u.y * u.x),
    }
  }

  pub fn len(&self) -> f64 {
    self.len_sqrd().sqrt()
  }

  pub fn len_sqrd(&self) -> f64 {
    (self.x * self.x) +
    (self.y * self.y) +
    (self.z * self.z)
  }
}


impl Add<Vec3> for Vec3 {
  type Output = Vec3;
  fn add(self, other: Vec3) -> Self::Output {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}


impl Sub<Vec3> for Vec3 {
  type Output = Vec3;
  fn sub(self, other: Vec3) -> Self::Output {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z,
    }
  }
}


impl Mul<f64> for Vec3 {
  type Output = Vec3;
  fn mul(self, scalar: f64) -> Self::Output {
    Self {
      x: self.x * scalar,
      y: self.y * scalar,
      z: self.z * scalar,
    }
  }
}


impl Mul<Vec3> for Vec3 {
  type Output = Vec3;
  fn mul(self, other: Vec3) -> Self::Output {
    Self {
      x: self.x * other.x,
      y: self.y * other.y,
      z: self.z * other.z,
    }
  }
}


impl Div<f64> for Vec3 {
  type Output = Vec3;
  fn div(self, scalar: f64) -> Self::Output {
    self * (1_f64 / scalar)
  }
}


impl Neg for Vec3 {
  type Output = Vec3;
  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}


impl AddAssign<Vec3> for Vec3 {
  fn add_assign(&mut self, other: Self) {
    *self = Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    };
  }
}


impl MulAssign<f64> for Vec3 {
  fn mul_assign(&mut self, t: f64) {
    *self = Self {
      x: self.x * t,
      y: self.y * t,
      z: self.z * t,
    };
  }
}


impl DivAssign<f64> for Vec3 {
  fn div_assign(&mut self, t: f64) {
    *self = Self {
      x: self.x * (1 as f64 / t),
      y: self.y * (1 as f64 / t),
      z: self.z * (1 as f64 / t),
    };
  }
}


pub type Point3 = Vec3;
pub type Color = Vec3;