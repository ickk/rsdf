use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Spline<'a> {
  pub segments: &'a [Segment],
  pub channels: Channels,
}

#[derive(Debug)]
pub struct Distance {
  pub distance: f32,
  pub signed_pseudo_distance: f32,
  pub orthogonality: f32,
}

/// A measure of the "orthogonality" of two vectors.
///
/// Since the area of the parallelogram defined by two normalised vectors is at a maximum (1) when
/// they are orthogonal and a minimum (0) when they are parallel, we use this area as the measure.
#[inline]
fn orthogonality(a: Vector, b: Vector) -> f32 {
  Vector::area(a.norm(), b.norm())
}

impl Spline<'_> {
  /// Get a `Distance` object at the point containing the signed distance, the signed pseudo
  /// distance and a measure of orthogonality.
  pub fn distance_to(&self, point: Point) -> Distance {
    // find the segment with the smallest pseudo distance to the point
    let mut selected_segment = 0;
    let mut selected_t = self.segments[0].closest_param_t(point);
    let mut selected_signed_pseudo_distance = self.segments[0].distance_to_point_at_t(point, selected_t);
    for (s, segment) in self.segments.iter().enumerate().skip(1) {
      let t = segment.closest_param_t(point);
      let signed_pseudo_distance = segment.distance_to_point_at_t(point, t);

      if signed_pseudo_distance.abs() < selected_signed_pseudo_distance.abs() {
        selected_t = t;
        selected_signed_pseudo_distance = signed_pseudo_distance;
        selected_segment = s;
      }
    }

    // if the t value lays outside the interval [0, 1] then we need clamp it to get the distance.
    if selected_t < 0.0 {
      Distance {
        distance: self.segments[selected_segment]
          .distance_to_point_at_t(point, 0.0)
          .abs(),
        signed_pseudo_distance: selected_signed_pseudo_distance,
        orthogonality: orthogonality(
          self.segments[selected_segment].extension_start(),
          Vector::from_points(self.segments[selected_segment].start(), point),
        ),
      }
    } else if selected_t > 1.0 {
      Distance {
        distance: self.segments[selected_segment]
          .distance_to_point_at_t(point, 1.0)
          .abs(),
        signed_pseudo_distance: selected_signed_pseudo_distance,
        orthogonality: orthogonality(
          self.segments[selected_segment].extension_end(),
          Vector::from_points(self.segments[selected_segment].end(), point),
        ),
      }
    } else {
      Distance {
        distance: selected_signed_pseudo_distance.abs(),
        signed_pseudo_distance: selected_signed_pseudo_distance,
        orthogonality: 1.0, // we don't care about orthogonality in this case
      }
    }
  }
}
