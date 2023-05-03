pub mod colour;
pub mod distance;
pub mod sample;
pub mod primitives;

use crate::*;
pub use colour::{Colour, Colour::*};
pub use primitives::*;

/// threshold for float comparisons
const EPSILON: f32 = 0.0001;

/// The kind of a segment & an index into the points buffer where this segment
/// starts
///
/// SegmentKind implicitly gives the length.
pub type SegmentRef = (SegmentKind, /* points index */ usize);

/// The length of a spline (the number of segments it contains) and an index
/// into the segment buffer where the spline starts
pub type SplineRef = (
  /* length */ usize,
  /* segments index */ usize,
  Colour,
);

pub type ContourRef =
  (/* length */ usize, /* splines index */ usize);

/// A reference to a spline in the [`Contour`]
#[derive(Debug, Clone, Copy)]
pub struct Spline<'contour> {
  pub segments: &'contour [SegmentRef],
  pub colour: Colour,
}

impl Spline<'_> {
  /// The number of segments in this spline
  pub fn len(&self) -> usize {
    self.segments.len()
  }
}

/// A Contour is a path describing a closed region of space.
///
/// Sharp corners are assumed to be located at the boundary points of adjacent
/// splines.
#[derive(Debug, Clone, Copy)]
pub struct Contour<'shape> {
  pub splines: &'shape [SplineRef],
}

/// A representation of a shape ready to be decomposed into a raster SDF
#[derive(Debug, Clone)]
pub struct Shape {
  /// A buffer containing the points
  pub points: Vec<Point>,
  /// A buffer containing references to the segments
  pub segments: Vec<SegmentRef>,
  /// A buffer containing references to the splines
  pub splines: Vec<SplineRef>,
  /// A buffer containing the contours
  pub contours: Vec<ContourRef>,
  // TODO: add a flag for fully-smooth. Otherwise there's an ambiguity
  // between teardrop and fully-smooth contours.
}

impl Shape {
  /// Get the contour given a spline index and length
  fn get_contour<'shape>(&'shape self, i: usize) -> Contour<'shape> {
    let (length, index) = self.contours[i];
    Contour {
      splines: &self.splines[index..index + length],
    }
  }

  /// Get the `Spline` at the given index
  #[inline]
  fn get_spline(&self, i: usize) -> Spline {
    let (length, index, colour) = self.splines[i];
    Spline {
      segments: &self.segments[index..index + length],
      colour,
    }
  }

  /// Get an iterator over the [`Spline`]s in a given [`Contour`]
  pub fn splines<'shape>(
    &'shape self,
    contour: Contour<'shape>,
  ) -> impl Iterator<Item = Spline> + 'shape {
    contour
      .splines
      .iter()
      .map(|&spline_ref| self.get_spline(spline_ref))
  }

  /// Get an iterator over the [`Segment`]s in a given [`Spline`]
  #[inline]
  pub fn segments<'shape>(
    &'shape self,
    spline: Spline<'shape>,
  ) -> impl Iterator<Item = Segment> + 'shape {
    spline
      .segments
      .iter()
      .map(|&segment_ref| self.get_segment(segment_ref))
  }

  pub fn contours<'shape>(
    &'shape self,
  ) -> impl Iterator<Item = Contour<'shape>> {
    (0..self.contours.len()).map(|index| self.get_contour(index))
  }
}

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
