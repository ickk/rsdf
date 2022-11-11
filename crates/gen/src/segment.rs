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
        let s_e_norm = Vector::from_points(start, end).norm();
        let p_onto_line = s_p.dot(s_e_norm);
        // Then we normalise it with respect to the length of the line.
        let line_length = Vector::from_points(start, end).abs();
        p_onto_line / line_length
      },
      _ => unimplemented!(),
    }
  }

  /// Distance between some point `P` and the segment at time `t`.
  #[inline]
  pub fn distance_to_point_at_t(&self, point: Point, t: f32) -> f32 {
    match *self {
      Line { start, end } => {
        let sign = self.sign_at_point(point);

        Vector::from_points(start + (t * Vector::from_points(start, end)), point).abs().copysign(sign)
      },
      _ => unimplemented!(),
    }
  }

  #[inline]
  pub fn sign_at_point(&self, point: Point) -> f32 {
    match *self {
      Line { start, end } => {
        let a = Vector::from_points(start, end);
        let b = Vector::from_points(start, point);
        1f32.copysign(a.signed_area(b))
      },
      _ => unimplemented!(),
    }
  }

  /// The vector continuing in the direction of the start of the segment.
  #[inline]
  pub fn extension_start(&self) -> Vector {
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
  pub fn extension_end(&self) -> Vector {
    match *self {
      Line { start, end } => Vector::from_points(start, end),
      QuadBezier { control, end, .. } => Vector::from_points(control, end),
      CubicBezier { control_2, end, .. } => Vector::from_points(control_2, end),
    }
  }

  #[inline]
  pub fn start(&self) -> Point {
    match *self {
      Line { start, .. } | QuadBezier { start, .. } | CubicBezier { start, .. } => start,
    }
  }

  #[inline]
  pub fn end(&self) -> Point {
    match *self {
      Line { end, .. } | QuadBezier { end, .. } | CubicBezier { end, .. } => end,
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
