use crate::*;
use arrayvec::ArrayVec;

/// The kind of a segment
#[derive(Debug, Clone, Copy)]
enum SegmentKind {
  Line,
  QuadBezier,
  CubicBezier,
}

// SegmentKind implicitly gives the length
type SegmentIndex = (SegmentKind, /* points index */ usize);

/// A reference to a segment in the Contour
#[derive(Debug, Clone, Copy, PartialEq)]
enum Segment<'contour> {
  Line(&'contour [Point]),
  QuadBezier(&'contour [Point]),
  CubicBezier(&'contour [Point]),
}

impl Segment<'_> {
  fn kind(&self) -> SegmentKind {
    use Segment::*;
    match self {
      Line(_) => SegmentKind::Line,
      QuadBezier(_) => SegmentKind::QuadBezier,
      CubicBezier(_) => SegmentKind::CubicBezier,
    }
  }
}

type SplineIndex = (/* length */ usize, /* segments index */ usize);

/// A reference to a spline in the Contour
#[derive(Debug, Clone, Copy)]
struct Spline<'contour> {
  segments: &'contour [SegmentIndex],
}

// A Contour is a path describing a closed region of space.
//
// Sharp corners are assumed to be located at the boundary points of adjacent
// splines.
pub struct Contour {
  /// A buffer containing the points
  points: Vec<Point>,
  /// A buffer containing references to the segments
  segments: Vec<SegmentIndex>,
  /// A buffer containing references to the splines
  splines: Vec<SplineIndex>,
  /// A buffer containing the colours corresponding to the respective Spline.
  ///
  /// Might not be computed.
  edge_colours: Option<Vec<Colour>>,
  // TODO: add a flag for fully-smooth. Otherwise there's an ambiguity
  // between teardrop and fully-smooth contours.
}

impl<'contour> Contour {
  #[inline]
  fn get_segment(&self, (kind, i): SegmentIndex) -> Segment {
    match kind {
      SegmentKind::Line => Segment::Line(&self.points[i..i + 2]),
      SegmentKind::QuadBezier => Segment::QuadBezier(&self.points[i..i + 3]),
      SegmentKind::CubicBezier => Segment::CubicBezier(&self.points[i..i + 4]),
    }
  }

  #[inline]
  fn get_spline(&self, (length, index): SplineIndex) -> Spline {
    Spline {
      segments: &self.segments[index..index + length],
    }
  }

  #[inline]
  fn segments(
    &'contour self,
    spline: Spline<'contour>,
  ) -> impl Iterator<Item = Segment> + 'contour {
    spline
      .segments
      .iter()
      .map(|segment_index| self.get_segment(*segment_index))
  }

  fn splines(&self) -> impl Iterator<Item = Spline> {
    self
      .splines
      .iter()
      .map(|&spline_index| self.get_spline(spline_index))
  }

  /// Calculate the signed distance to the spline
  fn spline_distance(&self, spline: Spline, point: Point) -> f32 {
    use Segment::*;
    // (/* dist */ f32, /* pseudo_dist */ f32) {
    let mut selected_dist = f32::INFINITY;
    // initial values don't matter since the first distance will always be set
    let mut selected_segment = None;
    let mut selected_t = 0.0;

    for (segment_index, segment) in self.segments(spline).enumerate() {
      let (dist, t) = match segment {
        Line(ps) => line_distance(ps, point),
        QuadBezier(ps) => quad_bezier_distance(ps, point),
        CubicBezier(ps) => cubic_bezier_distance(ps, point),
      };
      if dist < selected_dist {
        selected_dist = dist;
        selected_segment = Some(segment);
        selected_t = t;
      }
    }

    // unwrap is okay since the selected segment will be always be set assuming
    // any distances < infinity are calculated above.
    let sign = match selected_segment.unwrap() {
      Line(ps) => sample_line_direction(ps, selected_t)
        .signed_area(point - sample_line(ps, selected_t)),
      QuadBezier(ps) => sample_quad_bezier_direction(ps, selected_t)
        .signed_area(point - sample_quad_bezier(ps, selected_t)),
      CubicBezier(ps) => sample_cubic_bezier_direction(ps, selected_t)
        .signed_area(point - sample_cubic_bezier(ps, selected_t)),
    };

    selected_dist.copysign(sign)
  }
}

#[inline]
fn line_distance(ps: &[Point], point: Point) -> (/* dist */ f32, /* t */ f32) {
  let t = find_t_line(ps, point).clamp(0., 1.);
  let dist = (point - sample_line(ps, t)).abs();

  (dist, t)
}

#[inline]
fn quad_bezier_distance(
  ps: &[Point],
  point: Point,
) -> (/* dist */ f32, /* t */ f32) {
  let mut selected_t;
  let mut selected_dist;
  // check endpoints
  {
    // start
    selected_t = 0.0;
    selected_dist = (point - sample_quad_bezier(ps, selected_t)).abs();
    // end
    let end_dist = (point - sample_quad_bezier(ps, 1.0)).abs();
    if end_dist < selected_dist {
      selected_dist = end_dist;
      selected_t = 1.0;
    }
  }
  // check perpendiculars
  for t in find_ts_quad_bezier(ps, point) {
    let dist = (point - sample_quad_bezier(ps, t)).abs();
    if dist < selected_dist {
      selected_dist = dist;
      selected_t = t;
    }
  }

  (selected_dist, selected_t)
}

#[inline]
fn cubic_bezier_distance(
  ps: &[Point],
  point: Point,
) -> (/* dist */ f32, /* t */ f32) {
  let mut selected_t;
  let mut selected_dist;
  // check endpoints
  {
    // start
    selected_t = 0.0;
    selected_dist = (point - sample_cubic_bezier(ps, selected_t)).abs();
    // end
    let end_dist = (point - sample_cubic_bezier(ps, 1.0)).abs();
    if end_dist < selected_dist {
      selected_dist = end_dist;
      selected_t = 1.0;
    }
  }
  // check perpendiculars
  for t in find_ts_cubic_bezier(ps, point) {
    let dist = (point - sample_cubic_bezier(ps, t)).abs();
    if dist < selected_dist {
      selected_dist = dist;
      selected_t = t;
    }
  }

  (selected_dist, selected_t)
}

#[inline]
fn find_t_line(p: &[Point], point: Point) -> f32 {
  let v0 = point - p[0];
  let v1 = p[1] - p[0];

  v0.dot(v1) / v1.dot(v1)
}

fn find_ts_quad_bezier(p: &[Point], point: Point) -> ArrayVec<f32, 4> {
  let v2 = p[2].as_vector() - 2f32 * p[1].as_vector() + p[0].as_vector();
  // check if the curve degenerates into a line
  if v2 == Vector::ZERO {
    let line = [p[0], p[1]];
    let mut a = ArrayVec::new();
    a.push(find_t_line(&line, point));
    return a;
  }
  let v0 = point - p[0];
  let v1 = p[1] - p[0];

  let polynomial = [
    -v1.dot(v0),
    2f32 * v1.dot(v1) - v2.dot(v0),
    3f32 * v1.dot(v2),
    v2.dot(v2),
  ];

  roots_in_range(&polynomial, 0f32..1f32)
}

fn find_ts_cubic_bezier(p: &[Point], point: Point) -> ArrayVec<f32, 6> {
  let v0 = point - p[0];
  let v1 = p[1] - p[0];
  let v2 = p[2].as_vector() - 2f32 * p[1].as_vector() + p[0].as_vector();
  let v3 = p[3].as_vector() - 3f32 * p[2].as_vector()
    + 3f32 * p[1].as_vector()
    - p[0].as_vector();

  let polynomial = [
    -v1.dot(v0),
    3f32 * v1.dot(v1) - 2f32 * v2.dot(v0),
    9f32 * v1.dot(v2) - v2.dot(v0),
    4f32 * v1.dot(v3) + 6f32 * v2.dot(v2),
    5f32 * v2.dot(v3),
    v3.dot(v3),
  ];

  roots_in_range(&polynomial, 0f32..1f32)
}

#[inline]
fn sample_line(p: &[Point], t: f32) -> Point {
  p[0] + t * (p[1] - p[0])
}

#[inline]
#[rustfmt::skip]
fn sample_quad_bezier(p: &[Point], t: f32) -> Point {
  p[0]
    + 2f32*t
      * (p[1] - p[0])
    + t*t
      * (p[2].as_vector() - 2f32*p[1].as_vector() + p[0].as_vector())
}

#[inline]
#[rustfmt::skip]
fn sample_cubic_bezier(p: &[Point], t: f32) -> Point {
  p[0]
    + 3f32*t
      * (p[1] - p[0])
    + 3f32*t*t
      * (p[2].as_vector() - 2f32*p[1].as_vector() + p[0].as_vector())
    + t.powi(3)
      * (p[3].as_vector() - 3f32*p[2].as_vector() + 3f32*p[1].as_vector() - p[0].as_vector())
}

// Return a vector pointing in the dirction of the tangent of a line at time t.
// Could have any magnitude.
#[inline]
fn sample_line_direction(p: &[Point], t: f32) -> Vector {
  (p[1] - p[0]).norm()
}

// Return a vector pointing in the dirction of the tangent of a quadratic
// bezier at time t.
// Could have any magnitude.
#[inline]
#[rustfmt::skip]
fn sample_quad_bezier_direction(p: &[Point], t: f32) -> Vector {
  (
    2f32*(p[1] - p[0])
      + 2f32*t
        * (p[2].as_vector() - 2f32*p[1].as_vector() + p[0].as_vector())
  ).norm()
}

// Return a vector pointing in the dirction of the tangent of a cubic bezier at
// time t.
// Could have any magnitude.
#[inline]
#[rustfmt::skip]
fn sample_cubic_bezier_direction(p: &[Point], t: f32) -> Vector {
  (
    3f32*(p[1] - p[0])
      + 6f32*t*(p[2].as_vector() - 2f32*p[1].as_vector() + p[0].as_vector())
      + 3f32*t*t
        * (p[3].as_vector() - 3f32*p[2].as_vector() + 3f32*p[1].as_vector() - p[0].as_vector())
  ).norm()
}

#[cfg(any(test, doctest))]
mod tests {
  use float_cmp::assert_approx_eq;

  #[test]
  fn spline_distance() {
    use super::*;
    use std::f32::consts::SQRT_2;

    let contour = Contour {
      points: vec![
        (5., -1.).into(),
        (4., 1.).into(),
        (3., 3.).into(),
        (1., 1.).into(),
        (0., 0.).into(),
        (5., -1.).into(),
      ],
      segments: vec![
        (SegmentKind::Line, 0),
        (SegmentKind::QuadBezier, 1),
        (SegmentKind::Line, 3),
        (SegmentKind::Line, 4),
      ],
      splines: vec![(3, 0), (1, 3)],
      edge_colours: None,
    };

    let spline = contour.splines().next().unwrap();

    {
      let point = (0., 0.).into();
      let dist = contour.spline_distance(spline, point);
      let expected = 0.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., 1.).into();
      let dist = contour.spline_distance(spline, point);
      let expected = -SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., -1.).into();
      // lies exactly on the curve so the sign is undefined
      let dist = contour.spline_distance(spline, point).abs();
      let expected = SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (0.5, 1.5).into();
      let dist = contour.spline_distance(spline, point);
      let expected = -SQRT_2 / 2.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 3.).into();
      let dist = contour.spline_distance(spline, point);
      let expected = -1.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 1.5).into();
      let dist = contour.spline_distance(spline, point);
      let expected = 0.5;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (5., 0.).into();
      let dist = contour.spline_distance(spline, point);
      let expected = -1. / 5f32.sqrt();
      assert_approx_eq!(f32, dist, expected);
    }
  }

  #[test]
  fn contour_get_spline_segments() {
    use super::*;
    let contour = Contour {
      points: vec![
        (0., 0.).into(),
        (1., 1.).into(),
        (2., 2.).into(),
        (3., 3.).into(),
        (4., 4.).into(),
        (5., 5.).into(),
        (6., 6.).into(),
        (7., 7.).into(),
        (0., 0.).into(),
      ],
      segments: vec![
        (SegmentKind::Line, 0),
        (SegmentKind::QuadBezier, 1),
        (SegmentKind::CubicBezier, 3),
        (SegmentKind::Line, 6),
        (SegmentKind::Line, 7),
      ],
      splines: vec![(3, 0), (2, 3)],
      edge_colours: None,
    };

    {
      let result: Vec<_> = contour
        .segments(contour.get_spline(contour.splines[0]))
        .collect();

      let s1 = [(0., 0.).into(), (1., 1.).into()];
      let s2 = [(1., 1.).into(), (2., 2.).into(), (3., 3.).into()];
      let s3 = [
        (3., 3.).into(),
        (4., 4.).into(),
        (5., 5.).into(),
        (6., 6.).into(),
      ];

      let expected = vec![
        Segment::Line(&s1),
        Segment::QuadBezier(&s2),
        Segment::CubicBezier(&s3),
      ];

      assert_eq!(result, expected);
    }

    {
      let result: Vec<_> = contour
        .segments(contour.get_spline(contour.splines[1]))
        .collect();

      let s4 = [(6., 6.).into(), (7., 7.).into()];
      let s5 = [(7., 7.).into(), (0., 0.).into()];

      let expected = vec![Segment::Line(&s4), Segment::Line(&s5)];

      assert_eq!(result, expected);
    }
  }

  #[test]
  fn sample_line() {
    use super::*;
    {
      let line = [(0., 0.).into(), (4., 0.).into()];
      {
        let t = 0.;
        let result = sample_line(&line, t);
        let expected = (0., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 0.5;
        let result = sample_line(&line, t);
        let expected = (2., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 1.;
        let result = sample_line(&line, t);
        let expected = (4., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = -1.;
        let result = sample_line(&line, t);
        let expected = (-4., 0.).into();
        assert_eq!(result, expected);
      }
    }
  }

  #[test]
  fn sample_quad_bezier() {
    use super::*;
    {
      let quad = [(0., 0.).into(), (2., 2.).into(), (4., 0.).into()];
      {
        let t = 0.;
        let result = sample_quad_bezier(&quad, t);
        let expected = (0., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 0.5;
        let result = sample_quad_bezier(&quad, t);
        let expected = (2., 1.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 1.;
        let result = sample_quad_bezier(&quad, t);
        let expected = (4., 0.).into();
        assert_eq!(result, expected);
      }
    }
  }

  #[test]
  fn sample_cubic_bezier() {
    use super::*;
    {
      let cubic = [
        (0., 0.).into(),
        (1., 2.).into(),
        (3., 2.).into(),
        (4., 0.).into(),
      ];
      {
        let t = 0.;
        let result = sample_cubic_bezier(&cubic, t);
        let expected = (0., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 0.5;
        let result = sample_cubic_bezier(&cubic, t);
        let expected = (2., 1.5).into();
        assert_eq!(result, expected);
      }
      {
        let t = 1.;
        let result = sample_cubic_bezier(&cubic, t);
        let expected = (4., 0.).into();
        assert_eq!(result, expected);
      }
    }
  }

  #[test]
  fn sample_line_direction() {
    use super::*;
    {
      let line = [(0., 0.).into(), (4., 0.).into()];
      {
        let t = 0.;
        let result = sample_line_direction(&line, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 0.5;
        let result = sample_line_direction(&line, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 1.;
        let result = sample_line_direction(&line, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
    }
    {
      let line = [(0., 0.).into(), (1., 1.).into()];
      {
        let t = 0.;
        let result = sample_line_direction(&line, t).norm();
        let expected = (SQRT_2 / 2., SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 0.5;
        let result = sample_line_direction(&line, t).norm();
        let expected = (SQRT_2 / 2., SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 1.;
        let result = sample_line_direction(&line, t).norm();
        let expected = (SQRT_2 / 2., SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
    }
  }

  #[test]
  fn sample_quad_bezier_direction() {
    use super::*;
    {
      let quad = [(0., 0.).into(), (2., 2.).into(), (4., 0.).into()];
      {
        let t = 0.;
        let result = sample_quad_bezier_direction(&quad, t).norm();
        let expected = (SQRT_2 / 2., SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 0.5;
        let result = sample_quad_bezier_direction(&quad, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 1.;
        let result = sample_quad_bezier_direction(&quad, t).norm();
        let expected = (SQRT_2 / 2., -SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
    }
  }

  #[test]
  fn sample_cubic_bezier_direction() {
    use super::*;
    {
      let cubic = [
        (0., 0.).into(),
        (1., 2.).into(),
        (3., 2.).into(),
        (4., 0.).into(),
      ];
      {
        let t = 0.;
        let result = sample_cubic_bezier_direction(&cubic, t).norm();
        let expected = Vector::from((1., 2.)).norm();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 0.5;
        let result = sample_cubic_bezier_direction(&cubic, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 1.;
        let result = sample_cubic_bezier_direction(&cubic, t).norm();
        let expected = Vector::from((1., -2.)).norm();
        assert_approx_eq!(Vector, result, expected);
      }
    }
  }
}
