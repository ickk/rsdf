use itertools::izip;
use rsdf_core::*;
use std::env;
use std::fs::File;

fn main() {
  gen();
  view();
}

fn gen() {
  let Some(filename) = env::args().nth(1) else { panic!("No output filename given") };
  eprintln!("{filename:?}");

  let points: Vec<Point> = vec![
    (40., 30.).into(),
    (30., 30.).into(),
    (19., 40.).into(),
    (10., 20.).into(),
    (25., 20.).into(),
    (30., 10.).into(),
    (40., 30.).into(),
  ];

  let contour = Contour {
    points,
    segments: vec![
      (SegmentKind::CubicBezier, 0),
      (SegmentKind::Line, 3),
      (SegmentKind::QuadBezier, 4),
    ],
    splines: vec![(1, 0), (1, 1), (1, 2)],
    spline_colours: vec![Magenta, Yellow, Cyan],
  };

  let shape = Shape {
    contours: vec![contour],
  };

  let mut image = Image::new(&filename, [50, 50]);
  let start_time = std::time::Instant::now();
  for y in 0..image.height {
    for x in 0..image.width {
      let point = Point::from((x as f32, y as f32));
      let pixel = shape.sample(point);
      let pixel = pixel.map(|sp| distance_color(sp));
      image.set_pixel([x, y], pixel);
    }
  }

  let duration_time = std::time::Instant::now() - start_time;
  dbg!(duration_time);

  image.flush();
}

fn view() {
  let Some(input_filename) = env::args().nth(1) else {
    panic!("No input filename given")
  };
  eprintln!("{input_filename:?}");

  let Some(output_filename) = env::args().nth(2) else {
    panic!("No output filename given")
  };
  eprintln!("{output_filename:?}");

  let decoder = png::Decoder::new(File::open(input_filename).unwrap());
  let mut reader = decoder.read_info().unwrap();
  let mut buf = vec![0; reader.output_buffer_size()];
  let info = reader.next_frame(&mut buf).unwrap();

  let bytes = &buf[..info.buffer_size()];

  let sdf_width = info.width as usize;
  let sdf_height = info.height as usize;

  let mut image =
    Image::new(&output_filename, [sdf_width * 10, sdf_height * 10]);

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
      if value > 100 {
        r_output = 255;
      }

      image.set_pixel([x, y], [r_output, output, output]);
    }
  }
  image.flush();
}
