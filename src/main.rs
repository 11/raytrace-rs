mod vec3;

fn main() {
  let img_width: i64 = 256;
  let img_height: i64 = 256;
  println!("P3\n{} {}\n255", img_width, img_height);

  for j in (0..img_height).rev() {
    eprintln!("Scanlines Remaining: {}", j);
    for i in 0..img_width {
      let r = i as f64 / (img_width-1) as f64;
      let g = j as f64 / (img_height-1) as f64;
      let b = 0.25 as f64;

      let ir = 255.999 * r;
      let ig = 255.999 * g;
      let ib = 255.999 * b;

      println!("{} {} {}", ir as i32, ig as i32, ib as i32);
    }
  }

  eprintln!("Done");
}