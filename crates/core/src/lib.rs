mod image;
mod math;
mod shape;

pub(crate) use math::*;
pub(crate) use shape::*;

pub use image::Image;
pub use math::{distance_color, Point, Vector};
pub use shape::{Colour, Colour::*, Contour, SegmentKind, Shape};
