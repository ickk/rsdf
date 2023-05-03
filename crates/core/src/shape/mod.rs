pub mod colour;
pub mod contour;
pub mod distance;
pub mod primitives;

pub use colour::{Colour, Colour::*};
pub use contour::*;
pub use distance::*;
pub use primitives::*;

use crate::*;

/// threshold for float comparisons
const EPSILON: f32 = 0.0001;

/// The kind of a segment & an index into the points buffer where this segment
/// starts
///
/// SegmentKind implicitly gives the length.
pub type SegmentRef = (SegmentKind, /* points index */ usize);

/// The length of a spline (the number of segments it contains) and an index
/// into the segment buffer where the spline starts
pub type SplineRef =
  (/* length */ usize, /* segments index */ usize);

pub type ContourRef =
  (/* length */ usize, /* splines index */ usize);

/// A representation of a shape ready to be decomposed into a raster SDF
pub struct Shape {
  /// A buffer containing the points
  pub points: Vec<Point>,
  /// A buffer containing references to the segments
  pub segments: Vec<SegmentRef>,
  /// A buffer containing references to the splines
  pub splines: Vec<SplineRef>,
  /// A buffer containing the colours corresponding to the respective Spline.
  pub spline_colours: Vec<Colour>,
  /// A buffer containing the contours
  pub contours: Vec<ContourRef>,
  // TODO: add a flag for fully-smooth. Otherwise there's an ambiguity
  // between teardrop and fully-smooth contours.
}

impl Shape {
  fn get_contour<'shape>(&'shape self, (length, splines_index): ContourRef) -> Contour<'shape> {
    Contour {
      points: &self.points[..],
      segments: &self.segments[..],
      splines: &self.splines[splines_index..splines_index + length],
      spline_colours: &self.spline_colours[..],
    }
  }

  pub fn contours<'shape>(&'shape self) -> impl Iterator<Item = Contour<'shape>> {
    self.contours.iter().map(|&contour_index| self.get_contour(contour_index))
  }

  /// Sample the signed distance of the shape at the given [`Point`]
  pub fn sample_single_channel(&self, point: Point) -> f32 {
    let mut selected_dist = (f32::INFINITY, f32::NEG_INFINITY);
    for contour in self.contours() {
      for spline in contour.splines() {
        let dist = contour.spline_distance_orthogonality(spline, point);
        if closer(dist, selected_dist) {
          selected_dist = dist;
        }
      }
    }
    selected_dist.0
  }

  /// Sample the multi-channel signed pseudo distance of the shape at the given
  /// [`Point`]
  pub fn sample(&self, point: Point) -> [f32; 3] {
    let mut red_spline = None;
    let mut green_spline = None;
    let mut blue_spline = None;

    let mut red_dist = (f32::INFINITY, f32::NEG_INFINITY);
    let mut green_dist = (f32::INFINITY, f32::NEG_INFINITY);
    let mut blue_dist = (f32::INFINITY, f32::NEG_INFINITY);

    let mut red_contour = None;
    let mut green_contour = None;
    let mut blue_contour = None;

    for (contour_i, contour) in self.contours().enumerate() {
      for (spline_i, spline) in contour.splines().enumerate() {
        let dist = contour.spline_distance_orthogonality(spline, point);

        if (spline.colour & Red == Red) && closer(dist, red_dist) {
          red_dist = dist;
          red_spline = Some(spline_i);
          red_contour = Some(contour_i);
        }

        if (spline.colour & Green == Green) && closer(dist, green_dist) {
          green_dist = dist;
          green_spline = Some(spline_i);
          green_contour = Some(contour_i);
        }

        if (spline.colour & Blue == Blue) && closer(dist, blue_dist) {
          blue_dist = dist;
          blue_spline = Some(spline_i);
          blue_contour = Some(contour_i);
        }
      }
    }

    let red_pseudo_dist = red_spline.map_or(f32::NEG_INFINITY, |spline_i| {
      red_contour.map_or(f32::NEG_INFINITY, |contour_i| {
        let contour = self.get_contour(self.contours[contour_i]);
        let spline = contour.get_spline(spline_i);
        self.get_contour(self.contours[contour_i]).spline_pseudo_distance(spline, point)
      })
    });
    let green_pseudo_dist = green_spline.map_or(f32::NEG_INFINITY, |spline_i| {
      green_contour.map_or(f32::NEG_INFINITY, |contour_i| {
        let contour = self.get_contour(self.contours[contour_i]);
        let spline = contour.get_spline(spline_i);
        self.get_contour(self.contours[contour_i]).spline_pseudo_distance(spline, point)
      })
    });
    let blue_pseudo_dist = blue_spline.map_or(f32::NEG_INFINITY, |spline_i| {
      blue_contour.map_or(f32::NEG_INFINITY, |contour_i| {
        let contour = self.get_contour(self.contours[contour_i]);
        let spline = contour.get_spline(spline_i);
        self.get_contour(self.contours[contour_i]).spline_pseudo_distance(spline, point)
      })
    });

    // alternative outputs for testing
    // const _SCALE: f32 = 3.;
    [
      // red_dist.0, green_dist.0, blue_dist.0,
      // red_dist.1 * _SCALE, green_dist.1 * _SCALE, blue_dist.1 * _SCALE,
      red_pseudo_dist,
      green_pseudo_dist,
      blue_pseudo_dist,
    ]
  }
}

/// Comparison function for pairs of distances
fn closer(
  (distance_a, orthogonality_a): (f32, f32),
  (distance_b, orthogonality_b): (f32, f32),
) -> bool {
  distance_b.abs() - distance_a.abs() > EPSILON
    || (orthogonality_a.abs() > orthogonality_b.abs()
      && float_cmp::approx_eq!(
        f32,
        distance_a.abs(),
        distance_b.abs(),
        epsilon = EPSILON
      ))
}
