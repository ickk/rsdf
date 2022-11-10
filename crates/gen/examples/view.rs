use sdf_gen::*;
use std::env;

fn main() {
  do_thing_2()
}

fn do_thing_2() {
  let Some(filename) = env::args().nth(1) else { panic!("No filename given") };
  eprintln!("{filename:?}");

  use std::fs::File;
  // The decoder is a build for reader and can be used to set various decoding options
  // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
  let decoder = png::Decoder::new(File::open(filename).unwrap());
  let mut reader = decoder.read_info().unwrap();
  // Allocate the output buffer.
  let mut buf = vec![0; reader.output_buffer_size()];
  // Read the next frame. An APNG might contain multiple frames.
  let info = reader.next_frame(&mut buf).unwrap();
  // Grab the bytes of the image.
  let bytes = &buf[..info.buffer_size()];

  eprintln!("{bytes:?}");

  let sdf_width = 30;
  let sdf_height = 30;

  let mut image = Image::new("view_image.png", [1000, 1000]);
  for y in 0..image.height {
    for x in 0..image.width {

      // normalised coordinates
      let x_norm = x as f32 / image.width as f32;
      let y_norm = y as f32 / image.height as f32;

      // points in sdf coordinate system
      let x_sdf_p = x_norm * (sdf_width - 1) as f32;
      let y_sdf_p = y_norm * (sdf_height - 1) as f32;

      let sdf = |x, y| -> [u8; 3] {
        let offset = (y * sdf_height + x) * 3;
        [bytes[offset], bytes[offset+1], bytes[offset+2]]
      };

      // sample from points
      let pixel = {
        // floor
        // (x_sdf_p.floor() as usize, y_sdf_p.floor() as usize)
        // bilinear
        let x1 = (x_sdf_p - 0.5).floor();
        let y1 = (y_sdf_p - 0.5).floor();
        let x2 = x1 + 1.;
        let y2 = y1 + 1.;
        let wx = x_sdf_p - x1 - 0.5;
        let wy = y_sdf_p - y1 - 0.5;

        let t1 = sdf(x1 as usize, y1 as usize).map(|v| (1. - wx) * (1. - wy) * v as f32);
        let t2 = sdf(x2 as usize, y1 as usize).map(|v| wx * (1. - wy) * v as f32);
        let t3 = sdf(x1 as usize, y2 as usize).map(|v| (1. - wx) * wy * v as f32);
        let t4 = sdf(x2 as usize, y2 as usize).map(|v| wx * wy * v as f32);

        let result: Vec<f32> = t1.iter().zip(t2).zip(t3).zip(t4).map(|(((v1, v2), v3), v4)| v1 + v2 + v3 + v4).collect();
        [result[0] as u8, result[1] as u8, result[2] as u8]
      };

      let val = std::cmp::max(std::cmp::min(pixel[0], pixel[1]), std::cmp::min(pixel[0], pixel[2]));

      let mut new_val = 0;
      if val > 125 {
        new_val = 255;
      }
      image.set_pixel([x,y], [new_val, new_val, new_val]);


      image.set_pixel([x, y], pixel)
    }
  }
  image.flush();
}


fn do_thing() {
  let Some(filename) = env::args().nth(1) else { panic!("No filename given") };
  eprintln!("{filename:?}");

  use std::fs::File;
  // The decoder is a build for reader and can be used to set various decoding options
  // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
  let decoder = png::Decoder::new(File::open(filename).unwrap());
  let mut reader = decoder.read_info().unwrap();
  // Allocate the output buffer.
  let mut buf = vec![0; reader.output_buffer_size()];
  // Read the next frame. An APNG might contain multiple frames.
  let info = reader.next_frame(&mut buf).unwrap();
  // Grab the bytes of the image.
  let bytes = &buf[..info.buffer_size()];

  eprintln!("{bytes:?}");

  let sdf_width = 30;
  let sdf_height = 30;

  let mut image = Image::new("view_image.png", [30, 30]);
  for y in 0..image.height {
    for x in 0..image.width {
      let offset = (y*image.height + x)*3;
      let pixel = [bytes[offset], bytes[offset+1], bytes[offset+2]];

      let r = pixel[0] != 0;
      let g = pixel[1] != 0;
      let b = pixel[2] != 0;
      let val = std::cmp::max(std::cmp::max(pixel[0], pixel[1]), pixel[2]);

      let mut new_val = 0;
      if r && g || r && b || g && b {
        if val > 100 {
          new_val = 255;
        }
      }

      image.set_pixel([x,y], [new_val, new_val, new_val])
    }
  }
  image.flush();
}

