pub mod colour;
pub mod distance;
pub mod primitives;
pub mod sample;

use crate::*;
pub use colour::{Colour, Colour::*};
pub use primitives::{Primitive, Segment, SegmentKind};
use std::ops::Range;

/// Reference to a segment
///
/// SegmentKind implicitly gives the length.
#[derive(Debug, Clone, Copy)]
pub struct SegmentRef {
  pub kind: SegmentKind,
  pub points_index: usize,
}

/// Reference to a spline in the [`Contour`]
#[derive(Debug, Clone)]
pub struct Spline {
  pub segments_range: Range<usize>,
  pub colour: Colour,
}

/// Path describing a closed region of space
///
/// Sharp corners are assumed to be located at the boundary points of adjacent
/// splines.
#[derive(Debug, Clone)]
pub struct Contour {
  pub spline_range: Range<usize>,
}

/// Representation of a shape ready to be decomposed into a raster SDF
#[derive(Debug, Clone)]
pub struct Shape {
  /// Buffer containing the points
  pub points: Vec<Point>,
  /// Buffer containing references to the segments
  pub segments: Vec<SegmentRef>,
  /// Buffer containing references to the splines
  pub splines: Vec<Spline>,
  /// Buffer containing the contours
  pub contours: Vec<Contour>,
  // TODO: add a flag for fully-smooth. Otherwise there's an ambiguity
  // between teardrop and fully-smooth contours.
}

#[test]
fn get_segment() {
  use super::*;
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
