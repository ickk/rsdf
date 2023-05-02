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

/// A representation of a shape ready to be decomposed into a raster SDF
pub struct Shape {
  pub contours: Vec<Contour>,
}

impl Shape {
  /// Sample the shape at a point
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

    for contour in self.contours.iter() {
      for spline in contour.splines() {
        let dist = contour.spline_distance(spline, point);

        if (spline.colour & Red == Red) && closer(dist, red_dist) {
          red_dist = dist;
          red_spline = Some(spline);
          red_contour = Some(contour);
        }

        if (spline.colour & Green == Green) && closer(dist, green_dist) {
          green_dist = dist;
          green_spline = Some(spline);
          green_contour = Some(contour);
        }

        if (spline.colour & Blue == Blue) && closer(dist, blue_dist) {
          blue_dist = dist;
          blue_spline = Some(spline);
          blue_contour = Some(contour);
        }
      }
    }

    let red_pseudo_dist = red_spline.map_or(f32::NEG_INFINITY, |spline| {
      red_contour.map_or(f32::NEG_INFINITY, |contour| {
        contour.spline_pseudo_distance(spline, point)
      })
    });
    let green_pseudo_dist = green_spline.map_or(f32::NEG_INFINITY, |spline| {
      green_contour.map_or(f32::NEG_INFINITY, |contour| {
        contour.spline_pseudo_distance(spline, point)
      })
    });
    let blue_pseudo_dist = blue_spline.map_or(f32::NEG_INFINITY, |spline| {
      blue_contour.map_or(f32::NEG_INFINITY, |contour| {
        contour.spline_pseudo_distance(spline, point)
      })
    });

    // alternative outputs for testing
    const _SCALE: f32 = 3.;
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
