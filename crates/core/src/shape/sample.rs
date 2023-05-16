use crate::*;
use std::f32::{INFINITY, NEG_INFINITY};

/// Threshold for float comparisons
const EPSILON: f32 = 0.0001;

type Dist = (/* distance */ f32, /* orthogonality */ f32);

impl Shape {
  /// Sample the signed distance of the shape at the given [`Point`]
  pub fn sample_single_channel(&self, point: Point) -> f32 {
    let mut selected_dist: Dist = (INFINITY, NEG_INFINITY);

    for contour in self.contours.iter() {
      for Spline {
        segments_range,
        colour: _,
      } in self.splines[contour.spline_range.clone()].iter()
      {
        let (dist, _) =
          self.spline_distance_orthogonality(segments_range.clone(), point);
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
    let [mut red_spline, mut green_spline, mut blue_spline] =
      [None, None, None];
    let [mut red_dist, mut green_dist, mut blue_dist]: [Dist; 3] =
      [(INFINITY, NEG_INFINITY); 3];

    for Contour { spline_range } in self.contours.iter() {
      for Spline {
        segments_range,
        colour,
      } in self.splines[spline_range.clone()].iter().cloned()
      {
        let (dist, bias) =
          self.spline_distance_orthogonality(segments_range.clone(), point);
        if (colour & Red == Red) && closer(dist, red_dist) {
          red_dist = dist;
          red_spline = Some((segments_range.clone(), bias));
        }
        if (colour & Green == Green) && closer(dist, green_dist) {
          green_dist = dist;
          green_spline = Some((segments_range.clone(), bias));
        }
        if (colour & Blue == Blue) && closer(dist, blue_dist) {
          blue_dist = dist;
          blue_spline = Some((segments_range.clone(), bias));
        }
      }
    }

    [red_spline, green_spline, blue_spline].map(|r| {
      r.map_or(NEG_INFINITY, |(spline, bias)| {
        self.spline_pseudo_distance(spline, point, bias)
      })
    })
  }
}

/// Comparison function for pairs of distances
fn closer(
  (distance_a, orthogonality_a): Dist,
  (distance_b, orthogonality_b): Dist,
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
