#![doc = include_str!("../../../README.md")]

mod image;
mod math;
mod shape;

use math::*;
use shape::*;

pub use image::Image;
pub use math::{Point, Vector};
pub use shape::{
  primitives::elliptical_arc, Colour, Colour::*, Contour, SegmentKind,
  SegmentRef, Shape, Spline,
};

pub const MAX_DISTANCE: f32 = 5.;
pub const MAX_COLOUR: f32 = 256.0;

/// Function to convert a distance in the range [0, 1] to an 8-bit integer
/// value centered in the middle of the 8bit range, to be stored in a colour
/// channel in an image.
#[inline]
pub fn distance_color(distance: f32) -> u8 {
  let distance = distance.clamp(-MAX_DISTANCE, MAX_DISTANCE);
  (((distance + MAX_DISTANCE) / (2.0 * MAX_DISTANCE) * MAX_COLOUR) - 1.0) as u8
}

/// A marker to store which end of a segment a point's distance references
///
/// Helps to solve artifacts caused by a spline's pseudo_distance function
/// looping back on itself.
#[derive(Copy, Clone, Debug)]
pub enum Bias {
  Start,
  End,
  Centre,
}
