use ab_glyph_rasterizer::{Point, Rasterizer};
use rsdf_core::Image;

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

trait Scale {
  fn scale(&self, scale: f32) -> Self;
}
impl Scale for Point {
  fn scale(&self, scale: f32) -> Self {
    Point {
      x: self.x * scale,
      y: self.y * scale,
    }
  }
}

fn main() {
  const WIDTH: usize = 97;
  const HEIGHT: usize = 86;
  const SCALE: f32 = 5.0;

  let scaled_width = (WIDTH as f32 * SCALE) as usize;
  let scaled_height = (HEIGHT as f32 * SCALE) as usize;

  let mut raster = Rasterizer::new(scaled_width, scaled_height);

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

  for op in ops {
    match op {
      Op::Line(p0, p1) => raster.draw_line(
        Into::<Point>::into(p0).scale(SCALE),
        Into::<Point>::into(p1).scale(SCALE),
      ),
      Op::QuadBezier(p0, p1, p2) => raster.draw_quad(
        Into::<Point>::into(p0).scale(SCALE),
        Into::<Point>::into(p1).scale(SCALE),
        Into::<Point>::into(p2).scale(SCALE),
      ),
      Op::CubicBezier(p0, p1, p2, p3) => raster.draw_cubic(
        Into::<Point>::into(p0).scale(SCALE),
        Into::<Point>::into(p1).scale(SCALE),
        Into::<Point>::into(p2).scale(SCALE),
        Into::<Point>::into(p3).scale(SCALE),
      ),
      Op::BeginContour(..) | Op::EndContour => (),
      _ => panic!("Unknown Op"),
    }
  }

  // output
  let output_filename = "ab_glyph_rasterizer.png";
  let mut image = Image::new(&output_filename, [scaled_width, scaled_height]);
  raster.for_each_pixel_2d(|x, y, alpha| {
    let value = (alpha * 255.0) as u8;
    let (mut r_output, mut g_output, mut b_output) = (13, 17, 23);
    if value > 123 {
      r_output = 255;
      g_output = 255;
      b_output = 255;
    }
    image.set_pixel([x as _, y as _], [r_output, g_output, b_output]);
  });
  image.flush();
}
