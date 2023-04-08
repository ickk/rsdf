use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Segment {
  Line {
    start: Point,
    end: Point,
  },
  QuadBezier {
    start: Point,
    control: Point,
    end: Point,
  },
  CubicBezier {
    start: Point,
    control_1: Point,
    control_2: Point,
    end: Point,
  },
}

use Segment::*;

impl Segment {
  /// Get the parameter `t` of the Segment when it is closest to the Point.
  ///
  /// Note this param may be in the range (-inf, inf). i.e. it's not restricted by the end-points.
  #[inline]
  pub fn closest_param_t(&self, point: Point) -> f32 {
    match *self {
      Line { start, end } => {
        // We can find the closest point on the line by projecting the point onto the line.
        let s_p = Vector::from_points(start, point);
        let s_e = Vector::from_points(start, end);
        let p_onto_line = s_p.dot(s_e.norm());
        // Then we normalise it with respect to the length of the line.
        p_onto_line / s_e.abs()
      },
      QuadBezier {
        start,
        control,
        end,
      } => {
        // some of this can be cached as it doesn't rely on the `point`.
        // vectors
        let p0 = Vector::from_points(start, point);
        let p1 = Vector::from_points(start, control);
        let p2 =
          end.as_vector() - 2. * control.as_vector() + start.as_vector();
        // The roots of the cubic equation yield `t` when the vector from `t`
        // to `point` is perpendicular to the quadratic bezier.
        assert!(p2.abs() > 0.0001, "quadratic bezier is degenerate");
        // the control falls directly between start & end. i.e. a line.

        // a*t^3 + b*t^2 + c*t + d = 0
        let a = p2.dot(p2);
        let b = 3. * p1.dot(p2);
        let c = 2. * p1.dot(p1) - p2.dot(p0);
        let d = -1. * p1.dot(p0);

        // these need to be clamped, and we need to take the extensions into account
        let roots = cubic::roots(a, b, c, d);
        match roots {
          cubic::Roots::One(t0) => t0,
          // t0.clamp(0.0, 1.0),
          cubic::Roots::Two(t0, t1) => {
            // let (t0, t1) = (t0.clamp(0.0, 1.0), t1.clamp(0.0, 1.0));
            let d0 = if t0 < 0.0 {
              let line = Line { start, end: control };
              let t0 = line.closest_param_t(point);
              Self::distance_to_point_at_t(&line, point, t0)
            } else if t0 > 1.0 {
              let line = Line { start: control, end };
              let t0 = line.closest_param_t(point);
              Self::distance_to_point_at_t(&line, point, t0)
            } else {
              self.distance_to_point_at_t(point, t0)
            };
            let d1 = if t1 < 0.0 {
              let line = Line { start, end: control };
              let t1 = line.closest_param_t(point);
              Self::distance_to_point_at_t(&line, point, t1)
            } else if t1 > 1.0 {
              let line = Line { start: control, end };
              let t1 = line.closest_param_t(point);
              Self::distance_to_point_at_t(&line, point, t1)
            } else {
              self.distance_to_point_at_t(point, t1)
            };
            if d0 <= d1 {
              t0
            } else {
              t1
            }
          },
          cubic::Roots::Three(t0, t1, t2) => {
            let (mut t0, mut t1, mut t2) = (t0, t1, t2);
            // let (t0, t1, t2) = (t0.clamp(0.0, 1.0), t1.clamp(0.0, 1.0), t2.clamp(0.0, 1.0));
            // let d0 = self.distance_to_point_at_t(point, t0);
            // let d1 = self.distance_to_point_at_t(point, t1);
            // let d2 = self.distance_to_point_at_t(point, t2);
            if t0 < 0.0 {
              let line = Line { start, end: control };
              t0 = line.closest_param_t(point);
            } else if t0 > 1.0 {
              let line = Line { start: control, end };
              t0 = line.closest_param_t(point);
            }
            if t1 < 0.0 {
              let line = Line { start, end: control };
              t1 = line.closest_param_t(point);
            } else if t1 > 1.0 {
              let line = Line { start: control, end };
              t1 = line.closest_param_t(point);
            }
            if t2 < 0.0 {
              let line = Line { start, end: control };
              t2 = line.closest_param_t(point);
            } else if t1 > 1.0 {
              let line = Line { start: control, end };
              t2 = line.closest_param_t(point);
            }

            let d0 = self.distance_to_point_at_t(point, t0);
            let d1 = self.distance_to_point_at_t(point, t1);
            let d2 = self.distance_to_point_at_t(point, t2);

            if d0 <= d1 && d0 <= d2 {
              t0
            } else if d1 <= d0 && d1 <= d2 {
              t1
            } else {
              t2
            }
          },
        }
      },
      _ => unimplemented!(),
    }
  }

  /// Distance between some point `P` and the segment at time `t`. `t` is clamped to [0, 1];
  #[inline]
  pub fn distance_to_point_at_t(&self, point: Point, t: f32) -> f32 {
    let t = t.clamp(0., 1.);

    match *self {
      Line { start, end } => {
        let b = start + (t * Vector::from_points(start, end));
        Vector::from_points(b, point).abs()
      },
      QuadBezier {
        start,
        control,
        end,
      } => {
        let b = start
          + 2. * t * Vector::from_points(start, control)
          + t
            * t
            * (Vector::from_points(end, control)
              + Vector::from_points(control, start));
        Vector::from_points(b, point).abs()
      },
      _ => unimplemented!(),
    }
  }

  #[inline]
  pub fn signed_pseudo_distance_to_point_at_t(
    &self,
    point: Point,
    t: f32,
  ) -> f32 {
    match *self {
      Line { start, end } => {
        let signed_area = {
          let a = Vector::from_points(start, end);
          let b = Vector::from_points(start, point);
          a.signed_area(b)
        };

        Vector::from_points(
          start + (t * Vector::from_points(start, end)),
          point,
        )
        .abs()
        .copysign(signed_area)
      },
      QuadBezier {
        start,
        control,
        end,
      } => {
        let signed_area = {
          let a = Vector::from_points(start, end);
          let b = Vector::from_points(start, point);
          a.signed_area(b)
        };

        if t <= 0. {
          let extension = Segment::Line {
            start: self.start(),
            end: self.start() + self.vector_start(),
          };
          extension
            .distance_to_point_at_t(point, extension.closest_param_t(point))
            .copysign(signed_area)
        } else if t >= 1. {
          let extension = Segment::Line {
            start: self.end(),
            end: self.end() + self.vector_end(),
          };
          extension
            .distance_to_point_at_t(point, extension.closest_param_t(point))
            .copysign(signed_area)
        } else {
          let b = start
            + 2. * t * Vector::from_points(start, control)
            + t
              * t
              * (Vector::from_points(end, control)
                + Vector::from_points(control, start));
          Vector::from_points(b, point).abs().copysign(signed_area)
        }
      },
      _ => unimplemented!(),
    }
  }

  /// The vector continuing in the direction of the start of the segment.
  #[inline]
  pub fn vector_start(&self) -> Vector {
    match *self {
      Line { start, end } => Vector::from_points(start, end),
      QuadBezier { start, control, .. } => Vector::from_points(start, control),
      CubicBezier {
        start, control_1, ..
      } => Vector::from_points(start, control_1),
    }
  }

  /// The vector continuing in the direction of the end of the segment.
  #[inline]
  pub fn vector_end(&self) -> Vector {
    match *self {
      Line { start, end } => Vector::from_points(start, end),
      QuadBezier { control, end, .. } => Vector::from_points(control, end),
      CubicBezier { control_2, end, .. } => {
        Vector::from_points(control_2, end)
      },
    }
  }

  #[inline]
  pub fn start(&self) -> Point {
    match *self {
      Line { start, .. }
      | QuadBezier { start, .. }
      | CubicBezier { start, .. } => start,
    }
  }

  #[inline]
  pub fn end(&self) -> Point {
    match *self {
      Line { end, .. } | QuadBezier { end, .. } | CubicBezier { end, .. } => {
        end
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use float_cmp::approx_eq;

  #[test]
  fn segment_line_closest_param_t() {
    let line = Line {
      start: (0.0, 0.0).into(),
      end: (10.0, 0.0).into(),
    };

    {
      let point = Point::from((0.0, 0.0));
      let t = line.closest_param_t(point);
      let expected = 0.0;
      assert!(approx_eq!(f32, t, expected, ulps = 2));
    }
    {
      let point = Point::from((10.0, 0.0));
      let t = line.closest_param_t(point);
      let expected = 1.0;
      assert!(approx_eq!(f32, t, expected, ulps = 2));
    }
    {
      let point = Point::from((5.0, 0.0));
      let t = line.closest_param_t(point);
      let expected = 0.5;
      assert!(approx_eq!(f32, t, expected, ulps = 2));
    }
    {
      let point = Point::from((3.0, 4.0));
      let t = line.closest_param_t(point);
      let expected = 0.3;
      assert!(approx_eq!(f32, t, expected, ulps = 2));
    }
    {
      let point = Point::from((11.0, 4.0));
      let t = line.closest_param_t(point);
      let expected = 1.1;
      assert!(approx_eq!(f32, t, expected, ulps = 2));
    }
    {
      let point = Point::from((-2.3, 4.0));
      let t = line.closest_param_t(point);
      let expected = -0.23;
      assert!(approx_eq!(f32, t, expected, ulps = 2));
    }
  }

  #[test]
  fn segment_line_distance_to_point_at_t() {
    let line = Line {
      start: (0.0, 0.0).into(),
      end: (10.0, 0.0).into(),
    };

    {
      let point = Point::from((0.0, 1.0));
      let dist = line.distance_to_point_at_t(point, 0.0);
      let expected = 1.0;
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let point = Point::from((0.0, 3.0));
      let dist = line.distance_to_point_at_t(point, 0.4);
      let expected = 5.0;
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
  }
}
