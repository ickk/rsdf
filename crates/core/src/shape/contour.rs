use crate::*;
use arrayvec::ArrayVec;
use std::ops::{RangeBounds, Bound};

/// The kind of a segment
#[derive(Debug, Clone, Copy)]
pub enum SegmentKind {
  Line,
  QuadBezier,
  CubicBezier,
}

// SegmentKind implicitly gives the length
pub(crate) type SegmentIndex = (SegmentKind, /* points index */ usize);

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
pub(crate) struct Spline<'contour> {
  pub(crate) segments: &'contour [SegmentIndex],
  pub colour: Option<Colour>,
}

impl Spline<'_> {
  pub fn len(&self) -> usize {
    self.segments.len()
  }
}

// A Contour is a path describing a closed region of space.
//
// Sharp corners are assumed to be located at the boundary points of adjacent
// splines.
pub struct Contour {
  /// A buffer containing the points
  pub points: Vec<Point>,
  /// A buffer containing references to the segments
  pub segments: Vec<SegmentIndex>,
  /// A buffer containing references to the splines
  pub splines: Vec<SplineIndex>,
  /// A buffer containing the colours corresponding to the respective Spline.
  ///
  /// Might not be computed.
  pub spline_colours: Option<Vec<Colour>>,
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
  fn get_spline(&self, i: usize) -> Spline {
    let (length, index) = self.splines[i];
    Spline {
      segments: &self.segments[index..index + length],
      colour: self.spline_colours.as_deref().map(|cs| cs[i]),
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

  pub(crate) fn splines(&self) -> impl Iterator<Item = Spline> {
    (0..self.splines.len()).map(|i| self.get_spline(i))
  }

  /// Calculate the signed distance to the spline
  pub(crate) fn spline_distance(
    &self,
    spline: Spline,
    point: Point,
  ) -> (/* dist */ f32, /* orth */ f32) {
    use Segment::*;
    let mut selected_dist = f32::INFINITY;
    // initial values don't matter since the first distance will always be set
    let mut selected_segment = None;
    let mut selected_t = 0.0;

    for segment in self.segments(spline) {
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
    // any dist < infinity are found above.
    let orthogonality = match selected_segment.unwrap() {
      Line(ps) => sample_line_direction(ps, selected_t)
        .signed_area((point - sample_line(ps, selected_t)).norm()),
      QuadBezier(ps) => sample_quad_bezier_direction(ps, selected_t)
        .signed_area((point - sample_quad_bezier(ps, selected_t)).norm()),
      CubicBezier(ps) => sample_cubic_bezier_direction(ps, selected_t)
        .signed_area((point - sample_cubic_bezier(ps, selected_t)).norm()),
    };

    // kind of redundant
    let signed_dist = selected_dist.copysign(orthogonality);

    (signed_dist, orthogonality)
  }

  /// Calculate the signed pseudo distance to the spline
  pub(crate) fn spline_pseudo_distance(
    &self,
    spline: Spline,
    point: Point,
  ) -> (/* dist */ f32) {
    use Segment::*;
    let mut selected_dist = f32::INFINITY;
    let mut selected_segment = None;
    let mut selected_t = 0.0;

    let mut segments = self.segments(spline);
    if let Some(start_segment) = segments.next() {

    }


    for (i, segment) in self.segments(spline).enumerate() {
      // start of the spline
      if i == 0 {
        let (dist, t) = match segment {
          Line(ps) => line_pseudo_distance(ps, point, ..=1.0),
          QuadBezier(ps) => quad_bezier_pseudo_distance(ps, point, ..=1.0),
          CubicBezier(ps) => cubic_bezier_pseudo_distance(ps, point, ..=1.0),
        };
        if dist < selected_dist {
          selected_dist = dist;
          selected_segment = Some(segment);
          selected_t = t;
        }
      }
      // end of the spline
      else if i == spline.len() - 1 {
        let (dist, t) = match segment {
          Line(ps) => line_pseudo_distance(ps, point, 0f32..),
          QuadBezier(ps) => quad_bezier_pseudo_distance(ps, point, 0f32..),
          CubicBezier(ps) => cubic_bezier_pseudo_distance(ps, point, 0f32..),
        };
        if dist < selected_dist {
          selected_dist = dist;
          selected_segment = Some(segment);
          selected_t = t;
        }
      }
      // middle of the spline
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

    let sign = match selected_segment.unwrap() {
      Line(ps) => sample_line_direction(ps, selected_t)
        .signed_area((point - sample_line(ps, selected_t))),
      QuadBezier(ps) => sample_quad_bezier_direction(ps, selected_t)
        .signed_area((point - sample_quad_bezier(ps, selected_t))),
      CubicBezier(ps) => sample_cubic_bezier_direction(ps, selected_t)
        .signed_area((point - sample_cubic_bezier(ps, selected_t))),
    };

    let signed_pseudo_dist = selected_dist.copysign(sign);

    signed_pseudo_dist
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
  for t in find_ts_quad_bezier(ps, point, 0f32..=1f32) {
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
  for t in find_ts_cubic_bezier(ps, point, 0f32..=1f32) {
    let dist = (point - sample_cubic_bezier(ps, t)).abs();
    if dist < selected_dist {
      selected_dist = dist;
      selected_t = t;
    }
  }

  (selected_dist, selected_t)
}

fn range_to_values<R: RangeBounds<f32> + Clone>(range: R) -> (/* start */ f32, /* end */ f32) {
  use Bound::*;
  match (range.start_bound(), range.end_bound()) {
    (Unbounded, Unbounded) => (-f32::INFINITY, f32::INFINITY),
    (Unbounded, Included(&end)) | (Unbounded, Excluded(&end)) => (-f32::INFINITY, end),
    (Included(&start), Unbounded) | (Excluded(&start), Unbounded) => (start, f32::INFINITY),
    (Included(&start), Included(&end)) | (Included(&start), Excluded(&end)) | (Excluded(&start), Excluded(&end)) | (Excluded(&start), Included(&end)) => (start, end),
  }
}

#[inline]
fn line_pseudo_distance<R: RangeBounds<f32> + Clone>(ps: &[Point], point: Point, range: R) -> (/* dist */ f32, /* t */ f32) {
  let (start, end) = range_to_values(range);

  let t = find_t_line(ps, point).clamp(start, end);
  let dist = (point - sample_line(ps, t)).abs();

  (dist, t)
}

#[inline]
fn quad_bezier_pseudo_distance<R: RangeBounds<f32> + Clone>(
  ps: &[Point],
  point: Point,
  range: R,
) -> (/* dist */ f32, /* t */ f32) {
  let mut selected_t = 0.; // initial value doesn't matter
  let mut selected_dist = f32::INFINITY;

  // check perpendiculars
  for t in find_ts_quad_bezier(ps, point, range.clone()) {
    let dist = (point - sample_quad_bezier(ps, t)).abs();
    if dist < selected_dist {
      selected_dist = dist;
      selected_t = t;
    }
  }

  // check any end-points
  let (start, end) = range_to_values(range);
  if start.is_finite() {
    let start_dist = (point - sample_quad_bezier(ps, start)).abs();
    if start_dist < selected_dist {
      selected_dist = start_dist;
      selected_t = start;
    }
  }
  if end.is_finite() {
    let end_dist = (point - sample_quad_bezier(ps, end)).abs();
    if end_dist < selected_dist {
      selected_dist = end_dist;
      selected_t = end;
    }
  }

  (selected_dist, selected_t)
}

#[inline]
fn cubic_bezier_pseudo_distance<R: RangeBounds<f32> + Clone>(
  ps: &[Point],
  point: Point,
  range: R,
) -> (/* dist */ f32, /* t */ f32) {
  let mut selected_t = 0.; // initial value doesn't matter
  let mut selected_dist = f32::INFINITY;

  // check perpendiculars
  for t in find_ts_cubic_bezier(ps, point, ..) {
    let dist = (point - sample_cubic_bezier(ps, t)).abs();
    if dist < selected_dist {
      selected_dist = dist;
      selected_t = t;
    }
  }

  // check any end-points
  let (start, end) = range_to_values(range);
  if start.is_finite() {
    let start_dist = (point - sample_quad_bezier(ps, start)).abs();
    if start_dist < selected_dist {
      selected_dist = start_dist;
      selected_t = start;
    }
  }
  if end.is_finite() {
    let end_dist = (point - sample_quad_bezier(ps, end)).abs();
    if end_dist < selected_dist {
      selected_dist = end_dist;
      selected_t = end;
    }
  }

  (selected_dist, selected_t)
}

#[inline]
fn find_t_line(ps: &[Point], point: Point) -> f32 {
  let v0 = point - ps[0];
  let v1 = ps[1] - ps[0];

  v0.dot(v1) / v1.dot(v1)
}

fn find_ts_quad_bezier<R: RangeBounds<f32>>(ps: &[Point], point: Point, range: R) -> ArrayVec<f32, 4> {
  let v2 = ps[2].as_vector() - 2f32 * ps[1].as_vector() + ps[0].as_vector();
  // check if the curve degenerates into a line
  if v2 == Vector::ZERO {
    let line = [ps[0], ps[1]];
    let mut a = ArrayVec::new();
    a.push(find_t_line(&line, point));
    return a;
  }
  let v0 = point - ps[0];
  let v1 = ps[1] - ps[0];

  let polynomial = [
    -v1.dot(v0),
    2f32 * v1.dot(v1) - v2.dot(v0),
    3f32 * v1.dot(v2),
    v2.dot(v2),
  ];

  roots_in_range(&polynomial, range)
}

fn find_ts_cubic_bezier<R: RangeBounds<f32>>(ps: &[Point], point: Point, range: R) -> ArrayVec<f32, 6> {
  let v0 = point - ps[0];
  let v1 = ps[1] - ps[0];
  let v2 = ps[2].as_vector() - 2f32 * ps[1].as_vector() + ps[0].as_vector();
  let v3 = ps[3].as_vector() - 3f32 * ps[2].as_vector()
    + 3f32 * ps[1].as_vector()
    - ps[0].as_vector();

  let polynomial = [
    -v1.dot(v0),
    3f32 * v1.dot(v1) - 2f32 * v2.dot(v0),
    9f32 * v1.dot(v2) - v2.dot(v0),
    4f32 * v1.dot(v3) + 6f32 * v2.dot(v2),
    5f32 * v2.dot(v3),
    v3.dot(v3),
  ];

  roots_in_range(&polynomial, range)
}

#[inline]
fn sample_line(ps: &[Point], t: f32) -> Point {
  ps[0] + t * (ps[1] - ps[0])
}

#[inline]
#[rustfmt::skip]
fn sample_quad_bezier(ps: &[Point], t: f32) -> Point {
  ps[0]
    + 2f32*t
      * (ps[1] - ps[0])
    + t*t
      * (ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector())
}

#[inline]
#[rustfmt::skip]
fn sample_cubic_bezier(ps: &[Point], t: f32) -> Point {
  ps[0]
    + 3f32*t
      * (ps[1] - ps[0])
    + 3f32*t*t
      * (ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector())
    + t.powi(3)
      * (ps[3].as_vector() - 3f32*ps[2].as_vector() + 3f32*ps[1].as_vector() - ps[0].as_vector())
}

// Return a vector pointing in the dirction of the tangent of a line at time t.
#[inline]
fn sample_line_direction(ps: &[Point], t: f32) -> Vector {
  (ps[1] - ps[0]).norm()
}

// Return a vector pointing in the dirction of the tangent of a quadratic
// bezier at time t.
#[inline]
#[rustfmt::skip]
fn sample_quad_bezier_direction(ps: &[Point], t: f32) -> Vector {
  (
    2f32*(ps[1] - ps[0])
      + 2f32*t
        * (ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector())
  ).norm()
}

// Return a vector pointing in the dirction of the tangent of a cubic bezier at
// time t.
#[inline]
#[rustfmt::skip]
fn sample_cubic_bezier_direction(ps: &[Point], t: f32) -> Vector {
  (
    3f32*(ps[1] - ps[0])
      + 6f32*t*(ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector())
      + 3f32*t*t
        * (ps[3].as_vector() - 3f32*ps[2].as_vector() + 3f32*ps[1].as_vector() - ps[0].as_vector())
  ).norm()
}

#[cfg(any(test, doctest))]
mod tests {
  use float_cmp::assert_approx_eq;

  #[test]
  fn spline_pseudo_distance() {
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
      spline_colours: None,
    };

    let spline = contour.splines().next().unwrap();

    {
      let point = (0., 0.).into();
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = 0.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., 1.).into();
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = -SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., -1.).into();
      // lies exactly on the curve so the sign is undefined
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = 0.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (0.5, 1.5).into();
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = -SQRT_2 / 2.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 3.).into();
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = -1.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 1.5).into();
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = 0.5;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (5., 0.).into();
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = -1. / 5f32.sqrt();
      assert_approx_eq!(f32, dist, expected);
    }
  }

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
      spline_colours: None,
    };

    let spline = contour.splines().next().unwrap();

    {
      let point = (0., 0.).into();
      let (dist, _) = contour.spline_distance(spline, point);
      let expected = 0.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., 1.).into();
      let (dist, _) = contour.spline_distance(spline, point);
      let expected = -SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., -1.).into();
      // lies exactly on the curve so the sign is undefined
      let dist = contour.spline_distance(spline, point).0.abs();
      let expected = SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (0.5, 1.5).into();
      let (dist, _) = contour.spline_distance(spline, point);
      let expected = -SQRT_2 / 2.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 3.).into();
      let (dist, _) = contour.spline_distance(spline, point);
      let expected = -1.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 1.5).into();
      let (dist, _) = contour.spline_distance(spline, point);
      let expected = 0.5;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (5., 0.).into();
      let (dist, _) = contour.spline_distance(spline, point);
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
      spline_colours: None,
    };

    {
      let result: Vec<_> = contour.segments(contour.get_spline(0)).collect();

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
      let result: Vec<_> = contour.segments(contour.get_spline(1)).collect();

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
