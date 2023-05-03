pub mod colour;
pub mod distance;
pub mod sample;
pub mod primitives;

use crate::*;
use std::ops::Range;
pub use colour::{Colour, Colour::*};
pub use primitives::*;

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
  pub spline_range: Range<usize>
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

/*
#[test]
  fn contour_get_spline_segments() {
    use super::*;
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
      (SegmentKind::Line, 0),
      (SegmentKind::QuadBezier, 1),
      (SegmentKind::CubicBezier, 3),
      (SegmentKind::Line, 6),
      (SegmentKind::Line, 7),
      ];
    let splines = vec![(3, 0, Magenta), (2, 3, Yellow)];
    let contours = vec![(1, 0), (1, 1)];
    let shape = Shape {
      points,
      segments,
      splines,
      contours,
    };

    {
      let result: Vec<_> = shape.segments(shape.get_spline((3, 0, Magenta))).collect();

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
      let result: Vec<_> = shape.segments(shape.get_spline(splines[1])).collect();

      let s4 = [(6., 6.).into(), (7., 7.).into()];
      let s5 = [(7., 7.).into(), (0., 0.).into()];

      let expected = vec![Segment::Line(&s4), Segment::Line(&s5)];

      assert_eq!(result, expected);
    }
  }
*/
