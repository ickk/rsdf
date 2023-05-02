use super::*;

/// Quadratic bezier curve primitive
pub struct QuadBezier;

impl Primitive for QuadBezier {
  type Ts = ArrayVec<f32, 4>;

  #[inline]
  #[rustfmt::skip]
  fn sample(ps: &[Point], t: f32) -> Point {
    ps[0]
      + 2f32*t
        * (ps[1] - ps[0])
      + t*t
        * (ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector())
  }

  #[inline]
  #[rustfmt::skip]
  fn sample_derivative(ps: &[Point], t: f32) -> Vector {
    2f32*(ps[1] - ps[0])
      + 2f32*t
        * (ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector())
  }

  #[inline]
  fn pseudo_distance<R: RangeBounds<f32> + Clone>(
    ps: &[Point],
    point: Point,
    range: R,
  ) -> (/* dist */ f32, /* t */ f32) {
    let mut selected_t = 0.; // initial value doesn't matter
    let mut selected_dist = f32::INFINITY;

    // check perpendiculars
    for t in Self::find_normals(ps, point, range.clone()) {
      let dist = (point - Self::sample(ps, t)).abs();
      if dist < selected_dist {
        selected_dist = dist;
        selected_t = t;
      }
    }

    // check any end-points
    let (start, end) = range_to_values(range);
    if start.is_finite() {
      let start_dist = (point - Self::sample(ps, start)).abs();
      if start_dist < selected_dist {
        selected_dist = start_dist;
        selected_t = start;
      }
    }
    if end.is_finite() {
      let end_dist = (point - Self::sample(ps, end)).abs();
      if end_dist < selected_dist {
        selected_dist = end_dist;
        selected_t = end;
      }
    }

    (selected_dist, selected_t)
  }

  fn find_normals<R: RangeBounds<f32> + Clone>(
    ps: &[Point],
    point: Point,
    range: R,
  ) -> ArrayVec<f32, 4> {
    let v2 = ps[2].as_vector() - 2f32 * ps[1].as_vector() + ps[0].as_vector();
    // check if the curve degenerates into a line
    if v2 == Vector::ZERO {
      let line = [ps[0], ps[1]];
      let mut a = ArrayVec::new();
      Line::find_normals(&line, point, range)
        .iter()
        .for_each(|&f| a.push(f));
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
        let result = QuadBezier::sample(&quad, t);
        let expected = (0., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 0.5;
        let result = QuadBezier::sample(&quad, t);
        let expected = (2., 1.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 1.;
        let result = QuadBezier::sample(&quad, t);
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
        let result = QuadBezier::sample_derivative(&quad, t).norm();
        let expected = (SQRT_2 / 2., SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 0.5;
        let result = QuadBezier::sample_derivative(&quad, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 1.;
        let result = QuadBezier::sample_derivative(&quad, t).norm();
        let expected = (SQRT_2 / 2., -SQRT_2 / 2.).into();
        assert_approx_eq!(Vector, result, expected);
      }
    }
  }
}
