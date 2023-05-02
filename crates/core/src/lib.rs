#![doc = include_str!("../../../README.md")]

mod image;
mod math;
mod shape;

pub(crate) use math::*;
pub(crate) use shape::*;

pub use image::Image;
pub use math::{Point, Vector};
pub use shape::{
  distance_color, Colour, Colour::*, Contour, SegmentIndex, SegmentKind,
  Shape, SplineIndex,
};
