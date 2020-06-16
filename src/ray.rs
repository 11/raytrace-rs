use crate::Vec3;

struct Ray {
  origin: Point3,
  direction: Vec3,
}

impl Ray {

  pub fn new(origin: Point3, direction: Vec3) -> Self{
    Ray { origin, direction }
  }

  pub fn at(&self, t: f64) -> f64 {
    self.origin + (t * self.direction);
  }

}