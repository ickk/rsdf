use super::*;

#[inline]
pub fn sample_line(ps: &[Point], t: f32) -> Point {
  ps[0] + t * (ps[1] - ps[0])
}

// Return a vector pointing in the dirction of the tangent of a line at time t.
#[inline]
pub fn sample_line_direction(ps: &[Point], _t: f32) -> Vector {
  (ps[1] - ps[0]).norm()
}

#[inline]
pub fn find_t_line(ps: &[Point], point: Point) -> f32 {
  let v0 = point - ps[0];
  let v1 = ps[1] - ps[0];

  v0.dot(v1) / v1.dot(v1)
}

#[inline]
pub fn line_distance(
  ps: &[Point],
  point: Point,
) -> (/* dist */ f32, /* t */ f32) {
  let t = find_t_line(ps, point).clamp(0., 1.);
  let dist = (point - sample_line(ps, t)).abs();

  (dist, t)
}

#[inline]
pub fn line_pseudo_distance<R: RangeBounds<f32> + Clone>(
  ps: &[Point],
  point: Point,
  range: R,
) -> (/* dist */ f32, /* t */ f32) {
  let (start, end) = range_to_values(range);

  let t = find_t_line(ps, point).clamp(start, end);
  if !(start..=end).contains(&t) {
    dbg!(t);
  }
  let dist = (point - sample_line(ps, t)).abs();

  (dist, t)
}

#[cfg(any(test, doctest))]
mod tests {
  use float_cmp::assert_approx_eq;

  #[test]
  fn sample_line() {
    use super::*;
    {
      let line = [(0., 0.).into(), (4., 0.).into()];
      {
        let t = 0.;
        let result = sample_line(&line, t);
        let expected = (0., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 0.5;
        let result = sample_line(&line, t);
        let expected = (2., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 1.;
        let result = sample_line(&line, t);
        let expected = (4., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = -1.;
        let result = sample_line(&line, t);
        let expected = (-4., 0.).into();
        assert_eq!(result, expected);
      }
    }
  }

  #[test]
  fn sample_line_direction() {
    use super::*;
    {
      let line = [(0., 0.).into(), (4., 0.).into()];
      {
        let t = 0.;
        let result = sample_line_direction(&line, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 0.5;
        let result = sample_line_direction(&line, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 1.;
        let result = sample_line_direction(&line, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
    }
    {
      let line = [(0., 0.).into(), (1., 1.).into()];
      {
        let t = 0.;
        let result = sample_line_direction(&line, t).norm();
        let expected = (SQRT_2 / 2., SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 0.5;
        let result = sample_line_direction(&line, t).norm();
        let expected = (SQRT_2 / 2., SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 1.;
        let result = sample_line_direction(&line, t).norm();
        let expected = (SQRT_2 / 2., SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
    }
  }
}
