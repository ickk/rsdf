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
#[derive(Debug, Clone, Copy)]
pub enum SegmentKind {
  Line,
  QuadBezier,
  CubicBezier,
}

#[inline]
pub fn range_to_values<R: RangeBounds<f32> + Clone>(
  range: R,
) -> (/* start */ f32, /* end */ f32) {
  use Bound::*;
  match (range.start_bound(), range.end_bound()) {
    (Unbounded, Unbounded) => (-f32::INFINITY, f32::INFINITY),
    (Unbounded, Included(&end)) | (Unbounded, Excluded(&end)) => {
      (-f32::INFINITY, end)
    },
    (Included(&start), Unbounded) | (Excluded(&start), Unbounded) => {
      (start, f32::INFINITY)
    },
    (Included(&start), Included(&end))
    | (Included(&start), Excluded(&end))
    | (Excluded(&start), Excluded(&end))
    | (Excluded(&start), Included(&end)) => (start, end),
  }
}
