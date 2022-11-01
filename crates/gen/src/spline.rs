use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Spline<'a> {
  pub segments: &'a [Segment],
  pub channels: Channels,
  pub corner_rays: &'a CornerRays,
}

// TODO
impl Spline<'_> {
  pub fn distance_to(&self, position: Point) -> f32 {
    let mut selected_dist = f32::INFINITY;
    let mut selected_segment = None;
    let mut selected_t = None; // simple optimisation

    for (s, segment) in self.segments.iter().enumerate() {
      let t = segment.closest_param_t(position);
      let dist = segment.distance_to_point_from_t(position, t.clamp(0.0, 1.0));
      debug_assert!(dist >= 0.0, "dist must be an absolute value, but was found to be {dist}");
      if dist < selected_dist {
        selected_dist = dist;
        selected_segment = Some(s);
        selected_t = Some(t);
      }
    }

    if match selected_segment {
      Some(0) => {
        selected_t.unwrap() < 0.0
        && self.segments[selected_segment.unwrap()].inside_ray_start(self.corner_rays.start, position)
      },
      Some(x) if x == self.segments.len() => {
        selected_t.unwrap() > 1.0
        && self.segments[selected_segment.unwrap()].inside_ray_end(self.corner_rays.end, position)
      },
      _ => true,
    } {
      selected_dist
    } else {
      f32::INFINITY
    }
  }

  // TODO: unit test
  pub fn signed_pseudo_distance_to(&self, position: Point) -> f32 {
    let mut selected_dist = f32::INFINITY;
    let mut selected_segment = None;
    let mut selected_t = None;

    for (s, segment) in self.segments.iter().enumerate() {
      let t = segment.closest_param_t(position);
      let dist = segment.distance_to_point_from_t(position, t.clamp(0.0, 1.0));
      debug_assert!(dist >= 0.0, "dist must be an absolute value, but was found to be {dist}");
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
      _ => selected_dist
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
