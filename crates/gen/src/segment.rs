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

  // TODO: unit test
  pub fn closest_param_t(&self, point: Point) -> f32 {
    match self {
      &Line{start, end} => {
        Vector::from_points(start, point).dot(Vector::from_points(start, end))
        / Vector::from_points(start, end).dot(Vector::from_points(start, end))
      },
      _ => unimplemented!(),
    }
  }

  // TODO: unit test
  pub fn distance_to_point_from_t(&self, point: Point, t: f32) -> f32 {
    match self {
      &Line{start, end} => {
        Vector::from_points(
          start + (t * Vector::from_points(start, end)),
          point
        ).abs()
      },

      &QuadBezier{..} => unimplemented!(),
      &CubicBezier{..} => unimplemented!(),
    }
  }

  pub fn distance_to(&self, point: Point) -> f32 {
    self.distance_to_point_from_t(point, self.closest_param_t(point).clamp(0.0, 1.0))
  }

  // TODO: unit test
  pub fn pseudo_distance_to(&self, point: Point) -> f32 {
    // this is true for Line, maybe not for others
    self.distance_to_point_from_t(point, self.closest_param_t(point))
  }

  pub fn inside_ray_start(&self, ray: Vector, point: Point) -> bool {
    match self {
      &Line{start, ..} => {
        Vector::from_points(start, point).signed_area(ray) >= 0.0
      },
      _ => unimplemented!(),
    }
  }

  pub fn inside_ray_end(&self, ray: Vector, point: Point) -> bool {
    match self {
      &Line{end, ..} => {
        Vector::from_points(end, point).signed_area(ray) <= 0.0
      },
      _ => unimplemented!(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn segment_line_distance_to() {
    let line = Line {
      start: (0.0, 0.0).into(),
      end: (2.0, 4.0).into(),
    };

    assert_eq!(0.0, line.distance_to((0.0, 0.0).into()));
    assert_eq!(0.0, line.distance_to((0.5, 1.0).into()));
    assert_eq!(0.0, line.distance_to((1.0, 2.0).into()));
    assert_eq!(0.0, line.distance_to((2.0, 4.0).into()));
    assert_eq!(5.0f32.sqrt(), line.distance_to((3.0, 1.0).into()));
    assert_eq!(5.0f32.sqrt(), line.distance_to((-1.0, 3.0).into()));
    assert_eq!(5.0f32.sqrt(), line.distance_to((-1.0, -2.0).into()));
    assert_eq!(5.0f32.sqrt(), line.distance_to((3.0, 6.0).into()));
    assert_eq!(1.0, line.distance_to((0.0, -1.0).into()));
    assert_eq!(2.0, line.distance_to((-2.0, 0.0).into()));
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
}
