use super::*;

#[inline]
#[rustfmt::skip]
pub fn sample_quad_bezier(ps: &[Point], t: f32) -> Point {
  ps[0]
    + 2f32*t
      * (ps[1] - ps[0])
    + t*t
      * (ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector())
}

// Return a vector pointing in the dirction of the tangent of a quadratic
// bezier at time t.
#[inline]
#[rustfmt::skip]
pub fn sample_quad_bezier_direction(ps: &[Point], t: f32) -> Vector {
  (
    2f32*(ps[1] - ps[0])
      + 2f32*t
        * (ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector())
  ).norm()
}

pub fn find_ts_quad_bezier<R: RangeBounds<f32>>(
  ps: &[Point],
  point: Point,
  range: R,
) -> ArrayVec<f32, 4> {
  let v2 = ps[2].as_vector() - 2f32 * ps[1].as_vector() + ps[0].as_vector();
  // check if the curve degenerates into a line
  if v2 == Vector::ZERO {
    let line = [ps[0], ps[1]];
    let mut a = ArrayVec::new();
    a.push(find_t_line(&line, point));
    return a;
  }
  let v0 = point - ps[0];
  let v1 = ps[1] - ps[0];

  let polynomial = [
    -v1.dot(v0),
    2f32 * v1.dot(v1) - v2.dot(v0),
    3f32 * v1.dot(v2),
    v2.dot(v2),
  ];

  roots_in_range(&polynomial, range)
}

#[inline]
pub fn quad_bezier_distance(
  ps: &[Point],
  point: Point,
) -> (/* dist */ f32, /* t */ f32) {
  let mut selected_t;
  let mut selected_dist;
  // check endpoints
  {
    // start
    selected_t = 0.0;
    selected_dist = (point - sample_quad_bezier(ps, selected_t)).abs();
    // end
    let end_dist = (point - sample_quad_bezier(ps, 1.0)).abs();
    if end_dist < selected_dist {
      selected_dist = end_dist;
      selected_t = 1.0;
    }
  }
  // check perpendiculars
  for t in find_ts_quad_bezier(ps, point, 0f32..=1f32) {
    let dist = (point - sample_quad_bezier(ps, t)).abs();
    if dist < selected_dist {
      selected_dist = dist;
      selected_t = t;
    }
  }

  (selected_dist, selected_t)
}

#[inline]
pub fn quad_bezier_pseudo_distance<R: RangeBounds<f32> + Clone>(
  ps: &[Point],
  point: Point,
  range: R,
) -> (/* dist */ f32, /* t */ f32) {
  let mut selected_t = 0.; // initial value doesn't matter
  let mut selected_dist = f32::INFINITY;

  // check perpendiculars
  for t in find_ts_quad_bezier(ps, point, range.clone()) {
    let dist = (point - sample_quad_bezier(ps, t)).abs();
    if dist < selected_dist {
      selected_dist = dist;
      selected_t = t;
    }
  }

  // check any end-points
  let (start, end) = range_to_values(range);
  if start.is_finite() {
    let start_dist = (point - sample_quad_bezier(ps, start)).abs();
    if start_dist < selected_dist {
      selected_dist = start_dist;
      selected_t = start;
    }
  }
  if end.is_finite() {
    let end_dist = (point - sample_quad_bezier(ps, end)).abs();
    if end_dist < selected_dist {
      selected_dist = end_dist;
      selected_t = end;
    }
  }

  (selected_dist, selected_t)
}

#[cfg(any(test, doctest))]
mod tests {
  use float_cmp::assert_approx_eq;

  #[test]
  fn sample_quad_bezier() {
    use super::*;
    {
      let quad = [(0., 0.).into(), (2., 2.).into(), (4., 0.).into()];
      {
        let t = 0.;
        let result = sample_quad_bezier(&quad, t);
        let expected = (0., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 0.5;
        let result = sample_quad_bezier(&quad, t);
        let expected = (2., 1.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 1.;
        let result = sample_quad_bezier(&quad, t);
        let expected = (4., 0.).into();
        assert_eq!(result, expected);
      }
    }
  }

  #[test]
  fn sample_quad_bezier_direction() {
    use super::*;
    {
      let quad = [(0., 0.).into(), (2., 2.).into(), (4., 0.).into()];
      {
        let t = 0.;
        let result = sample_quad_bezier_direction(&quad, t).norm();
        let expected = (SQRT_2 / 2., SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 0.5;
        let result = sample_quad_bezier_direction(&quad, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 1.;
        let result = sample_quad_bezier_direction(&quad, t).norm();
        let expected = (SQRT_2 / 2., -SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
    }
  }
}
