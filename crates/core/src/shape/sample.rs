use crate::*;

impl Shape {
  /// Sample the signed distance of the shape at the given [`Point`]
  pub fn sample_single_channel(&self, point: Point) -> f32 {
    let mut selected_dist = (f32::INFINITY, f32::NEG_INFINITY);
    for contour in self.contours() {
      for spline in self.splines(contour) {
        let dist = self.spline_distance_orthogonality(spline, point);
        if closer(dist, selected_dist) {
          selected_dist = dist;
        }
      }
    }
    selected_dist.0
  }

  /// Sample the multi-channel signed pseudo distance of the shape at the given
  /// [`Point`]
  #[rustfmt::skip]
  pub fn sample(&self, point: Point) -> [f32; 3] {
    let [mut red_spline, mut green_spline, mut blue_spline] = [None; 3];
    let [mut red_dist, mut green_dist, mut blue_dist] =
      [(f32::INFINITY, f32::NEG_INFINITY); 3];

    for contour in self.contours() {
      for spline in self.splines(contour) {
        let dist = self.spline_distance_orthogonality(spline, point);

        if (spline.colour & Red == Red) && closer(dist, red_dist) {
          red_dist = dist;
          red_spline = Some(
            spline,
          );
        }

        if (spline.colour & Green == Green) && closer(dist, green_dist) {
          green_dist = dist;
          green_spline = Some(
            spline,
          );
        }

        if (spline.colour & Blue == Blue) && closer(dist, blue_dist) {
          blue_dist = dist;
          blue_spline = Some(
            spline,
          );
        }
      }
    }

    [red_spline, green_spline, blue_spline].map(|r| {
      r.map_or(
        f32::NEG_INFINITY,
        |spline|
          self.spline_pseudo_distance(spline, point)
      )
    })
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
