use itertools::izip;
use rsdf_builder::*;
use rsdf_core::*;
use std::env;
use std::fs::File;

fn main() {
  let shape = ShapeBuilder::new()
    .contour((10., 20.))
    // .line((60., 10.))
    .elliptical_arc(22.5, 10., 0., false, true, (55., 20.))
    // .elliptical_arc(25., 10., 0., false, true, (55., 20.))
    .line((50., 30.))
    .quadratic_bezier((40., 50.), (20., 30.))
    .line((10., 20.))
    .end_contour()
    .build();

  let mut shape = ShapeBuilder::new()
    .contour((30., 10.))
    .elliptical_arc(10., 10., 0., true, true, (30., 10.))
    .end_contour()
    .build();



  shape.points[4].y = std::f32::consts::PI;

  dbg!(&shape);

  let Some(filename) = env::args().nth(1) else { panic!("No output filename given") };
  eprintln!("{filename:?}");
  let input_filename = format!("{filename}.png");
  let image = Image::new(&input_filename, [70, 70]);

  gen(image, shape).flush();
  view(&input_filename, &format!("{filename}_render.png"));
}

fn gen(mut image: Image, shape: Shape) -> Image {
  let start_time = std::time::Instant::now();
  for y in 0..image.height {
    for x in 0..image.width {
      let point = Point::from((x as f32, y as f32));
      // let pixel = shape.sample_single_channel(point);
      // let pixel = [pixel, pixel, pixel].map(|sp| distance_color(sp));
      let pixel = shape.sample(point);
      let pixel = pixel.map(|sp| distance_color(sp));
      image.set_pixel([x, y], pixel);
    }
  }

  let duration_time = std::time::Instant::now() - start_time;
  dbg!(duration_time);

  image
}

fn view(input_filename: &str, output_filename: &str) {
  let decoder = png::Decoder::new(File::open(input_filename).unwrap());
  let mut reader = decoder.read_info().unwrap();
  let mut buf = vec![0; reader.output_buffer_size()];
  let info = reader.next_frame(&mut buf).unwrap();

  let bytes = &buf[..info.buffer_size()];

  let sdf_width = info.width as usize;
  let sdf_height = info.height as usize;

  let mut image =
    Image::new(&output_filename, [sdf_width * 100, sdf_height * 100]);

  for y in 0..image.height {
    for x in 0..image.width {
      // normalised coordinates
      let x_norm = x as f32 / (image.width) as f32;
      let y_norm = y as f32 / (image.height) as f32;

      // points in sdf coordinate system
      let x_sdf_p = x_norm * (sdf_width - 1) as f32;
      let y_sdf_p = y_norm * (sdf_height - 1) as f32;

      // sample from points, bilinear
      let pixel = {
        let sample_sdf = |x, y| {
          let offset = (y * sdf_width + x) * 3;
          [bytes[offset], bytes[offset + 1], bytes[offset + 2]]
        };

        let x1 = (x_sdf_p - 0.5).floor();
        let y1 = (y_sdf_p - 0.5).floor();
        let x2 = x1 + 1.;
        let y2 = y1 + 1.;
        let wx = x_sdf_p - x1 - 0.5;
        let wy = y_sdf_p - y1 - 0.5;

        let t1 = sample_sdf(x1 as usize, y1 as usize)
          .map(|v| (1. - wx) * (1. - wy) * v as f32);
        let t2 = sample_sdf(x2 as usize, y1 as usize)
          .map(|v| wx * (1. - wy) * v as f32);
        let t3 = sample_sdf(x1 as usize, y2 as usize)
          .map(|v| (1. - wx) * wy * v as f32);
        let t4 =
          sample_sdf(x2 as usize, y2 as usize).map(|v| wx * wy * v as f32);

        let result: Vec<f32> = izip!(t1, t2, t3, t4)
          .map(|(v1, v2, v3, v4)| v1 + v2 + v3 + v4)
          .collect();

        [result[0] as u8, result[1] as u8, result[2] as u8]
      };

      // find the median value
      let median = |a, b, c| {
        if (a <= b && b <= c) || (c <= b && b <= a) {
          b
        } else if (a <= c && c <= b) || (b <= c && c <= a) {
          c
        } else {
          a
        }
      };
      let value = median(pixel[0], pixel[1], pixel[2]);

      // colour the output based on a simple threshold
      let mut output = 0;
      if value > 120 {
        output = 255;
      }
      // add a red outline effect
      let mut r_output = output;
      if value > 117 {
        r_output = 255;
      }

      image.set_pixel([x, y], [r_output, output, output]);
    }
  }
  image.flush();
}
