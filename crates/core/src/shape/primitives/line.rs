use super::*;

/// Line primitive
pub struct Line;

impl Primitive for Line {
  type Ts = Option<f32>;

  #[inline]
  fn sample(ps: &[Point], t: f32) -> Point {
    ps[0] + t * (ps[1] - ps[0])
  }

  #[inline]
  fn sample_derivative(ps: &[Point], _t: f32) -> Vector {
    ps[1] - ps[0]
  }

  #[inline]
  fn pseudo_distance<R: RangeBounds<f32> + Clone>(
    ps: &[Point],
    point: Point,
    range: R,
  ) -> (/* dist */ f32, /* t */ f32) {
    let (start, end) = range_to_values(range);

    // unwrapping is okay because we know find_normals with an unbounded range
    // will always return a value.
    let t = Line::find_normals(ps, point, ..).unwrap().clamp(start, end);
    let dist = (point - Line::sample(ps, t)).abs();
    (dist, t)
  }

  #[inline]
  fn find_normals<R: RangeBounds<f32> + Clone>(
    ps: &[Point],
    point: Point,
    range: R,
  ) -> Option<f32> {
    let v0 = point - ps[0];
    let v1 = ps[1] - ps[0];

    let t = v0.dot(v1) / v1.dot(v1);
    Some(t).filter(|t| range.contains(t))
  }
}

#[cfg(any(test, doctest))]
mod tests {
  use float_cmp::assert_approx_eq;

  #[test]
  fn sample() {
    use super::*;
    {
      let line = [(0., 0.).into(), (4., 0.).into()];
      {
        let t = 0.;
        let result = Line::sample(&line, t);
        let expected = (0., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 0.5;
        let result = Line::sample(&line, t);
        let expected = (2., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 1.;
        let result = Line::sample(&line, t);
        let expected = (4., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = -1.;
        let result = Line::sample(&line, t);
        let expected = (-4., 0.).into();
        assert_eq!(result, expected);
      }
    }
  }

  #[test]
  fn sample_derivative() {
    use super::*;
    {
      let line = [(0., 0.).into(), (4., 0.).into()];
      {
        let t = 0.;
        let result = Line::sample_derivative(&line, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 0.5;
        let result = Line::sample_derivative(&line, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 1.;
        let result = Line::sample_derivative(&line, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
    }
    {
      let line = [(0., 0.).into(), (1., 1.).into()];
      {
        let t = 0.;
        let result = Line::sample_derivative(&line, t).norm();
        let expected = (SQRT_2 / 2., SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 0.5;
        let result = Line::sample_derivative(&line, t).norm();
        let expected = (SQRT_2 / 2., SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 1.;
        let result = Line::sample_derivative(&line, t).norm();
        let expected = (SQRT_2 / 2., SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
    }
  }
}
