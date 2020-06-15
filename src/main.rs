mod vec3;
mod color;

use crate::vec3::{Color};
use crate::color::{write_color};

fn main() {
  let img_width: i64 = 256;
  let img_height: i64 = 256;
  println!("P3\n{} {}\n255", img_width, img_height);

  for j in (0..img_height).rev() {
    eprintln!("Scanlines Remaining: {}", j);
    for i in 0..img_width {
      let pixel_color = Color::from(
        i as f64/(img_width-1) as f64,
        j as f64/(img_height-1) as f64,
        0.25 as f64
      );

      write_color(pixel_color);
    }
  }

  eprintln!("Done");
}