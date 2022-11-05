#![allow(unused)]

mod channels;
mod contour;
mod image;
mod math;
mod segment;
mod shape;
mod spline;
pub use channels::*;
pub use contour::*;
pub use image::*;
pub use math::*;
pub use segment::*;
pub use shape::*;
pub use spline::*;

// distanceColor
// TODO: cleanup, unit test
const MAX_DISTANCE: f32 = 10.0;
const MAX_COLOUR: f32 = 256.0;
#[inline]
pub fn distance_color(distance: f32) -> u8 {
  let distance = distance.clamp(-MAX_DISTANCE, MAX_DISTANCE);
  (((distance + MAX_DISTANCE) / (2.0 * MAX_DISTANCE) * MAX_COLOUR) - 1.0) as u8
}
