use itertools::izip;
use rsdf_builder::*;
use rsdf_core::{distance_color, Image, Point, Shape};
use std::fs::File;

pub enum Op<P>
where
  P: Into<Point>,
{
  BeginContour(P),
  EndContour,
  Line(P, P),
  QuadBezier(P, P, P),
  CubicBezier(P, P, P, P),
  EllipticalArc {
    p0: P,
    rx: f32,
    ry: f32,
    phi: f32,
    large_arc: bool,
    sweep_ccw: bool,
    p1: P,
  },
}

fn main() {
  const WIDTH: usize = 97;
  const HEIGHT: usize = 86;
  const SCALE: f32 = 5.0;

  let ops = vec![
    Op::BeginContour((48.0, 14.5)),
    Op::CubicBezier((48.0, 14.5), (54.0, 0.5), (80.5, 2.0), (89.0, 17.5)),
    Op::CubicBezier((89.0, 17.5), (81.0, 17.0), (77.0, 20.0), (77.0, 24.0)),
    Op::Line((77.0, 24.0), (77.0, 28.0)),
    Op::Line((77.0, 28.0), (73.0, 28.0)),
    Op::Line((73.0, 28.0), (73.0, 33.0)),
    Op::Line((73.0, 33.0), (77.0, 33.0)),
    Op::Line((77.0, 33.0), (77.0, 48.0)),
    Op::Line((77.0, 48.0), (84.0, 48.0)),
    Op::Line((84.0, 48.0), (84.0, 33.0)),
    Op::Line((84.0, 33.0), (90.0, 33.0)),
    Op::Line((90.0, 33.0), (91.0, 28.0)),
    Op::Line((91.0, 28.0), (84.0, 28.0)),
    Op::Line((84.0, 28.0), (84.0, 25.0)),
    Op::CubicBezier((84.0, 25.0), (84.0, 23.0), (87.0, 21.5), (91.0, 23.0)),
    Op::CubicBezier((91.0, 23.0), (94.5, 33.0), (93.5, 40.0), (85.5, 48.7)),
    Op::Line((85.5, 48.7), (48.0, 80.0)),
    Op::Line((48.0, 80.0), (10.0, 48.0)),
    Op::Line((10.0, 48.0), (20.0, 48.0)),
    Op::Line((20.0, 48.0), (20.0, 44.0)),
    Op::Line((20.0, 44.0), (15.0, 44.0)),
    Op::Line((15.0, 44.0), (15.0, 38.0)),
    Op::QuadBezier((15.0, 38.0), (17.0, 33.0), (21.0, 32.0)),
    Op::Line((21.0, 32.0), (21.0, 36.0)),
    Op::Line((21.0, 36.0), (25.0, 36.0)),
    Op::Line((25.0, 36.0), (26.0, 27.0)),
    Op::QuadBezier((26.0, 27.0), (18.0, 24.5), (14.2, 31.0)),
    Op::Line((14.2, 31.0), (13.5, 27.0)),
    Op::Line((13.5, 27.0), (6.0, 27.0)),
    Op::Line((6.0, 27.0), (6.0, 31.0)),
    Op::Line((6.0, 31.0), (9.0, 31.5)),
    Op::Line((9.0, 31.5), (9.0, 44.0)),
    Op::Line((9.0, 44.0), (7.0, 44.0)),
    Op::CubicBezier((7.0, 44.0), (0.5, 33.0), (0.5, 20.0), (9.0, 12.0)),
    Op::CubicBezier((9.0, 12.0), (20.0, 1.0), (40.0, 0.5), (48.0, 14.5)),
    Op::EndContour,
    Op::BeginContour((44.0, 34.0)),
    Op::Line((44.0, 34.0), (46.0, 29.0)),
    Op::CubicBezier((46.0, 29.0), (38.0, 23.0), (28.5, 26.0), (28.5, 33.0)),
    Op::QuadBezier((28.5, 33.0), (29.0, 38.0), (35.5, 39.0)),
    Op::QuadBezier((35.5, 39.0), (40.0, 39.7), (40.0, 42.0)),
    Op::CubicBezier((40.0, 42.0), (40.0, 44.3), (34.0, 45.0), (30.0, 41.5)),
    Op::Line((30.0, 41.5), (27.0, 46.0)),
    Op::CubicBezier((27.0, 46.0), (34.0, 51.0), (46.5, 50.0), (47.0, 42.0)),
    Op::QuadBezier((47.0, 42.0), (47.0, 37.5), (40.5, 35.5)),
    Op::QuadBezier((40.5, 35.5), (34.0, 34.5), (35.0, 32.0)),
    Op::CubicBezier((35.0, 32.0), (35.5, 30.0), (40.0, 30.0), (44.0, 34.0)),
    Op::EndContour,
    Op::BeginContour((63.0, 46.5)),
    Op::Line((63.0, 46.5), (63.0, 48.0)),
    Op::Line((63.0, 48.0), (69.5, 48.0)),
    Op::Line((69.5, 48.0), (69.5, 18.5)),
    Op::Line((69.5, 18.5), (63.0, 17.0)),
    Op::Line((63.0, 17.0), (63.0, 27.0)),
    Op::QuadBezier((63.0, 27.0), (61.0, 25.6), (57.5, 26.0)),
    Op::CubicBezier((57.5, 26.0), (47.5, 26.5), (47.5, 47.0), (57.5, 49.0)),
    Op::QuadBezier((57.5, 49.0), (61.0, 49.0), (63.0, 46.5)),
    Op::EndContour,
    Op::BeginContour((63.0, 41.0)),
    Op::CubicBezier((63.0, 41.0), (61.0, 45.5), (56.0, 43.0), (56.0, 38.0)),
    Op::CubicBezier((56.0, 38.0), (56.0, 31.0), (61.0, 30.0), (63.0, 33.5)),
    Op::Line((63.0, 33.5), (63.0, 41.0)),
    Op::EndContour,
  ];

  let mut shape_builder = Some(ShapeBuilder::new());
  let mut contour_builder = None;
  for op in ops {
    match op {
      Op::BeginContour(p0) => {
        contour_builder = shape_builder.take().map(|b| b.contour(p0))
      },
      Op::EndContour => {
        shape_builder = contour_builder.take().map(|b| b.end_contour());
      },
      Op::Line(_, p1) => {
        contour_builder = contour_builder.map(|b| b.line(p1));
      },
      Op::QuadBezier(_, p1, p2) => {
        contour_builder = contour_builder.map(|b| b.quadratic_bezier(p1, p2));
      },
      Op::CubicBezier(_, p1, p2, p3) => {
        contour_builder = contour_builder.map(|b| b.cubic_bezier(p1, p2, p3));
      },
      Op::EllipticalArc {
        p0: _,
        rx,
        ry,
        phi,
        large_arc,
        sweep_ccw,
        p1,
      } => {
        contour_builder = contour_builder
          .map(|b| b.elliptical_arc(rx, ry, phi, large_arc, sweep_ccw, p1))
      },
    }
  }
  let shape = shape_builder.unwrap().build();

  // use std::env;
  // let Some(filename) = env::args().nth(1) else { panic!("No output filename given") };
  // let input_filename = format!("{filename}.png");
  // let output_filename = format!("{filename}_render.png");
  // eprintln!("input: {input_filename}");
  // eprintln!("output: {output_filename}");

  let input_filename = "rsdf.png";
  let output_filename = "rsdf_render.png";
  let image = Image::new(&input_filename, [WIDTH, HEIGHT]);
  gen(image, shape).flush();
  view(&input_filename, &output_filename, SCALE);
}

fn gen(mut image: Image, shape: Shape) -> Image {
  let start_time = std::time::Instant::now();
  for y in 0..image.height {
    for x in 0..image.width {
      let point = Point::from((x as f32, y as f32));
      // "single channel"
      // let sample = shape.sample_single_channel(point);
      // let mut color @ [r, g, b] = [sample; 3].map(|sp| distance_color(sp));

      // multi channel
      let sample = shape.sample(point);
      let mut color @ [r, g, b] = sample.map(|sp| distance_color(sp));

      // clip remaining values when bulk is 0
      let sum = r as u16 + g as u16 + b as u16;
      if r as u16 == sum || g as u16 == sum || b as u16 == sum {
        color = [0; 3]
      }

      image.set_pixel([x, y], color);
    }
  }

  let duration_time = std::time::Instant::now() - start_time;
  dbg!(duration_time);

  image
}

fn view(input_filename: &str, output_filename: &str, scale: f32) {
  let decoder = png::Decoder::new(File::open(input_filename).unwrap());
  let mut reader = decoder.read_info().unwrap();
  let mut buf = vec![0; reader.output_buffer_size()];
  let info = reader.next_frame(&mut buf).unwrap();

  let bytes = &buf[..info.buffer_size()];

  let sdf_width = info.width as usize;
  let sdf_height = info.height as usize;

  let mut image = Image::new(
    &output_filename,
    [
      (sdf_width as f32 * scale) as usize,
      (sdf_height as f32 * scale) as usize,
    ],
  );

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

      let (mut r_output, mut g_output, mut b_output) = (13, 17, 23);
      if value > 123 {
        r_output = 255;
        g_output = 255;
        b_output = 255;
      }

      image.set_pixel([x, y], [r_output, g_output, b_output]);
    }
  }
  image.flush();
}
