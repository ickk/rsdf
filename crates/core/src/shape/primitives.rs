pub mod cubic_bezier;
pub mod elliptical_arc;
pub mod line;
pub mod quad_bezier;

pub use cubic_bezier::*;
pub use elliptical_arc::*;
pub use line::*;
pub use quad_bezier::*;

use crate::*;
use arrayvec::ArrayVec;
use std::ops::{Bound, RangeBounds};

/// The primitive type of a [`Segment`]
#[derive(Debug, Clone, Copy)]
pub enum SegmentKind {
  /// Line, consisting of 2 [`Point`] - starting, ending positions.
  Line,
  /// Degree 2 bezier curve, consisting of 3 [`Point`] - starting, control,
  /// ending positions.
  QuadBezier,
  /// Degree 3 bezier curve, consisting of 4 [`Point`] - starting, control,
  /// ending positions.
  CubicBezier,
  /// Elliptical arc, consists of 4 [`Points`]
  /// - (centre_x, centre_y), (radius, aspect_ratio), (phi, _), (theta, delta)
  EllipticalArc,
}

impl Shape {
  /// Get a segment given a `SegmentRef`
  #[inline]
  #[rustfmt::skip]
  pub fn get_segment(
    &self,
    SegmentRef{kind, points_index: i}: SegmentRef
  ) -> Segment {
    match kind {
      SegmentKind::Line => Segment::Line(&self.points[i..i + 2]),
      SegmentKind::QuadBezier => Segment::QuadBezier(&self.points[i..i + 3]),
      SegmentKind::CubicBezier => Segment::CubicBezier(&self.points[i..i + 4]),
      SegmentKind::EllipticalArc => Segment::EllipticalArc(&self.points[i..i+4]),
    }
  }
}

/// A reference to a segment in the Contour
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Segment<'contour> {
  Line(&'contour [Point]),
  QuadBezier(&'contour [Point]),
  CubicBezier(&'contour [Point]),
  EllipticalArc(&'contour [Point]),
}

impl Segment<'_> {
  /// Sample the segment to get its value at time `t`
  #[inline]
  pub fn sample(self, t: f32) -> Point {
    let clamped_t = t.clamp(0f32, 1f32);
    let sample = match self {
      Segment::Line(ps) => Line::sample(ps, clamped_t),
      Segment::QuadBezier(ps) => QuadBezier::sample(ps, clamped_t),
      Segment::CubicBezier(ps) => CubicBezier::sample(ps, clamped_t),
      Segment::EllipticalArc(ps) => EllipticalArc::sample(ps, clamped_t),
    };
    // We handle the cases for t < 0 & t > 1, and sample the ray extending
    // along the straight line from the ends of the segment instead.
    if t < 0f32 {
      let p0 = sample;
      let p1 = p0 + self.sample_derivative(clamped_t);
      let line = [p0, p1];
      Line::sample(&line, t)
    } else if t > 0f32 {
      let p1 = sample;
      let p0 = p1 - self.sample_derivative(clamped_t);
      let line = [p0, p1];
      Line::sample(&line, t)
    } else {
      sample
    }
  }

  /// Return a tangent to the segment at time `t`
  #[inline]
  pub fn sample_derivative(self, t: f32) -> Vector {
    // t can be clamped to [0,1], since the derivative of pseudo distance
    // > 1 will be the same as =1 (since it's a straight line extending
    // beyond the end of the curve). ditto for the < 0.
    let t = t.clamp(0f32, 1f32);
    match self {
      Segment::Line(ps) => Line::sample_derivative(ps, t),
      Segment::QuadBezier(ps) => QuadBezier::sample_derivative(ps, t),
      Segment::CubicBezier(ps) => CubicBezier::sample_derivative(ps, t),
      Segment::EllipticalArc(ps) => EllipticalArc::sample_derivative(ps, t),
    }
  }

  /// Get the pseudo-distance from a point to the primitive at time `t`, where
  /// `t` is contained within the given `range`.
  #[inline]
  pub fn pseudo_distance<R: RangeBounds<f32> + Clone>(
    self,
    point: Point,
    range: R,
  ) -> (/* dist */ f32, /* t */ f32) {
    match self {
      Segment::Line(ps) => Line::pseudo_distance(ps, point, range),
      Segment::QuadBezier(ps) => QuadBezier::pseudo_distance(ps, point, range),
      Segment::CubicBezier(ps) => {
        CubicBezier::pseudo_distance(ps, point, range)
      },
      Segment::EllipticalArc(ps) => {
        EllipticalArc::pseudo_distance(ps, point, range)
      },
    }
  }

  /// Get the distance from a point to the segment at time `t`
  #[inline]
  pub fn distance(self, point: Point) -> (/* dist */ f32, /* t */ f32) {
    match self {
      Segment::Line(ps) => Line::distance(ps, point),
      Segment::QuadBezier(ps) => QuadBezier::distance(ps, point),
      Segment::CubicBezier(ps) => CubicBezier::distance(ps, point),
      Segment::EllipticalArc(ps) => EllipticalArc::distance(ps, point),
    }
  }
}

pub trait Primitive {
  /// The type of the collection of `t` values that can be iterated
  type Ts: IntoIterator<Item = f32>;

  /// Sample the primitive to get its value at time `t`
  fn sample(ps: &[Point], t: f32) -> Point;

  /// Return a tangent to the primitve at time `t`
  fn sample_derivative(ps: &[Point], t: f32) -> Vector;

  /// Find when normals of the primitive intersect the given point, where the
  /// times returned fall within the given `range`
  fn find_normals<R: RangeBounds<f32> + Clone>(
    ps: &[Point],
    point: Point,
    range: R,
  ) -> Self::Ts;

  /// Get the pseudo-distance from a point to the primitive at time `t`, where
  /// `t` is contained within the given `range`
  #[inline]
  fn pseudo_distance<R: RangeBounds<f32> + Clone>(
    ps: &[Point],
    point: Point,
    range: R,
  ) -> (/* dist */ f32, /* t */ f32) {
    let mut selected_t = 0.; // initial value doesn't matter
    let mut selected_dist = f32::INFINITY;
    // check perpendiculars
    for t in Self::find_normals(ps, point, range.clone()) {
      let dist = (point - Self::sample(ps, t)).abs();
      if dist < selected_dist {
        selected_dist = dist;
        selected_t = t;
      }
    }
    // check the line extending out from the segments endpoints
    let (start, end) = range_to_values(range);
    if start < 0. {
      let p0 = Self::sample(ps, 0.);
      let p1 = p0 + Self::sample_derivative(ps, 0.);
      let line = [p0, p1];
      if let Some(t) = Line::find_normals(&line, point, start..0f32) {
        let dist = (point - Line::sample(&line, t)).abs();
        if dist < selected_dist {
          selected_dist = dist;
          selected_t = t;
        }
      }
    }
    if end > 1. {
      let p1 = Self::sample(ps, 1.);
      let p0 = p1 - Self::sample_derivative(ps, 1.);
      let line = [p0, p1];
      if let Some(t) = Line::find_normals(&line, point, 1f32..end) {
        let dist = (point - Line::sample(&line, t)).abs();
        if dist < selected_dist {
          selected_dist = dist;
          selected_t = t;
        }
      }
    }
    // check the endpoints
    if start.is_finite() {
      let start_dist = (point - Self::sample(ps, start)).abs();
      if start_dist < selected_dist {
        selected_dist = start_dist;
        selected_t = start;
      }
    }
    if end.is_finite() {
      let end_dist = (point - Self::sample(ps, end)).abs();
      if end_dist < selected_dist {
        selected_dist = end_dist;
        selected_t = end;
      }
    }
    (selected_dist, selected_t)
  }

  /// Get the distance from a point to the primitive at time `t`
  #[inline]
  fn distance(ps: &[Point], point: Point) -> (/* dist */ f32, /* t */ f32) {
    Self::pseudo_distance(ps, point, 0f32..=1f32)
  }
}

/// Helps turn a `RangeBounds<f32>` into a pair of `f32`s.
#[rustfmt::skip]
#[inline]
pub fn range_to_values<R: RangeBounds<f32> + Clone>(
  range: R,
) -> (/* start */ f32, /* end */ f32) {
  use Bound::*;
  match (range.start_bound(), range.end_bound()) {
    (Unbounded, Unbounded) => (f32::NEG_INFINITY, f32::INFINITY),
    (Unbounded, Included(&end))
    | (Unbounded, Excluded(&end)) => (f32::NEG_INFINITY, end),
    (Included(&start), Unbounded)
    | (Excluded(&start), Unbounded) => (start, f32::INFINITY),
    (Included(&start), Included(&end))
    | (Included(&start), Excluded(&end))
    | (Excluded(&start), Excluded(&end))
    | (Excluded(&start), Included(&end)) => (start, end),
  }
}

#[cfg(any(test, doctest))]
mod tests {
  use super::*;

  #[test]
  fn get_segment() {
    use SegmentKind::*;

    let points = vec![
      (0., 0.).into(), // Line
      (1., 1.).into(), // Quad
      (2., 2.).into(),
      (3., 3.).into(), // Cubic
      (4., 4.).into(),
      (5., 5.).into(),
      (6., 6.).into(), // Line
      (7., 7.).into(), // Line
      (0., 0.).into(),
    ];
    let segments = vec![
      SegmentRef {
        kind: Line,
        points_index: 0,
      },
      SegmentRef {
        kind: QuadBezier,
        points_index: 1,
      },
      SegmentRef {
        kind: CubicBezier,
        points_index: 3,
      },
      SegmentRef {
        kind: Line,
        points_index: 6,
      },
      SegmentRef {
        kind: Line,
        points_index: 7,
      },
    ];
    let splines = vec![
      Spline {
        colour: Magenta,
        segments_range: 0..3,
      },
      Spline {
        colour: Yellow,
        segments_range: 3..5,
      },
    ];
    let contours = vec![
      Contour { spline_range: 0..1 },
      Contour { spline_range: 1..2 },
    ];
    let shape = Shape {
      points,
      segments: segments.clone(),
      splines,
      contours,
    };

    {
      let result: Vec<_> = shape
        .segments
        .iter()
        .map(|&segment_ref| shape.get_segment(segment_ref))
        .collect();

      let s1 = [(0., 0.).into(), (1., 1.).into()];
      let s2 = [(1., 1.).into(), (2., 2.).into(), (3., 3.).into()];
      let s3 = [
        (3., 3.).into(),
        (4., 4.).into(),
        (5., 5.).into(),
        (6., 6.).into(),
      ];
      let s4 = [(6., 6.).into(), (7., 7.).into()];
      let s5 = [(7., 7.).into(), (0., 0.).into()];

      let expected = vec![
        Segment::Line(&s1),
        Segment::QuadBezier(&s2),
        Segment::CubicBezier(&s3),
        Segment::Line(&s4),
        Segment::Line(&s5),
      ];

      assert_eq!(result, expected);
    }
  }
}
