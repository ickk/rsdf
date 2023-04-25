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
/// Since the area of the parallelogram defined by two normalised vectors is at
/// a maximum (1) when they are orthogonal and a minimum (0) when they are
/// parallel, we use this area as the measure.
#[inline]
fn orthogonality(a: Vector, b: Vector) -> f32 {
  Vector::area(a.norm(), b.norm())
}

impl Spline<'_> {
  /// Get a `Distance` object at the point containing the signed distance, the
  /// signed pseudo distance and a measure of orthogonality.
  pub fn distance_to(&self, point: Point) -> Distance {
    // find the segment with the smallest distance to the point.
    let mut selected_segment = 0;
    let (mut selected_t, mut selected_distance, mut selected_pseudo_dist) = self.segments[0].closest_param_t(point);
    // let mut selected_distance = self.segments[0]
    //   .distance_to_point_at_t(point, selected_t)
    //   .abs();
    for s in 1..self.segments.len() - 1 {
    // for (s, segment) in self.segments.iter().enumerate().skip(1) {
      let segment = &self.segments[s];
      let (t, distance, pseudo_dist) = segment.closest_param_t(point);
      // let distance = segment.distance_to_point_at_t(point, t).abs();
      if t >= 0.0 && t <= 1.0 {
        if distance < selected_distance {
          selected_t = t;
          selected_distance = distance;
          selected_segment = s;
          selected_pseudo_dist = pseudo_dist;
        }
      }
    }
    let (t, distance, pseudo_dist) = self.segments.last().unwrap().closest_param_t(point);
    if t >= 0.0 {
      if distance < selected_distance {
        selected_t = t;
        selected_distance = distance;
        selected_segment = self.segments.len()-1;
        selected_pseudo_dist = pseudo_dist;
      }
    }

    // the measure of orthogonality depends on the end of the spline the point
    // is nearest.
    let orthogonality = if selected_t < 0.0 {
      orthogonality(
        self.segments[selected_segment].vector_start(),
        Vector::from_points(self.segments[selected_segment].start(), point),
      )
    } else if selected_t > 1.0 {
      orthogonality(
        self.segments[selected_segment].vector_end(),
        Vector::from_points(self.segments[selected_segment].end(), point),
      )
    } else {
      1.0
    };

    // let signed_pseudo_distance = self.segments[selected_segment]
    //   .signed_pseudo_distance_to_point_at_t(point, selected_t);

    Distance {
      distance: selected_distance,
      signed_pseudo_distance: selected_pseudo_dist,
      orthogonality,
    }
  }
}
