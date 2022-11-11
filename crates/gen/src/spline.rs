use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Spline<'a> {
  pub segments: &'a [Segment],
  pub channels: Channels,
}

#[derive(Debug)]
pub struct Distance {
  pub dist: f32,
  pub sdist: f32,
  pub orthogonality: f32,
}

/// A measure of the "orthogonality" of two vectors.
///
/// Since the area of the parallelogram defined by two normalised vectors is at a maximum (1) when
/// they are orthogonal and a minimum (0) when they are parallel, we use this area as the measure.
fn orthogonality(a: Vector, b: Vector) -> f32 {
  Vector::area(a.norm(), b.norm())
}

struct Selected {
  dist: f32,
  t: f32,
  s: usize,
}

impl Spline<'_> {
  pub fn distance_to(&self, position: Point) -> Distance {
    let mut selected = {
      let s = 0;
      let t = self.segments[0].closest_param_t(position);
      let dist = self.segments[0].distance_to_point_at_t(position, t);

      Selected { s, t, dist }
    };

    for (s, segment) in self.segments.iter().enumerate().skip(1) {
      let t = segment.closest_param_t(position);
      let dist = segment.distance_to_point_at_t(position, t);

      if dist < selected.dist {
        selected.t = t;
        selected.dist = dist;
        selected.s = s;
      }
    }

    let segment = &self.segments[selected.s];
    let orth: f32;
    let sdist: f32;
    if selected.t < 0.0 {
      sdist = selected.dist.copysign(segment.sign_at_point(position));
      selected.dist = segment.distance_to_point_at_t(position, 0.0);
      orth = orthogonality(
        segment.extension_start(),
        Vector::from_points(segment.start(), position),
      );
    } else if selected.t > 1.0 {
      sdist = selected.dist.copysign(segment.sign_at_point(position));
      selected.dist = segment.distance_to_point_at_t(position, 1.0);
      orth = orthogonality(
        segment.extension_end(),
        Vector::from_points(segment.end(), position),
      );
    } else {
      sdist = selected.dist.copysign(segment.sign_at_point(position));
      orth = 1.0 // this is a largely bogus value but it shouldn't be needed in this case
    };

    Distance {
      dist: selected.dist,
      sdist,
      orthogonality: orth,
    }
  }
}
