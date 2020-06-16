mod vec3;
mod color;
mod ray;

use crate::vec3::{Color, Point3, Vec3, unit_vector};
use crate::color::{write_color};
use crate::ray::{Ray};


fn ray_color(ray: &Ray) -> Color {
  let unit_direction = unit_vector(&ray.direction);
  let t = 0.5 * (unit_direction.y + 1.0);

   Color::from(1.0, 1.0, 1.0) * (1.0-t) +
   Color::from(0.5, 0.7, 1.0) * t
}


fn main() {
  let aspect_ratio: f64 = 16_f64 / 9_f64;
  let img_width: i64 = 384;
  let img_height: i64 = img_width / aspect_ratio as i64;
  println!("P3\n{} {}\n255", img_width, img_height);

  // config variables
  let viewport_height = 2_f64;
  let viewport_width = aspect_ratio * viewport_height;
  let focal_length = 1_f64;

  let origin = Point3::new();
  let horizontal = Vec3::from(viewport_width, 0_f64, 0_f64);
  let vertical = Vec3::from(0_f64, viewport_height, 0_f64);
  let lower_left_corner = origin - (horizontal / 2_f64) - (vertical / 2_f64) - Vec3::from(0_f64, 0_f64, focal_length);

  for j in (0..img_height).rev() {
    eprintln!("Scanlines Remaining: {}", j);
    for i in 0..img_width {
      let u: f64 = i as f64 / (img_width-1) as f64;
      let v: f64 = j as f64 / (img_height-1) as f64;

      let ray = Ray::new(origin, lower_left_corner + (horizontal * u) + (vertical * v) - origin);
      let pixel_color = ray_color(&ray);

      write_color(pixel_color);
    }
  }

  eprintln!("Done");
}