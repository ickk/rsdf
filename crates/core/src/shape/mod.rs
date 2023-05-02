pub mod colour;
pub mod contour;
pub mod primitives;
pub use colour::{Colour, Colour::*};
pub use contour::*;
pub use primitives::*;

use crate::*;

const COMPARISON_ULPS: i32 = 10000;

pub struct Shape {
  pub contours: Vec<Contour>,
}

impl Shape {
  pub fn sample(&self, point: Point) -> [f32; 3] {
    let mut red_spline = None;
    let mut green_spline = None;
    let mut blue_spline = None;

    let mut red_dist = (f32::INFINITY, -f32::INFINITY);
    let mut green_dist = (f32::INFINITY, -f32::INFINITY);
    let mut blue_dist = (f32::INFINITY, -f32::INFINITY);

    let mut red_pseudo_dist = -f32::INFINITY;
    let mut green_pseudo_dist = -f32::INFINITY;
    let mut blue_pseudo_dist = -f32::INFINITY;

    // for contour in self.contours.iter() {
    if let Some(contour) = self.contours.first() {
      for spline in contour.splines() {
        let dist = contour.spline_distance(spline, point);

        if (spline.colour & Red == Red) && closer(dist, red_dist) {
          red_dist = dist;
          red_spline = Some(spline);
        }

        if (spline.colour & Green == Green) && closer(dist, green_dist) {
          green_dist = dist;
          green_spline = Some(spline);
        }

        if (spline.colour & Blue == Blue) && closer(dist, blue_dist) {
          blue_dist = dist;
          blue_spline = Some(spline);
        }
      }

      red_pseudo_dist = red_spline.map_or(-f32::INFINITY, |spline| {
        contour.spline_pseudo_distance(spline, point)
      });
      green_pseudo_dist = green_spline.map_or(-f32::INFINITY, |spline| {
        contour.spline_pseudo_distance(spline, point)
      });
      blue_pseudo_dist = blue_spline.map_or(-f32::INFINITY, |spline| {
        contour.spline_pseudo_distance(spline, point)
      });
    }

    [
      // red_dist, green_dist,
      // blue_dist,
      // red_orth * 10.0,
      // green_orth * 10.0,
      // blue_orth * 10.0,
      red_pseudo_dist,
      green_pseudo_dist,
      blue_pseudo_dist,
    ]
  }
}

// Comparison function for pairs of distances
fn closer(
  (distance_a, orthogonality_a): (f32, f32),
  (distance_b, orthogonality_b): (f32, f32),
) -> bool {
  distance_a.abs() < distance_b.abs()
    || (orthogonality_a.abs() > orthogonality_b.abs()
      && float_cmp::approx_eq!(
        f32,
        distance_a.abs(),
        distance_b.abs(),
        ulps = COMPARISON_ULPS
      ))
}
