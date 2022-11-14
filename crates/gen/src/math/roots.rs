use super::*;

#[derive(Debug, Copy, Clone)]
pub enum Roots {
  One(f32),
  Two(f32, f32),
  Three(f32, f32, f32),
}

/// Find the roots of the general cubic equation
/// a*x^3 + b*x^2 + c*x + d = 0
pub fn cubic_roots(a: f32, b: f32, c: f32, d: f32) -> Roots {
  // reduce to depressed cubic
  // t^3 + p*t + q = 0
  // by letting x = t - b / (3*a)
  let x_from_t = |t| t - b / (3. * a);

  let p = (3. * a * c - b * b) / (3. * a * a);
  let q = (2. * b.powi(3) - 9. * a * b * c + 27. * a * a * d) / (27. * a.powi(3));

  let discriminant = 27. * q * q + 4. * p.powi(3);
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
    // multiple root
    if p == 0. {
      // triple root at 0
      return Roots::One(x_from_t(0.));
    } else {
      // single root & double root
      return Roots::Two(x_from_t((3. * q) / p), x_from_t((-3. * q) / (2. * p)));
    }
  }
  unimplemented!()
}
