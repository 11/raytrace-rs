use crate::vec3::{Color};

pub fn write_color(pixel_color: Color) {
  println!("{} {} {}",
    (255.999 * pixel_color.x) as i64,
    (255.999 * pixel_color.y) as i64,
    (255.999 * pixel_color.z) as i64,
  );
}