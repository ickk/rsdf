pub mod cubic_bezier;
pub mod line;
pub mod quad_bezier;

pub use cubic_bezier::*;
pub use line::*;
pub use quad_bezier::*;

use crate::*;
use arrayvec::ArrayVec;
use std::ops::{Bound, RangeBounds};

/// The kind of a segment
///
/// A `Segment` is constructed from a [`SegmentIndex`], where the `SegmentIndex`
/// contains ([`SegmentKind`], `&[`[`Point`]`]`).
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
}

impl Shape {
  /// Get a segment given a `SegmentRef`
  #[inline]
  pub fn get_segment(&self, (kind, i): SegmentRef) -> Segment {
    match kind {
      SegmentKind::Line => Segment::Line(&self.points[i..i + 2]),
      SegmentKind::QuadBezier => Segment::QuadBezier(&self.points[i..i + 3]),
      SegmentKind::CubicBezier => Segment::CubicBezier(&self.points[i..i + 4]),
    }
  }
}

/// A reference to a segment in the Contour
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Segment<'contour> {
  Line(&'contour [Point]),
  QuadBezier(&'contour [Point]),
  CubicBezier(&'contour [Point]),
}

impl Segment<'_> {
  /// Sample the segment to get its value at time `t`
  #[inline]
  pub fn sample(self, t: f32) -> Point {
    match self {
      Segment::Line(ps) => Line::sample(ps, t),
      Segment::QuadBezier(ps) => QuadBezier::sample(ps, t),
      Segment::CubicBezier(ps) => CubicBezier::sample(ps, t),
    }
  }

  /// Return a tangent to the segment at time `t`
  #[inline]
  pub fn sample_derivative(self, t: f32) -> Vector {
    match self {
      Segment::Line(ps) => Line::sample_derivative(ps, t),
      Segment::QuadBezier(ps) => QuadBezier::sample_derivative(ps, t),
      Segment::CubicBezier(ps) => CubicBezier::sample_derivative(ps, t),
    }
  }

  /// Get the pseudo-distance from a point to the primitive at time `t`, where
  /// `t` is contained within the given `range`
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
    }
  }

  /// Get the distance from a point to the segment at time `t`
  #[inline]
  pub fn distance(self, point: Point) -> (/* dist */ f32, /* t */ f32) {
    match self {
      Segment::Line(ps) => Line::distance(ps, point),
      Segment::QuadBezier(ps) => QuadBezier::distance(ps, point),
      Segment::CubicBezier(ps) => CubicBezier::distance(ps, point),
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

  /// Get the pseudo-distance from a point to the primitive at time `t`, where
  /// `t` is contained within the given `range`
  fn pseudo_distance<R: RangeBounds<f32> + Clone>(
    ps: &[Point],
    point: Point,
    range: R,
  ) -> (/* dist */ f32, /* t */ f32);

  /// Find when normals of the primitive intersect the given point, where the
  /// times returned fall within the given `range`
  fn find_normals<R: RangeBounds<f32> + Clone>(
    ps: &[Point],
    point: Point,
    range: R,
  ) -> Self::Ts;

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
