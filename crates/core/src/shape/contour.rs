use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum Segment {
  Line,
  QuadBezier,
  CubicBezier,
}

#[derive(Debug, Clone, Copy)]
pub struct Spline {
  len: usize,
  index: usize,
}

// A Contour is a path describing a closed region of space.
//
// Sharp corners are assumed to be located at the boundary points of adjacent
// splines.
pub struct Contour {
  /// A buffer with all of the points in it.
  point_buffer: Vec<Point>,
  /// A buffer containing Segment kind and an index pairs. Where the index
  /// points into the point_buffer to the first of the Segment's points.
  segments: Vec<(Segment, /* `point_buffer` index */ usize)>,
  /// A buffer containing the splines
  splines: Vec<Spline>,
  /// A buffer containing the colours corresponding to the respective Spline.
  ///
  /// Might not be computed.
  edge_colours: Option<Vec<Colour>>,
  // TODO: add a flag for fully-smooth. Otherwise there's an ambiguity
  // between teardrop and fully-smooth contours.
}

impl Contour {
  pub fn spline_distance(&self, spline_index: usize, point: Point) -> f32 {
    todo!()
  }
}
