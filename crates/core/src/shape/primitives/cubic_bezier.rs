use super::*;

#[inline]
#[rustfmt::skip]
pub fn sample_cubic_bezier(ps: &[Point], t: f32) -> Point {
  ps[0]
    + 3f32*t
      * (ps[1] - ps[0])
    + 3f32*t*t
      * (ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector())
    + t.powi(3)
      * (ps[3].as_vector() - 3f32*ps[2].as_vector() + 3f32*ps[1].as_vector() - ps[0].as_vector())
}

// Return a vector pointing in the dirction of the tangent of a cubic bezier at
// time t.
#[inline]
#[rustfmt::skip]
pub fn sample_cubic_bezier_direction(ps: &[Point], t: f32) -> Vector {
  (
    3f32*(ps[1] - ps[0])
      + 6f32*t*(ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector())
      + 3f32*t*t
        * (ps[3].as_vector() - 3f32*ps[2].as_vector() + 3f32*ps[1].as_vector() - ps[0].as_vector())
  ).norm()
}

pub fn find_ts_cubic_bezier<R: RangeBounds<f32>>(
  ps: &[Point],
  point: Point,
  range: R,
) -> ArrayVec<f32, 6> {
  let v0 = point - ps[0];
  let v1 = ps[1] - ps[0];
  let v2 = ps[2].as_vector() - 2f32 * ps[1].as_vector() + ps[0].as_vector();
  let v3 = ps[3].as_vector() - 3f32 * ps[2].as_vector()
    + 3f32 * ps[1].as_vector()
    - ps[0].as_vector();

  let polynomial = [
    -v1.dot(v0),
    3f32 * v1.dot(v1) - 2f32 * v2.dot(v0),
    9f32 * v1.dot(v2) - v2.dot(v0),
    4f32 * v1.dot(v3) + 6f32 * v2.dot(v2),
    5f32 * v2.dot(v3),
    v3.dot(v3),
  ];

  roots_in_range(&polynomial, range)
}

#[inline]
pub fn cubic_bezier_distance(
  ps: &[Point],
  point: Point,
) -> (/* dist */ f32, /* t */ f32) {
  cubic_bezier_pseudo_distance(ps, point, 0.0..=1.0)
}

#[inline]
pub fn cubic_bezier_pseudo_distance<R: RangeBounds<f32> + Clone>(
  ps: &[Point],
  point: Point,
  range: R,
) -> (/* dist */ f32, /* t */ f32) {
  let mut selected_t = 0.; // initial value doesn't matter
  let mut selected_dist = f32::INFINITY;

  // check perpendiculars
  for t in find_ts_cubic_bezier(ps, point, range.clone()) {
    let dist = (point - sample_cubic_bezier(ps, t)).abs();
    if dist < selected_dist {
      selected_dist = dist;
      selected_t = t;
    }
  }

  // check any end-points
  let (start, end) = range_to_values(range);
  if start.is_finite() {
    let start_dist = (point - sample_cubic_bezier(ps, start)).abs();
    if start_dist < selected_dist {
      selected_dist = start_dist;
      selected_t = start;
    }
  }
  if end.is_finite() {
    let end_dist = (point - sample_cubic_bezier(ps, end)).abs();
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
  fn sample_cubic_bezier() {
    use super::*;
    {
      let cubic = [
        (0., 0.).into(),
        (1., 2.).into(),
        (3., 2.).into(),
        (4., 0.).into(),
      ];
      {
        let t = 0.;
        let result = sample_cubic_bezier(&cubic, t);
        let expected = (0., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 0.5;
        let result = sample_cubic_bezier(&cubic, t);
        let expected = (2., 1.5).into();
        assert_eq!(result, expected);
      }
      {
        let t = 1.;
        let result = sample_cubic_bezier(&cubic, t);
        let expected = (4., 0.).into();
        assert_eq!(result, expected);
      }
    }
  }

  #[test]
  fn sample_cubic_bezier_direction() {
    use super::*;
    {
      let cubic = [
        (0., 0.).into(),
        (1., 2.).into(),
        (3., 2.).into(),
        (4., 0.).into(),
      ];
      {
        let t = 0.;
        let result = sample_cubic_bezier_direction(&cubic, t).norm();
        let expected = Vector::from((1., 2.)).norm();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 0.5;
        let result = sample_cubic_bezier_direction(&cubic, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 1.;
        let result = sample_cubic_bezier_direction(&cubic, t).norm();
        let expected = Vector::from((1., -2.)).norm();
        assert_approx_eq!(Vector, result, expected);
      }
    }
  }
}
