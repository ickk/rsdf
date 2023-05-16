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
}
