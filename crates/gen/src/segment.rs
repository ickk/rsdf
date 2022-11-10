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
  // TODO: impl quad & cubic, unit test
  // TODO: break `t` into own method. store t and if first or last segment in
  // spline, call appropriate ray methods.

  /// Get the parameter `t` of the Segment when it is closest to the Point.
  ///
  /// Note this param may be in the range (-inf, inf). i.e. it's not restricted by the end-points.
  pub fn closest_param_t(&self, point: Point) -> f32 {
    match self {
      &Line { start, end } => {
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
  // todo: use the extensions for t < 0 and t > 1. (the pseudo distance)
  // then distances will be returned for `t.clamp(0, 1)` and pseudo-distances for `t`.
  pub fn distance_to_point_at_t(&self, point: Point, t: f32) -> f32 {
    match self {
      &Line { start, end } => {
        // Get the length of the vector from the line at `t` and the point.
        // eprintln!("  line: {:?}", self);
        // eprintln!("  point: {:?}", point);
        let r = Vector::from_points(start + (t * Vector::from_points(start, end)), point).abs();
        // eprintln!("  distance_to: {:?}", r);
        r
      },
      _ => unimplemented!(),
    }
  }

  pub fn signed_pseudo_distance_at_t(&self, point: Point, t: f32) -> i8 {
    match self {
      &Line { start, end } => {
        let a = Vector::from_points(start, end);
        let b = Vector::from_points(start, point);
        1f32.copysign(a.signed_area(b)) as i8
      },
      _ => unimplemented!()
    }
  }

  /// The distance from a Point to the Segment.
  // todo: remove
  pub fn distance_to(&self, point: Point) -> f32 {
    self.distance_to_point_at_t(point, self.closest_param_t(point).clamp(0.0, 1.0))
  }

  // todo: remove
  pub fn pseudo_distance_to(&self, point: Point) -> f32 {
    match self {
      // this is true for Line, maybe not for others
      &Line { .. } => self.distance_to_point_at_t(point, self.closest_param_t(point)),
      _ => unimplemented!(),
    }
  }

  /// The vector continuing in the direction of the start of the segment.
  pub fn extension_start(&self) -> Vector {
    match self {
      &Line { start, end } => Vector::from_points(start, end),
      &QuadBezier { start, control, .. } => Vector::from_points(start, control),
      &CubicBezier { start, control_1, .. } => Vector::from_points(start, control_1),
    }
  }

  /// The vector continuing in the direction of the end of the segment.
  pub fn extension_end(&self) -> Vector {
    match self {
      &Line { start, end } => Vector::from_points(start, end),
      &QuadBezier { control, end, .. } => Vector::from_points(control, end),
      &CubicBezier { control_2, end, .. } => Vector::from_points(control_2, end),
    }
  }

  pub fn start(&self) -> Point {
    match self {
      &Line{ start, .. } | &QuadBezier{ start, .. } | &CubicBezier{ start, .. } => start
    }
  }

  pub fn end(&self) -> Point {
    match self {
      &Line{ end, .. } | &QuadBezier{ end, .. } | &CubicBezier{ end, .. } => end
    }
  }

  // todo: remove
  pub fn inside_ray_start(&self, ray: Vector, point: Point) -> bool {
    match self {
      &Line { start, .. } => Vector::from_points(start, point).signed_area(ray) <= 0.0,
      _ => unimplemented!(),
    }
  }

  // todo: remove
  pub fn inside_ray_end(&self, ray: Vector, point: Point) -> bool {
    match self {
      &Line { end, .. } => Vector::from_points(end, point).signed_area(ray) >= 0.0,
      _ => unimplemented!(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use float_cmp::approx_eq;

  #[test]
  fn segment_line_distance_to() {
    let line = Line {
      start: (0.0, 0.0).into(),
      end: (2.0, 4.0).into(),
    };

    {
      let dist = line.distance_to((0.0, 0.0).into());
      let expected = 0.0;
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let dist = line.distance_to((0.5, 1.0).into());
      let expected = 0.0;
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let dist = line.distance_to((1.0, 2.0).into());
      let expected = 0.0;
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let dist = line.distance_to((2.0, 4.0).into());
      let expected = 0.0;
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let dist = line.distance_to((3.0, 1.0).into());
      let expected = 5.0f32.sqrt();
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let dist = line.distance_to((-1.0, 3.0).into());
      let expected = 5.0f32.sqrt();
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let dist = line.distance_to((-1.0, -2.0).into());
      let expected = 5.0f32.sqrt();
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let dist = line.distance_to((3.0, 6.0).into());
      let expected = 5.0f32.sqrt();
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let dist = line.distance_to((0.0, -1.0).into());
      let expected = 1.0;
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let dist = line.distance_to((-2.0, 0.0).into());
      let expected = 2.0;
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
  }

  #[test]
  fn segment_line_inside_ray_start() {
    let line = Line {
      start: (0.0, 0.0).into(),
      end: (10.0, 0.0).into(),
    };

    let ray = Vector::from((-1.0, 1.0)).norm();
    assert!(line.inside_ray_start(ray, (0.0, 1.0).into()));
    assert!(line.inside_ray_start(ray, (-1.0, 1.0).into()));
    assert!(!line.inside_ray_start(ray, (-1.0, 0.0).into()));
    assert!(!line.inside_ray_start(ray, (-1.01, 1.0).into()));
  }

  #[test]
  fn segment_line_inside_ray_end() {
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

  #[test]
  fn segment_line_pseudo_distance_to() {
    let line = Line {
      start: (0.0, 0.0).into(),
      end: (10.0, 0.0).into(),
    };

    {
      let point = Point::from((0.0, 1.0));
      let dist = line.pseudo_distance_to(point);
      let expected = 1.0;
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let point = Point::from((0.0, 3.0));
      let dist = line.pseudo_distance_to(point);
      let expected = 3.0;
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let point = Point::from((-3.0, 4.0));
      let dist = line.pseudo_distance_to(point);
      let expected = 4.0;
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
    {
      let point = Point::from((18.0, 5.0));
      let dist = line.pseudo_distance_to(point);
      let expected = 5.0;
      assert!(approx_eq!(f32, dist, expected, ulps = 2));
    }
  }
}
