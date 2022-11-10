use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Spline<'a> {
  pub segments: &'a [Segment],
  pub channels: Channels,
  pub corner_rays: &'a CornerRays,
}

#[derive(Debug)]
pub struct Distance {
  pub dist: f32,
  pub sdist: f32,
  pub orthogonality: f32,
  pub t: f32,
  pub segment: usize,
}

/// A measure of the "orthogonality" of two vectors.
///
/// Since the area of the parallelogram defined by two normalised vectors is at a maximum (1) when
/// they are orthogonal and a minimum (0) when they are parallel, we use this area as the measure.
fn orthogonality(vec: Vector, end_point: Point, point: Point) -> f32 {
  Vector::area(
    vec.norm(),
    Vector::from_points(end_point, point).norm(),
  )
}
// the fact that this is an abs area rather than a signed area (based on whether it's at the start
// or end) could be the cause of what the paper refers to as "false edges"?.

// TODO
// minimise the dist of each segment (yes we do have to check them all in a single spline)
// if, for the winning t, 0 > t  or t > 1 then we need the orthogonality as well. Assuming the
// spline is well-formed this should only be the case for the first or last segment.
// When compositing the spline distances, we need to maximise the orthogonality.
impl Spline<'_> {
  pub fn distance_to(&self, position: Point) -> Distance {
    struct Selected {
      dist: f32,
      t: f32,
      s: usize,
    }

    let mut selected = {
      let s = 0;
      let t = self.segments[0].closest_param_t(position);
      let dist = self.segments[0].distance_to_point_at_t(position, t);

      Selected {s, t, dist}
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
      sdist = selected.dist.copysign(segment.signed_pseudo_distance_at_t(position, selected.t) as f32);
      selected.dist = segment.distance_to_point_at_t(position, 0.0);
      orth = orthogonality(segment.extension_start(), segment.start(), position);
    } else if selected.t > 1.0 {
      sdist = selected.dist.copysign(segment.signed_pseudo_distance_at_t(position, selected.t) as f32);
      selected.dist = segment.distance_to_point_at_t(position, 1.0);
      orth = orthogonality(segment.extension_end(), segment.end(), position);
    } else {
      sdist = selected.dist.copysign(segment.signed_pseudo_distance_at_t(position, selected.t) as f32);
      orth = 1.0 // this is a largely bogus value but it shouldn't be needed in this case
    };

    let d = Distance {
      dist: selected.dist,
      sdist,
      orthogonality: orth,
      t: selected.t,
      segment: selected.s,
    };
    // eprintln!("distance: {d:?}");
    d
    // // last segment
    // {
    //   let s = self.segments.len() - 1;
    //   let (s, segment) = self.segments[s]
    // }
    // // mid segments
    // let mut segments = self.segments.iter().enumerate();

    // for (s, segment) in self.segments.iter().enumerate() {
    //   // first segment
    //   if s == 0 {

    //   }


    //   let t = segment.closest_param_t(position);
    //   match t {
    //     0.0..=1.0 => {
    //       t_selected = t;
    //       s_selected = s;
    //       break // if the t falls between 0-1 we know
    //     },
    //     t if s == 0 => if t > t_selected {
    //       t_selected = t;
    //       s_selected = s;
    //     },
    //     t if s == self.segments.len() - 1 => if t < t_selected {
    //       t_selected = t;
    //       s_selected = s;
    //     },
    //     _ => {},
    //   }
    // }

    // let mut selected_dist = f32::INFINITY;
    // let mut selected_segment = None;
    // let mut selected_t = None; // simple optimisation

    // for (s, segment) in self.segments.iter().enumerate() {
    //   let t = segment.closest_param_t(position);
    //   let dist = segment.distance_to_point_at_t(position, t.clamp(0.0, 1.0));
    //   debug_assert!(
    //     dist >= 0.0,
    //     "dist must be an absolute value, but was found to be {dist}"
    //   );
    //   eprintln!("! dist: {:?}", dist);
    //   if dist < selected_dist {
    //     selected_dist = dist;
    //     selected_segment = Some(s);
    //     selected_t = Some(t);
    //   }
    // }

    // if self.segments.len() == 1 {
    //   if (selected_t.unwrap() < 0.0
    //     && !self.segments[selected_segment.unwrap()].inside_ray_start(self.corner_rays.start, position))
    //     || (selected_t.unwrap() > 1.0 && !self.segments[selected_segment.unwrap()].inside_ray_end(self.corner_rays.end, position)) {
    //     f32::INFINITY
    //   } else {
    //     0.0
    //   }
    // } else {
    //   unimplemented!()
    // }

    // eprintln!("!t: {:?}", selected_t);
    // if match selected_segment {
    //   Some(0) => {
    //     eprintln!("start: t<0: {:?} inside_ray_start: {:?}", selected_t.unwrap() < 0.0, self.segments[selected_segment.unwrap()]
    //         .inside_ray_start(self.corner_rays.start, position));

    //     selected_t.unwrap() < 0.0
    //       && self.segments[selected_segment.unwrap()]
    //         .inside_ray_start(self.corner_rays.start, position)
    //   },
    //   Some(x) if x == self.segments.len() => {
    //     selected_t.unwrap() > 1.0
    //       && self.segments[selected_segment.unwrap()].inside_ray_end(self.corner_rays.end, position)
    //   },
    //   _ => true,
    // } {
    //   eprintln!("!! Selected_dist: {:?}", selected_dist);
    //   selected_dist
    // } else {
    //   f32::INFINITY
    // }
  }

  // TODO: unit test
  pub fn signed_pseudo_distance_to(&self, position: Point) -> f32 {
    let mut selected_dist = f32::INFINITY;
    let mut selected_segment = None;
    let mut selected_t = None;

    for (s, segment) in self.segments.iter().enumerate() {
      let t = segment.closest_param_t(position);
      let dist = segment.distance_to_point_at_t(position, t.clamp(0.0, 1.0));
      debug_assert!(
        dist >= 0.0,
        "dist must be an absolute value, but was found to be {dist}"
      );
      if dist < selected_dist {
        selected_dist = dist;
        selected_segment = Some(s);
        selected_t = Some(t);
      }
    }

    match selected_segment {
      Some(x) if x == self.segments.len() || x == 0 => {
        self.segments[selected_segment.unwrap()].pseudo_distance_to(position)
      },
      _ => selected_dist,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use Segment::*;

  #[test]
  fn spline_distance_to() {
    let line = Line {
      start: (0.0, 0.0).into(),
      end: (10.0, 0.0).into(),
    };

    let ray = Vector::from((2.0, 1.0)).norm();
    assert!(line.inside_ray_end(ray, (10.0, 1.0).into()));
    assert!(line.inside_ray_end(ray, (12.0, 1.0).into()));
    assert!(!line.inside_ray_end(ray, (12.01, 1.0).into()));
    assert!(!line.inside_ray_end(ray, (12.00, 0.0).into()));
  }
}
