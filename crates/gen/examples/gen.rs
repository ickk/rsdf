use sdf_gen::*;
use std::env;

fn main() {
  let Some(filename) = env::args().nth(1) else { panic!("No output filename given") };
  eprintln!("{filename:?}");

  use Segment::*;

  let contour_1 = {
    let point_a = Point { x: 14.0, y: 14.0 };
    let point_b = Point { x: 46.0, y: 8.0 };
    let point_c = Point { x: 46.0, y: 20.0 };
    let point_d = Point { x: 30.0, y: 40.0 };

    let segment_ab = Line {
      start: point_a,
      end: point_b,
    };
    let segment_bd = QuadBezier {
      start: point_b,
      control: point_c,
      end: point_d,
    };
    let segment_da = Line {
      start: point_d,
      end: point_a,
    };

    let channels_ab: Channels = 0b101.into();
    let channels_bd: Channels = 0b110.into();
    let channels_da: Channels = 0b011.into();

    Contour {
      segments: vec![segment_ab, segment_bd, segment_da],
      corners: Some(vec![0, 1, 3]),
      channels: Some(vec![channels_ab, channels_bd, channels_da]),
    }
  };

  let contour_2 = {
    let point_d = Point { x: 20.0, y: 16.5 };
    let point_e = Point { x: 40.0, y: 11.5 };
    let point_f = Point { x: 30.0, y: 34.0 };

    let line_df = Line {
      start: point_d,
      end: point_f,
    };
    let line_fe = Line {
      start: point_f,
      end: point_e,
    };
    let line_ed = Line {
      start: point_e,
      end: point_d,
    };

    let channels_df: Channels = 0b101.into();
    let channels_fe: Channels = 0b011.into();
    let channels_ed: Channels = 0b110.into();

    Contour {
      segments: vec![line_df, line_fe, line_ed],
      corners: Some(vec![0, 1, 2]),
      channels: Some(vec![channels_df, channels_fe, channels_ed]),
    }
  };

  let shape = Shape {
    contours: vec![contour_1, contour_2],
  };

  let mut image = Image::new(&filename, [40, 40]);
  let start_time = std::time::Instant::now();
  for y in 0..image.height {
    for x in 0..image.width {
      let point: Point = ((x as f32) + 10., (y as f32) + 5.).into();
      let pixel = shape.sample(point);
      let pixel = pixel.map(|sp| distance_color(sp));
      image.set_pixel([x, y], pixel);
    }
  }

  let duration_time = std::time::Instant::now() - start_time;
  dbg!(duration_time);

  image.flush();
}
