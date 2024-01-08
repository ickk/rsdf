use itertools::izip;
use rsdf_builder::*;
use rsdf_core::*;
use std::fs::File;

fn main() {
  let shape = ShapeBuilder::new()
    .contour((48.0, 14.5))
    // upper-right of heart
    .cubic_bezier((54., 0.5), (80.5, 2.), (89.0, 17.5))
    // letter 'f'
    .cubic_bezier((81., 17.), (77., 20.), (77., 24.))
    .line((77., 28.))
    .line((73., 28.))
    .line((73., 33.))
    .line((77., 33.))
    .line((77., 48.))
    .line((84., 48.))
    .line((84., 33.))
    .line((90., 33.))
    .line((91., 28.))
    .line((84., 28.))
    .line((84., 25.))
    .cubic_bezier((84., 23.), (87., 21.5), (91., 23.))
    // lower-portion of heart
    .cubic_bezier((94.5, 33.), (93.5, 40.), (85.5, 48.7))
    .line((48., 80.))
    .line((10., 48.))
    // letter 'r'
    .line((20., 48.))
    .line((20., 44.))
    .line((15., 44.))
    .line((15., 38.))
    .quadratic_bezier((17., 33.), (21., 32.))
    .line((21., 36.))
    .line((25., 36.))
    .line((26., 27.))
    .quadratic_bezier((18., 24.5), (14.2, 31.))
    .line((13.5, 27.))
    .line((6., 27.))
    .line((6., 31.))
    .line((9., 31.5))
    .line((9., 44.))
    .line((7., 44.))
    // upper-left of heart
    .cubic_bezier((0.5, 33.), (0.5, 20.), (9., 12.))
    .cubic_bezier((20., 1.), (40., 0.5), (48., 14.5))
    .end_contour()
    // letter 's'
    .contour((44., 34.))
    .line((46., 29.))
    .cubic_bezier((38., 23.), (28.5, 26.), (28.5, 33.))
    .quadratic_bezier((29., 38.), (35.5, 39.))
    .quadratic_bezier((40., 39.7), (40., 42.))
    .cubic_bezier((40., 44.3), (34., 45.), (30., 41.5))
    .line((27., 46.))
    .cubic_bezier((34., 51.), (46.5, 50.), (47., 42.))
    .quadratic_bezier((47., 37.5), (40.5, 35.5))
    .quadratic_bezier((34., 34.5), (35., 32.))
    .cubic_bezier((35.5, 30.), (40., 30.), (44., 34.))
    .end_contour()
    // letter 'd'
    .contour((63., 46.5))
    .line((63., 48.))
    .line((69.5, 48.))
    .line((69.5, 18.5))
    .line((63., 17.))
    .line((63., 27.))
    .quadratic_bezier((61., 25.6), (57.5, 26.))
    .cubic_bezier((47.5, 26.5), (47.5, 47.), (57.5, 49.))
    .quadratic_bezier((61., 49.), (63., 46.5))
    .end_contour()
    .contour((63., 41.))
    .cubic_bezier((61., 45.5), (56., 43.), (56., 38.))
    .cubic_bezier((56., 31.), (61., 30.), (63., 33.5))
    .line((63., 41.))
    .end_contour()
    .build();

  // use std::env;
  // let Some(filename) = env::args().nth(1) else { panic!("No output filename given") };
  // let input_filename = format!("{filename}.png");
  // let output_filename = format!("{filename}_render.png");
  // eprintln!("input: {input_filename}");
  // eprintln!("output: {output_filename}");

  let input_filename = "rsdf.png";
  let output_filename = "rsdf_render.png";
  let image = Image::new(&input_filename, [97, 86]);
  gen(image, shape).flush();
  view(&input_filename, &output_filename);
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

fn view(input_filename: &str, output_filename: &str) {
  let decoder = png::Decoder::new(File::open(input_filename).unwrap());
  let mut reader = decoder.read_info().unwrap();
  let mut buf = vec![0; reader.output_buffer_size()];
  let info = reader.next_frame(&mut buf).unwrap();

  let bytes = &buf[..info.buffer_size()];

  let sdf_width = info.width as usize;
  let sdf_height = info.height as usize;

  let mut image =
    Image::new(&output_filename, [sdf_width, sdf_height]);

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
