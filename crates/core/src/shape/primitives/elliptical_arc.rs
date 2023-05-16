use super::*;
use std::f32::consts::{PI, TAU};

/// EllipticalArc primitive, given as a centre parameterisation
///
/// ```ignore
/// [
///   Point(centre_x, centre_y),
///   Point(radius_x, aspect_ratio),
///   Point(phi, _),
///   Point(arc_start_angle, arc_delta_angle),
/// ]
/// ```
pub struct EllipticalArc;

impl Primitive for EllipticalArc {
  type Ts = ArrayVec<f32, 6>;

  #[inline]
  fn sample(ps: &[Point], t: f32) -> Point {
    let params = CentreParam::from_ps(ps);
    let angle = params.theta + t * params.delta;
    params.sample_ellipse(angle)
  }

  #[inline]
  fn sample_derivative(ps: &[Point], t: f32) -> Vector {
    let params = CentreParam::from_ps(ps);
    let angle = params.theta + t * params.delta;
    // we must negate the derivative when the curve is reversed.
    let sign = 1f32.copysign(params.delta);
    params.sample_ellipse_derivative(angle) * sign
  }

  #[inline]
  fn find_normals<R: RangeBounds<f32> + Clone>(
    ps: &[Point],
    point: Point,
    _range: R,
  ) -> Self::Ts {
    // TODO: I hate this.
    let range = 0f32..=1f32;

    let params = CentreParam::from_ps(ps);
    params
      .find_normals(point)
      .iter()
      .map(|angle| (angle - params.theta) / params.delta)
      .filter(|t| range.contains(t))
      .collect()
  }
}

/// A Centre Parameterisation of an ellipse
//
// Following: Goessner, S. "A Generalized Approach to Parameterizing Planar
// Elliptical Arcs" DOI:10.13140/RG.2.2.23485.15846
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct CentreParam {
  /// The coordinates of the **center** of the ellipse
  pub centre: Point,
  /// The **x-radius** of the ellipse
  pub r: f32,
  /// The **aspect ratio** of the ellipse; y-radius/x-radius
  ///
  /// Note: we use aspect ratio so that the arc can be scaled up purely by
  /// scaling `r`. A circle has an aspect-ratio of 1.
  ///
  /// https://en.wikipedia.org/wiki/Aspect_ratio#Ellipses
  pub k: f32,
  /// The ellipse's **axes angle** with respect to the coordinate system of the
  /// centre point.
  pub phi: f32,
  /// The **start angle** of the elliptical arc prior to stretch & rotate
  /// operations
  pub theta: f32,
  /// The **sweep angle** of the elliptical arc
  pub delta: f32,
}

impl CentreParam {
  /// Decompose a slice of `Point`s into a centre parameterisation of an
  /// ellipse
  #[inline]
  pub fn from_ps(ps: &[Point]) -> Self {
    CentreParam {
      centre: ps[0],
      r: ps[1].x,
      k: ps[1].y,
      phi: ps[2].x,
      theta: ps[3].x,
      delta: ps[3].y,
    }
  }

  /// Compose an array of `Point`s from a centre parameterisation of an
  /// ellipse
  #[inline]
  pub fn to_ps(self) -> [Point; 4] {
    [
      self.centre,
      (self.r, self.k).into(),
      (self.phi, f32::NAN).into(),
      (self.theta, self.delta).into(),
    ]
  }

  /// Sample the ellipse described by the parameters: `centre`, `r`, `k`, `phi`
  /// at the given pseudo_angle
  ///
  /// Note: does not take `theta` or `delta` into account.
  pub fn sample_ellipse(&self, angle: f32) -> Point {
    let ry = self.k * self.r;
    let (phi_sin, phi_cos) = self.phi.sin_cos();
    let (ang_sin, ang_cos) = angle.sin_cos();
    Point {
      x: self.r * phi_cos * ang_cos - ry * phi_sin * ang_sin + self.centre.x,
      y: self.r * phi_sin * ang_cos + ry * phi_cos * ang_sin + self.centre.y,
    }
  }

  /// Sample the derivative of the ellipse described by the parameters:
  /// `centre`, `r`, `k`, `phi` at the given pseudo_angle
  ///
  /// Note: does not take `theta` or `delta` into account.
  pub fn sample_ellipse_derivative(&self, angle: f32) -> Vector {
    let ry = self.k * self.r;
    let (phi_sin, phi_cos) = self.phi.sin_cos();
    let (ang_sin, ang_cos) = angle.sin_cos();
    Vector {
      x: -self.r * phi_cos * ang_sin - ry * phi_sin * ang_cos,
      y: -self.r * phi_sin * ang_sin + ry * phi_cos * ang_cos,
    }
  }

  /// Find the pseudo angle of the ellipse where a normal points in the
  /// direction of the given `Point`
  ///
  /// Note: does not take `theta` or `delta` into account. Just returns angles
  /// for the ellipse itself.
  pub fn find_normals(&self, point: Point) -> [f32; 8] {
    // Much like the beziers, we need to find the zeros of the equation:
    //   N(t) = (p(t) - P) dot dp/dt
    // where t_0  N(t_0) = 0 is the pseudo angle of the ellipse that is
    // closest to the point in question.
    let (a, b) = f32::sin_cos(self.phi);
    let (c, p, r) = (self.centre, point, Point::new(self.r, self.r * self.k));

    let m = 0.5 * (r.y * r.y - r.x * r.x) * (a * a + b * b);
    let n = r.x * (a * (p.y - c.y) + b * (p.x - c.x));
    let o = r.y * (a * (p.x - c.x) + b * (c.y - p.y));
    // `f` is N(t).
    let f = |t: f32| m * (2. * t).sin() + n * t.sin() + o * t.cos();
    // easily twice differentiable so we can use Halley's method.
    let df = |t: f32| 2. * m * (2. * t).cos() + n * t.cos() - o * t.sin();
    let ddf = |t: f32| -4. * m * (2. * t).sin() - n * t.sin() - o * t.cos();
    // assume a circle, should give close-ish initial guesses.
    let mut guess = f32::atan2(c.y - p.y, c.x - p.x) - self.phi;
    if guess < 0f32 {
      guess += TAU
    }
    // modulo shenanigans with TAU is to renormalise the result to [0, TAU].
    let t0 = (halleys_method(guess, f, df, ddf) % TAU + TAU) % TAU;
    let t1 =
      (halleys_method((guess + PI) % TAU, f, df, ddf) % TAU + TAU) % TAU;

    // this is ugly but we need to make sure we include the extra roots incase
    // there's a positive or negative offset for the arc
    // TODO: clean this up
    [
      t0,
      t1,
      t0 + TAU,
      t1 + TAU,
      t0 - TAU,
      t1 - TAU,
      t0 - 2. * TAU,
      t1 - 2. * TAU,
    ]
  }
}

impl float_cmp::ApproxEq for CentreParam {
  type Margin = float_cmp::F32Margin;

  fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
    let margin = margin.into();
    self.centre.approx_eq(other.centre, margin)
      && self.r.approx_eq(other.r, margin)
      && self.k.approx_eq(other.k, margin)
      && self.phi.approx_eq(other.phi, margin)
      && self.theta.approx_eq(other.theta, margin)
      && self.delta.approx_eq(other.delta, margin)
  }
}

/// An Endpoint parameterisation of an ellipse.
///
/// Corresponds to the parameterisation used by SVG elliptical arc path
/// commands.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct EndpointParam {
  /// The **starting point** of the arc
  pub start: Point,
  /// The **x-radius** of the ellipse
  pub rx: f32,
  /// The **y-radius** of the ellipse
  pub ry: f32,
  /// the angle from the x-axis of the current coordinate system to the x-axis
  /// of the ellipse
  pub phi: f32,
  /// large-arc-flag
  pub large_arc: bool,
  /// sweep-flag
  pub sweep_ccw: bool,
  /// The final point of the arc
  pub end: Point,
}

impl float_cmp::ApproxEq for EndpointParam {
  type Margin = float_cmp::F32Margin;

  fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
    let margin = margin.into();
    self.start.approx_eq(other.start, margin)
      && self.rx.approx_eq(other.rx, margin)
      && self.ry.approx_eq(other.ry, margin)
      && self.phi.approx_eq(other.phi, margin)
      && self.large_arc == other.large_arc
      && self.sweep_ccw == other.sweep_ccw
      && self.end.approx_eq(other.end, margin)
  }
}

impl From<CentreParam> for EndpointParam {
  fn from(
    params @ CentreParam {
      r,
      k,
      phi,
      theta,
      delta,
      ..
    }: CentreParam,
  ) -> EndpointParam {
    let start = params.sample_ellipse(theta);
    let end = params.sample_ellipse(theta + delta);
    let large_arc = delta.abs() > PI;
    let sweep_ccw = delta > 0f32;
    EndpointParam {
      start,
      rx: r,
      ry: k * r,
      phi,
      large_arc,
      sweep_ccw,
      end,
    }
  }
}

impl From<EndpointParam> for CentreParam {
  fn from(
    EndpointParam {
      start,
      rx,
      ry,
      phi,
      large_arc,
      sweep_ccw,
      end,
    }: EndpointParam,
  ) -> CentreParam {
    // conversion algorithm: https://www.w3.org/TR/SVG/implnote.html
    // with patches from:
    // https://mortoray.com/rendering-an-svg-elliptical-arc-as-bezier-curves/
    let (p0, p1) = (start, end);
    let (mut rx, mut ry) = (rx.abs(), ry.abs());
    let (phi_sin, phi_cos) = phi.sin_cos();
    let dp_half = Point {
      x: (p0.x - p1.x) / 2.,
      y: (p0.y - p1.y) / 2.,
    };
    // NOTE: this algorithm obviously fails when `start == end`. There would be
    // infinitely many ellipses that fit the constraints.
    // TODO: add a check for this, because otherwise we will try to divide by
    // zero..
    let p0_prime = Point {
      x: phi_cos * dp_half.x + phi_sin * dp_half.y,
      y: -phi_sin * dp_half.x + phi_cos * dp_half.y,
    };
    let mut rx_2 = rx * rx;
    let mut ry_2 = ry * ry;
    let p0_prime_2 = Point {
      x: p0_prime.x * p0_prime.x,
      y: p0_prime.y * p0_prime.y,
    };
    {
      let cr = p0_prime_2.x / rx_2 + p0_prime_2.y / ry_2;
      if cr > 1. {
        let s = cr.sqrt();
        rx *= s;
        ry *= s;
        rx_2 = rx * rx;
        ry_2 = ry * ry;
      }
    }
    let q = {
      let dq = rx_2 * p0_prime_2.y + ry_2 * p0_prime_2.x;
      let mut pq = (rx_2 * ry_2 - dq) / dq;
      if pq.is_infinite() {
        pq = 0.;
      }
      f32::max(0., pq)
        .sqrt()
        .copysign(-((large_arc == sweep_ccw) as i32) as f32)
    };
    let c_prime = Point {
      x: q * rx * p0_prime.y / ry,
      y: -q * ry * p0_prime.x / rx,
    };
    let centre = Point {
      x: phi_cos * c_prime.x - phi_sin * c_prime.y + (p0.x + p1.x) / 2.,
      y: phi_sin * c_prime.x + phi_cos * c_prime.y + (p0.y + p1.y) / 2.,
    };
    let v = Vector::new(
      (p0_prime.x - c_prime.x) / rx,
      (p0_prime.y - c_prime.y) / ry,
    );
    let theta = Vector::angle(Vector::new(1., 0.), v);
    let delta = {
      let mut delta = Vector::angle(
        v,
        Vector::new(
          (-p0_prime.x - c_prime.x) / rx,
          (-p0_prime.y - c_prime.y) / ry,
        ),
      ) % TAU;
      if !sweep_ccw && delta > 0f32 {
        delta -= TAU;
      } else if sweep_ccw && delta < 0f32 {
        delta += TAU;
      };
      delta
    };

    CentreParam {
      centre,
      r: rx,
      k: ry / rx,
      phi,
      theta,
      delta,
    }
  }
}

#[cfg(any(test, doctest))]
mod tests {
  use float_cmp::assert_approx_eq;
  #[test]
  fn endpoint_from_centre() {
    use super::*;
    {
      let centre = CentreParam {
        centre: (4., 2.).into(),
        r: 2.,
        k: 1.5,
        phi: 0f32,
        theta: PI,
        delta: -PI / 2.,
      };
      let endpoint = EndpointParam::from(centre);
      let expected = EndpointParam {
        start: (2., 2.).into(),
        rx: 2.,
        ry: 3.,
        phi: 0f32,
        large_arc: false,
        sweep_ccw: false,
        end: (4., 5.).into(),
      };
      assert_approx_eq!(EndpointParam, endpoint, expected);
    }
    {
      // the same as above but rotated 45 degrees
      let centre = CentreParam {
        centre: (2. + SQRT_2, 2. + SQRT_2).into(),
        r: 2.,
        k: 1.5,
        phi: PI / 4.,
        theta: PI,
        delta: -PI / 2.,
      };
      let endpoint = EndpointParam::from(centre);
      let expected = EndpointParam {
        start: (2., 2.).into(),
        rx: 2.,
        ry: 3.,
        phi: PI / 4.,
        large_arc: false,
        sweep_ccw: false,
        end: (
          (2. + 2. * SQRT_2 - 3.) / SQRT_2,
          (2. + 2. * SQRT_2 + 3.) / SQRT_2,
        )
          .into(),
      };
      assert_approx_eq!(EndpointParam, endpoint, expected);
    }
    // flags
    {
      let centre = CentreParam {
        centre: (4., 2.).into(),
        r: 2.,
        k: 1.5,
        phi: 0f32,
        theta: PI,
        delta: 3. * PI / 2.,
      };
      let endpoint = EndpointParam::from(centre);
      let expected = EndpointParam {
        start: (2., 2.).into(),
        rx: 2.,
        ry: 3.,
        phi: 0f32,
        large_arc: true,
        sweep_ccw: true,
        end: (4., 5.).into(),
      };
      assert_approx_eq!(EndpointParam, endpoint, expected);
    }
    {
      let centre = CentreParam {
        centre: (2., 5.).into(),
        r: 2.,
        k: 1.5,
        phi: 0f32,
        theta: -PI / 2.,
        delta: PI / 2.,
      };
      let endpoint = EndpointParam::from(centre);
      let expected = EndpointParam {
        start: (2., 2.).into(),
        rx: 2.,
        ry: 3.,
        phi: 0f32,
        large_arc: false,
        sweep_ccw: true,
        end: (4., 5.).into(),
      };
      assert_approx_eq!(EndpointParam, endpoint, expected);
    }
    {
      let centre = CentreParam {
        centre: (2., 5.).into(),
        r: 2.,
        k: 1.5,
        phi: 0f32,
        theta: -PI / 2.,
        delta: -3. * PI / 2.,
      };
      let endpoint = EndpointParam::from(centre);
      let expected = EndpointParam {
        start: (2., 2.).into(),
        rx: 2.,
        ry: 3.,
        phi: 0f32,
        large_arc: true,
        sweep_ccw: false,
        end: (4., 5.).into(),
      };
      assert_approx_eq!(EndpointParam, endpoint, expected);
    }
    {
      // special case: starting and ending angle result in coincident points
      let centre = CentreParam {
        centre: (2., 2.).into(),
        r: 1.,
        k: 1.,
        phi: 0f32,
        theta: 0f32,
        delta: TAU,
      };
      let endpoint = EndpointParam::from(centre);
      let expected = EndpointParam {
        start: (3., 2.).into(),
        rx: 1.,
        ry: 1.,
        phi: 0f32,
        large_arc: true,
        sweep_ccw: true,
        end: (3., 2.).into(),
      };
      assert_approx_eq!(EndpointParam, endpoint, expected);
    }
  }

  #[test]
  fn centre_from_endpoint() {
    use super::*;
    {
      let endpoint = EndpointParam {
        start: (2., 2.).into(),
        rx: 2.,
        ry: 3.,
        phi: 0f32,
        large_arc: false,
        sweep_ccw: false,
        end: (4., 5.).into(),
      };
      let centre = CentreParam::from(endpoint);
      let expected = CentreParam {
        centre: (4., 2.).into(),
        r: 2.,
        k: 1.5,
        phi: 0f32,
        theta: PI,
        delta: -PI / 2.,
      };
      assert_approx_eq!(CentreParam, centre, expected);
    }
    {
      let endpoint = EndpointParam {
        start: (2., 2.).into(),
        rx: 2.,
        ry: 3.,
        phi: 0f32,
        large_arc: true,
        sweep_ccw: true,
        end: (4., 5.).into(),
      };
      let centre = CentreParam::from(endpoint);
      let expected = CentreParam {
        centre: (4., 2.).into(),
        r: 2.,
        k: 1.5,
        phi: 0f32,
        theta: PI,
        delta: 3. * PI / 2.,
      };
      assert_approx_eq!(CentreParam, centre, expected);
    }
    {
      let endpoint = EndpointParam {
        start: (2., 2.).into(),
        rx: 2.,
        ry: 3.,
        phi: 0f32,
        large_arc: false,
        sweep_ccw: true,
        end: (4., 5.).into(),
      };
      let centre = CentreParam::from(endpoint);
      let expected = CentreParam {
        centre: (2., 5.).into(),
        r: 2.,
        k: 1.5,
        phi: 0f32,
        theta: -PI / 2.,
        delta: PI / 2.,
      };
      assert_approx_eq!(CentreParam, centre, expected);
    }
    {
      let endpoint = EndpointParam {
        start: (2., 2.).into(),
        rx: 2.,
        ry: 3.,
        phi: 0f32,
        large_arc: true,
        sweep_ccw: false,
        end: (4., 5.).into(),
      };
      let centre = CentreParam::from(endpoint);
      let expected = CentreParam {
        centre: (2., 5.).into(),
        r: 2.,
        k: 1.5,
        phi: 0f32,
        theta: -PI / 2.,
        delta: -3. * PI / 2.,
      };
      assert_approx_eq!(CentreParam, centre, expected);
    }
  }

  #[test]
  fn params_find_normals() {
    use super::*;
    {
      let circle = CentreParam {
        centre: (0f32, 0f32).into(),
        r: 1f32,
        k: 1f32,
        phi: 0f32,
        theta: 0f32,
        delta: 0f32,
      };
      let point = Point::from((2., 0.));
      let ts: Vec<_> = circle
        .find_normals(point)
        .into_iter()
        .filter(|t| (0f32..TAU).contains(t))
        .collect();
      assert_approx_eq!(&[f32], &ts, &[PI, 0f32]);

      let point = Point::from((0., 2.));
      let ts: Vec<_> = circle
        .find_normals(point)
        .into_iter()
        .filter(|t| (0f32..TAU).contains(t))
        .collect();
      assert_approx_eq!(&[f32], &ts, &[3. * PI / 2., PI / 2.]);

      let point = Point::from((1., 1.));
      let ts: Vec<_> = circle
        .find_normals(point)
        .into_iter()
        .filter(|t| (0f32..TAU).contains(t))
        .collect();
      assert_approx_eq!(&[f32], &ts, &[5. * PI / 4., PI / 4.]);
    }

    {
      let circle = CentreParam {
        centre: (0f32, 0f32).into(),
        r: 1f32,
        k: 1f32,
        phi: PI / 4f32,
        theta: 0f32,
        delta: 0f32,
      };
      let point = Point::from((2., 0.));
      let ts: Vec<_> = circle
        .find_normals(point)
        .into_iter()
        .filter(|t| (0f32..TAU).contains(t))
        .collect();
      assert_approx_eq!(&[f32], &ts, &[3. * PI / 4., 7. * PI / 4.]);

      let point = Point::from((0., 2.));
      let ts: Vec<_> = circle
        .find_normals(point)
        .into_iter()
        .filter(|t| (0f32..TAU).contains(t))
        .collect();
      assert_approx_eq!(&[f32], &ts, &[5. * PI / 4., PI / 4.]);

      let point = Point::from((1., 1.));
      let ts: Vec<_> = circle
        .find_normals(point)
        .into_iter()
        .filter(|t| (0f32..TAU).contains(t))
        .collect();
      assert_approx_eq!(&[f32], &ts, &[PI, 0f32]);
    }

    {
      let circle = CentreParam {
        centre: (1f32, 2f32).into(),
        r: 1f32,
        k: 1f32,
        phi: 0f32,
        theta: 0f32,
        delta: 0f32,
      };
      let point = Point::from((3., 2.));
      let ts: Vec<_> = circle
        .find_normals(point)
        .into_iter()
        .filter(|t| (0f32..TAU).contains(t))
        .collect();
      assert_approx_eq!(&[f32], &ts, &[PI, 0f32]);

      let point = Point::from((1., 4.));
      let ts: Vec<_> = circle
        .find_normals(point)
        .into_iter()
        .filter(|t| (0f32..TAU).contains(t))
        .collect();
      assert_approx_eq!(&[f32], &ts, &[3. * PI / 2., PI / 2.]);

      let point = Point::from((2., 3.));
      let ts: Vec<_> = circle
        .find_normals(point)
        .into_iter()
        .filter(|t| (0f32..TAU).contains(t))
        .collect();
      assert_approx_eq!(&[f32], &ts, &[5. * PI / 4., PI / 4.]);
    }
  }

  #[test]
  fn params_sample_ellipse() {
    use super::*;
    {
      let params = CentreParam {
        centre: Point::new(20., 20.),
        r: 10.,
        k: 1.,
        phi: 0.,
        // CentreParam::sample_ellipse doesn't use these values
        theta: f32::NAN,
        delta: f32::NAN,
      };

      {
        let angle = 0.;
        let sample = params.sample_ellipse(angle);
        let expected_sample = Point::new(30., 20.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = PI / 4.;
        let sample = params.sample_ellipse(angle);
        let expected_sample =
          Point::new(20. + 10. * SQRT_2 / 2., 20. + 10. * SQRT_2 / 2.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = PI / 2.;
        let sample = params.sample_ellipse(angle);
        let expected_sample = Point::new(20., 30.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = PI;
        let sample = params.sample_ellipse(angle);
        let expected_sample = Point::new(10., 20.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = 3. * PI / 2.;
        let sample = params.sample_ellipse(angle);
        let expected_sample = Point::new(20., 10.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = TAU;
        let sample = params.sample_ellipse(angle);
        let expected_sample = Point::new(30., 20.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = TAU + PI;
        let sample = params.sample_ellipse(angle);
        let expected_sample = Point::new(10., 20.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = -PI;
        let sample = params.sample_ellipse(angle);
        let expected_sample = Point::new(10., 20.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
    }
  }

  #[test]
  fn params_sample_ellipse_derivative() {
    use super::*;
    {
      let params = CentreParam {
        centre: Point::new(2., 2.),
        r: 1.,
        k: 1.,
        phi: 0.,
        // CentreParam::sample_ellipse_derivative doesn't use these values
        theta: f32::NAN,
        delta: f32::NAN,
      };

      {
        let angle = 0.;
        let sample = params.sample_ellipse_derivative(angle);
        let expected_sample = Vector::new(0., 1.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = PI / 4.;
        let sample = params.sample_ellipse_derivative(angle);
        let expected_sample = Vector::new(-SQRT_2 / 2., SQRT_2 / 2.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = PI / 2.;
        let sample = params.sample_ellipse_derivative(angle);
        let expected_sample = Vector::new(-1., 0.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = PI;
        let sample = params.sample_ellipse_derivative(angle);
        let expected_sample = Vector::new(0., -1.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = 3. * PI / 2.;
        let sample = params.sample_ellipse_derivative(angle);
        let expected_sample = Vector::new(1., 0.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = TAU;
        let sample = params.sample_ellipse_derivative(angle);
        let expected_sample = Vector::new(0., 1.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = TAU + PI;
        let sample = params.sample_ellipse_derivative(angle);
        let expected_sample = Vector::new(0., -1.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }

      {
        let angle = -PI;
        let sample = params.sample_ellipse_derivative(angle);
        let expected_sample = Vector::new(0., -1.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
    }
  }

  #[test]
  fn params_sample() {
    use super::*;
    {
      let ps: [Point; 4] = [
        (2., 2.).into(),            // centre
        (1f32, 1f32).into(),        // r, k
        (PI / 4., f32::NAN).into(), // phi, _
        (PI / 4., PI).into(),       // theta, delta
      ];
      {
        let t = 0.;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(2., 3.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 1.;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(2., 1.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 0.5;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(1., 2.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = -0.25;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(2. + SQRT_2 / 2., 2. + SQRT_2 / 2.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 1.25;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(2. + SQRT_2 / 2., 2. - SQRT_2 / 2.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 2.;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(2., 3.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = -2.;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(2., 3.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
    }
    {
      let ps: [Point; 4] = [
        (0., 0.).into(),           // centre
        (1f32, 2f32).into(),       // r, k
        (0., f32::NAN).into(),     // phi, _
        (PI / 4., PI / 2.).into(), // theta, delta
      ];
      {
        let t = 0.5;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(0., 2.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 1.5;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(-1., 0.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 2.5;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(0., -2.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 3.5;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(1., 0.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = -0.5;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(1., 0.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = -1.5;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(0., -2.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = -2.5;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(-1., 0.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = -3.5;
        let sample = EllipticalArc::sample(&ps, t);
        let expected_sample = Point::new(0., 2.);
        assert_approx_eq!(Point, sample, expected_sample, epsilon = 0.001);
      }
    }
  }

  #[test]
  fn params_sample_derivative() {
    use super::*;
    {
      let ps: [Point; 4] = [
        (2., 2.).into(),            // centre
        (1f32, 1f32).into(),        // r, k
        (PI / 4., f32::NAN).into(), // phi, _
        (PI / 4., PI).into(),       // theta, delta
      ];
      {
        let t = 0.;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(-1., 0.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 1.;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(1., 0.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 0.5;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(0., -1.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = -0.25;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(-SQRT_2 / 2., SQRT_2 / 2.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 1.25;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(SQRT_2 / 2., SQRT_2 / 2.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 2.;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(-1., 0.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = -2.;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(-1., 0.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
    }
    {
      let ps: [Point; 4] = [
        (0., 0.).into(),           // centre
        (1f32, 2f32).into(),       // r, k
        (0., f32::NAN).into(),     // phi, _
        (PI / 4., PI / 2.).into(), // theta, delta
      ];
      {
        let t = 0.5;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(-1., 0.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 1.5;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(0., -2.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 2.5;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(1., 0.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = 3.5;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(0., 2.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = -0.5;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(0., 2.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = -1.5;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(1., 0.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = -2.5;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(0., -2.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
      {
        let t = -3.5;
        let sample = EllipticalArc::sample_derivative(&ps, t);
        let expected_sample = Vector::new(-1., 0.);
        assert_approx_eq!(Vector, sample, expected_sample, epsilon = 0.001);
      }
    }
  }

  #[test]
  fn find_normals() {
    use super::*;
    {
      let ps: [Point; 4] = [
        (0f32, 0f32).into(),     // centre
        (1f32, 1f32).into(),     // r, k
        (0f32, f32::NAN).into(), // phi, _
        (0f32, PI).into(),       // theta, delta
      ];

      let point = Point::from((2., 0.));
      let ts = EllipticalArc::find_normals(&ps, point, 0f32..=1f32);
      assert_approx_eq!(&[f32], &ts, &[1., 0.]);

      let point = Point::from((0., 2.));
      let ts = EllipticalArc::find_normals(&ps, point, 0f32..=1f32);
      assert_approx_eq!(&[f32], &ts, &[0.5]);

      let point = Point::from((0., 2.));
      let ts = EllipticalArc::find_normals(&ps, point, 0f32..=2f32);
      assert_approx_eq!(&[f32], &ts, &[0.5]);

      let point = Point::from((1., 1.));
      let ts = EllipticalArc::find_normals(&ps, point, 0f32..=1f32);
      assert_approx_eq!(&[f32], &ts, &[0.25]);
    }

    {
      let ps: [Point; 4] = [
        (1f32, 2f32).into(),        // centre
        (1f32, 1f32).into(),        // r, k
        (PI / 4., f32::NAN).into(), // phi, _
        (0., PI).into(),            // theta, delta
      ];

      let point = Point::from((1., 4.));
      let ts = EllipticalArc::find_normals(&ps, point, 0f32..=1f32);
      assert_approx_eq!(&[f32], &ts, &[0.25]);

      let point = Point::from((0., 3.));
      let ts = EllipticalArc::find_normals(&ps, point, 0f32..=1f32);
      assert_approx_eq!(&[f32], &ts, &[0.5]);

      let point = Point::from((-1., 2.));
      let ts = EllipticalArc::find_normals(&ps, point, 0f32..=1f32);
      assert_approx_eq!(&[f32], &ts, &[0.75]);
    }

    {
      // edge case: theta + delta > TAU
      let ps: [Point; 4] = [
        (0f32, 0f32).into(),     // centre
        (1f32, 1f32).into(),     // r, k
        (0f32, f32::NAN).into(), // phi, _
        (PI, TAU).into(),        // theta, delta
      ];

      let point = Point::from((2., 0.));
      let ts = EllipticalArc::find_normals(&ps, point, 0f32..=1f32);
      // presicision issues reduce the number of results to 2
      // assert_approx_eq!(&[f32], &ts, &[0., 1., 0.5]);
      assert_approx_eq!(&[f32], &ts, &[1., 0.5]);
    }
  }
}
