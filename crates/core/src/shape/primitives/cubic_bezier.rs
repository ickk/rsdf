use super::*;

/// Cubic bezier curve primitive
pub struct CubicBezier;

impl Primitive for CubicBezier {
  type Ts = ArrayVec<f32, 6>;

  #[inline]
  #[rustfmt::skip]
  fn sample(ps: &[Point], t: f32) -> Point {
    let v1 = ps[1] - ps[0];
    let v2 = ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector();
    let v3 = ps[3].as_vector() - 3f32*ps[2].as_vector() + 3f32*ps[1].as_vector() - ps[0].as_vector();

    (t.powi(3)*v3 + 3f32*t*t*v2 + 3f32*t*v1 + ps[0].as_vector()).as_point()
  }

  #[inline]
  #[rustfmt::skip]
  fn sample_derivative(ps: &[Point], t: f32) -> Vector {
    3f32*(ps[1] - ps[0])
      + 6f32*t*(ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector())
      + 3f32*t*t
        * (ps[3].as_vector() - 3f32*ps[2].as_vector() + 3f32*ps[1].as_vector() - ps[0].as_vector())
  }

  #[rustfmt::skip]
  fn find_normals<R: RangeBounds<f32>>(
    ps: &[Point],
    point: Point,
    range: R,
  ) -> ArrayVec<f32, 6> {
    let v0 = point - ps[0];
    let v1 = ps[1] - ps[0];
    let v2 = ps[2].as_vector() - 2f32*ps[1].as_vector() + ps[0].as_vector();
    let v3 = ps[3].as_vector() - 3f32*ps[2].as_vector() + 3f32*ps[1].as_vector() - ps[0].as_vector();

    let polynomial = [
      -v1.dot(v0),
      3f32*v1.dot(v1) - 2f32*v2.dot(v0),
      9f32*v1.dot(v2) - v3.dot(v0),
      4f32*v1.dot(v3) + 6f32*v2.dot(v2),
      5f32*v2.dot(v3),
      v3.dot(v3),
    ];

    roots_in_range(&polynomial, range)
  }
}

#[cfg(any(test, doctest))]
mod tests {
  use float_cmp::assert_approx_eq;

  #[test]
  fn sample() {
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
        let result = CubicBezier::sample(&cubic, t);
        let expected = (0., 0.).into();
        assert_eq!(result, expected);
      }
      {
        let t = 0.5;
        let result = CubicBezier::sample(&cubic, t);
        let expected = (2., 1.5).into();
        assert_eq!(result, expected);
      }
      {
        let t = 1.;
        let result = CubicBezier::sample(&cubic, t);
        let expected = (4., 0.).into();
        assert_eq!(result, expected);
      }
    }

    {
      let ps = vec![
        (0., 0.).into(),
        (2., 4.).into(),
        (6., 4.).into(),
        (8., 0.).into(),
      ];

      {
        let t = 0.;
        let result = CubicBezier::sample(&ps, t);
        let expected = (0., 0.).into();
        assert_approx_eq!(Point, result, expected);
      }

      {
        let t = 1.;
        let result = CubicBezier::sample(&ps, t);
        let expected = (8., 0.).into();
        assert_approx_eq!(Point, result, expected);
      }

      {
        let t = 0.5;
        let result = CubicBezier::sample(&ps, t);
        let expected = (4., 3.).into();
        assert_approx_eq!(Point, result, expected);
      }

      {
        let t = 0.25;
        let result = CubicBezier::sample(&ps, t);
        let expected = (1.8125, 2.25).into();
        assert_approx_eq!(Point, result, expected);
      }

      {
        let t = 0.75;
        let result = CubicBezier::sample(&ps, t);
        let expected = (6.1875, 2.25).into();
        assert_approx_eq!(Point, result, expected);
      }
    }
  }

  #[test]
  fn sample_derivative() {
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
        let result = CubicBezier::sample_derivative(&cubic, t).norm();
        let expected = Vector::from((1., 2.)).norm();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 0.5;
        let result = CubicBezier::sample_derivative(&cubic, t).norm();
        let expected = (1., 0.).into();
        assert_approx_eq!(Vector, result, expected);
      }
      {
        let t = 1.;
        let result = CubicBezier::sample_derivative(&cubic, t).norm();
        let expected = Vector::from((1., -2.)).norm();
        assert_approx_eq!(Vector, result, expected);
      }
    }
  }

  #[test]
  fn find_normals() {
    use super::*;

    let ps = vec![
      (0., 0.).into(),
      (2., 4.).into(),
      (6., 4.).into(),
      (8., 0.).into(),
    ];

    {
      let point = (0., 0.).into();
      let ts = CubicBezier::find_normals(&ps, point, ..);
      let expected = vec![0.];
      assert_approx_eq!(&[f32], &ts, &expected);
    }

    {
      let point = (8., 0.).into();
      let ts = CubicBezier::find_normals(&ps, point, ..);
      let expected = vec![1.];
      assert_approx_eq!(&[f32], &ts, &expected);
    }

    {
      let point = (4., 3.).into();
      let ts = CubicBezier::find_normals(&ps, point, ..);
      let expected = vec![0.5];
      assert_approx_eq!(&[f32], &ts, &expected);
    }

    {
      let point = (1.8125, 2.25).into();
      let ts = CubicBezier::find_normals(&ps, point, ..);
      let expected = vec![0.25];
      assert_approx_eq!(&[f32], &ts, &expected);
    }

    {
      let point = (6.1875, 2.25).into();
      let ts = CubicBezier::find_normals(&ps, point, ..);
      let expected = vec![0.75];
      assert_approx_eq!(&[f32], &ts, &expected);
    }
  }

  #[test]
  fn distance() {
    use super::*;
    use std::f32::consts::SQRT_2;

    let ps = vec![
      (0., 0.).into(),
      (2., 4.).into(),
      (6., 4.).into(),
      (8., 0.).into(),
    ];

    {
      let point = (0., 0.).into();
      let (dist, t) = CubicBezier::distance(&ps, point);
      let expected_dist = 0.;
      let expected_t = 0.;
      assert_approx_eq!(f32, dist, expected_dist);
      assert_approx_eq!(f32, t, expected_t);
    }
    {
      let point = (8., 0.).into();
      let (dist, t) = CubicBezier::distance(&ps, point);
      let expected_dist = 0.;
      let expected_t = 1.;
      assert_approx_eq!(f32, dist, expected_dist);
      assert_approx_eq!(f32, t, expected_t);
    }
    {
      let point = (4., 3.).into();
      let (dist, t) = CubicBezier::distance(&ps, point);
      let expected_dist = 0.;
      let expected_t = 0.5;
      assert_approx_eq!(f32, dist, expected_dist);
      assert_approx_eq!(f32, t, expected_t);
    }
    {
      let point = (4., 4.).into();
      let (dist, t) = CubicBezier::distance(&ps, point);
      let expected_dist = 1.;
      let expected_t = 0.5;
      assert_approx_eq!(f32, dist, expected_dist);
      assert_approx_eq!(f32, t, expected_t);
    }
    {
      let point = (0., -1.).into();
      let (dist, t) = CubicBezier::distance(&ps, point);
      let expected_dist = 1.;
      let expected_t = 0.;
      assert_approx_eq!(f32, dist, expected_dist);
      assert_approx_eq!(f32, t, expected_t);
    }
    {
      let point = (4., 2.5).into();
      let (dist, t) = CubicBezier::distance(&ps, point);
      let expected_dist = 0.5;
      let expected_t = 0.5;
      assert_approx_eq!(f32, dist, expected_dist);
      assert_approx_eq!(f32, t, expected_t);
    }
    {
      let point = (-1., -1.).into();
      let (dist, t) = CubicBezier::distance(&ps, point);
      let expected_dist = SQRT_2;
      let expected_t = 0.;
      assert_approx_eq!(f32, dist, expected_dist);
      assert_approx_eq!(f32, t, expected_t);
    }
  }
}
