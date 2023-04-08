use super::*;

pub mod cubic {
  use super::*;

  #[derive(Debug, Copy, Clone)]
  pub enum Roots {
    One(f32),
    Two(f32, f32),
    Three(f32, f32, f32),
  }

  /// Find the roots of the general cubic equation
  /// a*x^3 + b*x^2 + c*x + d = 0
  pub fn roots(a: f32, b: f32, c: f32, d: f32) -> Roots {
    // reduce to depressed cubic
    // t^3 + p*t + q = 0
    // by letting x = t - b / (3*a)
    let x_from_t = |t| t - b / (3. * a);

    let p = (3. * a * c - b * b) / (3. * a * a);
    let q =
      (2. * b.powi(3) - 9. * a * b * c + 27. * a * a * d) / (27. * a.powi(3));

    let discriminant = (q * q) / 4. + p.powi(3) / 27.;
    if discriminant > 0. {
      // one real root
      let root_discriminant = discriminant.sqrt();
      let neg_half_q = -0.5 * q;
      let u1 = neg_half_q + root_discriminant;
      let u2 = neg_half_q - root_discriminant;

      let t = u1.cbrt() + u2.cbrt();
      return Roots::One(x_from_t(t));
    } else if discriminant < 0. {
      // three real roots
      // can use the trigonometric solution
      // t = l * cos( m - n * k ), where k = 0,1,2
      let l = 2. * (-p / 3.).sqrt();
      let m = (1. / 3.) * (((3. * q) / (2. * p)) * (-3. / p).sqrt()).acos();
      let n = TAU / 3.;

      let t0 = l * (m).cos();
      let t1 = l * (m - n).cos();
      let t2 = l * (m - 2. * n).cos();

      return Roots::Three(x_from_t(t0), x_from_t(t1), x_from_t(t2));
    } else if discriminant == 0. {
      // these equalities should probably be slightly more forgiving
      // multiple root
      if p == 0. {
        // triple root at 0
        let t = 0.;
        return Roots::One(x_from_t(t));
      } else {
        // single root & double root
        let t0 = (3. * q) / p;
        let t1 = (-3. * q) / (2. * p);
        return Roots::Two(x_from_t(t0), x_from_t(t1));
      }
    }
    unimplemented!()
  }

  #[cfg(test)]
  mod tests {
    use float_cmp::approx_eq;

    use super::*;

    #[test]
    fn one_root() {
      {
        // x^3 = 0
        let (a, b, c, d) = (1., 0., 0., 0.);
        let roots = roots(a, b, c, d);
        // dbg!(roots);
        let Roots::One(x) = roots else { panic!("Incorrect number of roots found") };
        assert!(approx_eq!(f32, x, 0.));
      }

      {
        // x^3 - 1 = 0
        let (a, b, c, d) = (1., 0., 0., -1.);
        let roots = roots(a, b, c, d);
        // dbg!(roots);
        let Roots::One(x) = roots else { panic!("Incorrect number of roots found") };
        assert!(approx_eq!(f32, x, 1.));
      }

      {
        // x^3 - 27 = 0
        let (a, b, c, d) = (1., 0., 0., -27.);
        let roots = roots(a, b, c, d);
        // dbg!(roots);
        let Roots::One(x) = roots else { panic!("Incorrect number of roots found") };
        assert!(approx_eq!(f32, x, 3.));
      }

      {
        // x^3 + 8 = 0
        let (a, b, c, d) = (1., 0., 0., 8.);
        let roots = roots(a, b, c, d);
        // dbg!(roots);
        let Roots::One(x) = roots else { panic!("Incorrect number of roots found") };
        assert!(approx_eq!(f32, x, -2.));
      }
    }

    #[test]
    fn two_roots() {
      {
        // x^3 + 3x^2 - 4 = 0
        let (a, b, c, d) = (1., 3., 0., -4.);
        let roots = roots(a, b, c, d);
        // dbg!(roots);
        let Roots::Two(x0, x1) = roots else { panic!("Incorrect number of roots found") };
        assert!(approx_eq!(f32, x0, 1.));
        assert!(approx_eq!(f32, x1, -2.));
      }

      {
        // 2x^3 - 3x^2 + 1 = 0
        let (a, b, c, d) = (2., -3., 0., 1.);
        let roots = roots(a, b, c, d);
        // dbg!(roots);
        let Roots::Two(x0, x1) = roots else { panic!("Incorrect number of roots found") };
        assert!(approx_eq!(f32, x0, -0.5));
        assert!(approx_eq!(f32, x1, 1.));
      }
    }

    #[test]
    fn three_roots() {
      {
        // x^3 - 6x^2 + 11x -6 = 0
        let (a, b, c, d) = (1., -6., 11., -6.);
        let roots = roots(a, b, c, d);
        // dbg!(roots);
        let Roots::Three(x0, x1, x2) = roots else { panic!("Incorrect number of roots found") };
        assert!(approx_eq!(f32, x0, 3.));
        assert!(approx_eq!(f32, x1, 2.));
        assert!(approx_eq!(f32, x2, 1.));
      }

      {
        // x^3 -23x^2 + 142x - 120 = 0
        let (a, b, c, d) = (1., -23., 142., -120.);
        let roots = roots(a, b, c, d);
        // dbg!(roots);
        let Roots::Three(x0, x1, x2) = roots else { panic!("Incorrect number of roots found") };
        assert!(approx_eq!(f32, x0, 12.));
        assert!(approx_eq!(f32, x1, 10.));
        assert!(approx_eq!(f32, x2, 1.));
      }

      {
        // x^3 -12x^2 + 39x - 28 = 0
        let (a, b, c, d) = (1., -12., 39., -28.);
        let roots = roots(a, b, c, d);
        // dbg!(roots);
        let Roots::Three(x0, x1, x2) = roots else { panic!("Incorrect number of roots found") };
        assert!(approx_eq!(f32, x0, 7.));
        assert!(approx_eq!(f32, x1, 4.));
        assert!(approx_eq!(f32, x2, 1.));
      }

      {
        // 2x^3 - 38x^2 + 228x - 432 = 0
        let (a, b, c, d) = (2., -38., 228., -432.);
        let roots = roots(a, b, c, d);
        // dbg!(roots);
        let Roots::Three(x0, x1, x2) = roots else { panic!("Incorrect number of roots found") };
        assert!(approx_eq!(f32, x0, 9.));
        assert!(approx_eq!(f32, x1, 6.));
        assert!(approx_eq!(f32, x2, 4.));
      }
    }
  }
}
