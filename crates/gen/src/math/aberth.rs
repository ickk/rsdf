use super::Complex;
use arrayvec::ArrayVec;
use std::f32::consts::{FRAC_PI_2, TAU};

/// Find all of the roots of a polynomial using Aberth's method.
///
/// Polynomial of the form f(x) = a + b*x + c*x^2 + d*x^3 + ...
/// `polynomial` is a slice containing the coefficients [a, b, c, d, ...]
/// When two successive iterations produce roots with less than `epsilon`
/// delta, the roots are returned.
pub fn aberth<const TERMS: usize>(
  polynomial: &[f32; TERMS],
  epsilon: f32,
) -> ArrayVec<Complex, TERMS> {
  let dydx = &derivative(polynomial);
  let mut zs = initial_guesses(polynomial);
  let mut new_zs = zs.clone();

  'iteration: loop {
    for i in 0..zs.len() {
      let p_of_z = sample_polynomial(polynomial, zs[i]);
      let dydx_of_z = sample_polynomial(dydx, zs[i]);
      let sum = (0..zs.len())
        .filter(|&k| k != i)
        .fold(Complex::ZERO, |acc, k| acc + 1.0 / (zs[i] - zs[k]));

      new_zs[i] = zs[i] + p_of_z / (p_of_z * sum - dydx_of_z);
    }
    std::mem::swap(&mut zs, &mut new_zs);

    for (&z, &new_z) in std::iter::zip(&zs, &new_zs) {
      if z.real().is_nan()
        || z.imaginary().is_nan()
        || z.real().is_infinite()
        || z.imaginary().is_infinite()
      {
        panic!("Failed to converge: {z:?}");
      }
      if !z.approx_eq(new_z, epsilon) {
        continue 'iteration;
      }
    }
    return zs;
  }
}

// Initial guesses using the method from "Iteration Methods for Finding all
// Zeros of a Polynomial Simultaneously" by Oliver Aberth.
fn initial_guesses<const TERMS: usize>(
  polynomial: &[f32; TERMS],
) -> ArrayVec<Complex, TERMS> {
  // naive transformation of the polynomial to find S(w)
  let n = polynomial.len() - 1; // the degree of the polynomial

  // convert polynomial to monic form
  let mut monic: ArrayVec<f32, TERMS> = ArrayVec::new();
  for c in polynomial {
    monic.push(c / polynomial[n]);
  }

  // let z = w + a, a = - c_1 / n
  let a = -monic[n - 1] / n as f32;

  let p_of_w = {
    // we can recycle monic on the fly.
    for coefficient_index in 0..=n {
      let c = monic[coefficient_index];
      monic[coefficient_index] = 0.0;
      for (index, power, pascal) in itertools::izip!(
        0..=coefficient_index,
        (0..=coefficient_index).rev(),
        PascalRowIter::new(coefficient_index),
      ) {
        let val = c * (pascal as f32) * a.powi(power as i32);
        monic[index] += val;
      }
    }
    monic
  };

  // convert P(w) into S(w)
  let s_of_w = {
    let mut p = p_of_w;
    // skip the last coefficient
    for i in 0..n {
      p[i] = -p[i].abs()
    }
    p
  };

  // find r_0
  let mut int = 1;
  let r_0 = loop {
    let s_at_r0 = sample_polynomial(&s_of_w, (int as f32).into());
    if s_at_r0.real() > 0.0 {
      break int;
    }
    int += 1;
  };
  drop(s_of_w);

  {
    let mut guesses: ArrayVec<_, TERMS> = ArrayVec::new();

    let frac_2pi_n = TAU / n as f32;
    let frac_pi_2n = FRAC_PI_2 / n as f32;

    for k in 0..n {
      let theta = frac_2pi_n * k as f32 + frac_pi_2n;
      let r_0 = r_0 as f32;

      let real = a + r_0 * theta.cos();
      let imaginary = r_0 * theta.sin();

      let val = Complex::new(real, imaginary);
      // SAFETY: we push 1 less values than there are terms.
      unsafe { guesses.push_unchecked(val) };
    }

    guesses
  }
}

/// An iterator for a particular row of Pascal's Triangle.
pub struct PascalRowIter {
  n: u32,
  k: u32,
  previous: u32,
}

impl PascalRowIter {
  pub fn new(n: usize) -> Self {
    Self {
      n: n as u32,
      k: 0,
      previous: 1,
    }
  }

  pub fn next(&mut self) -> Option<u32> {
    if self.k == 0 {
      self.k = 1;
      self.previous = 1;
      return Some(1);
    }
    if self.k > self.n {
      return None;
    }
    let new = self.previous * (self.n + 1 - self.k) / self.k;
    self.k += 1;
    self.previous = new;
    return Some(new);
  }
}

impl Iterator for PascalRowIter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    self.next()
  }
}

/// Return the value of the polynomial at some value of `x`.
///
/// Polynomial of the form f(x) = a + b*x + c*x^2 + d*x^3 + ...
/// `coefficients` is a slice containing the coefficients [a, b, c, d, ...]
pub fn sample_polynomial(coefficients: &[f32], x: Complex) -> Complex {
  coefficients
    .iter()
    .enumerate()
    .skip(1)
    .fold(coefficients[0].into(), |acc, (power, coefficient)| {
      acc + coefficient * x.powi(power as i32)
    })
}

/// Compute the derivative of a polynomial.
///
/// Polynomial of the form f(x) = a + b*x + c*x^2 + d*x^3 + ...
/// `coefficients` is a slice containing the coefficients [a, b, c, d, ...]
/// starting from the coefficient of the x with degree 0.
pub fn derivative<const TERMS: usize>(
  coefficients: &[f32; TERMS],
) -> ArrayVec<f32, TERMS> {
  coefficients
    .iter()
    .enumerate()
    .skip(1)
    .map(|(power, coefficient)| coefficient * (power as f32))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use float_cmp::assert_approx_eq;
  const EPSILON: f32 = 0.00005;

  #[test]
  fn derivative() {
    use super::derivative;

    {
      let y = [0.0, 1.0, 2.0, 3.0, 4.0];
      let dydx = derivative(&y);

      let expected = [1.0, 4.0, 9.0, 16.0];
      assert_approx_eq!(&[f32], &dydx, &expected);
    }

    {
      let y = [19.0, 2.3, 0.0, 8.3, 69.420];
      let dydx = derivative(&y);

      let expected = [2.3, 0.0, 24.9, 277.68];
      assert_approx_eq!(&[f32], &dydx, &expected);
    }
  }

  #[test]
  fn sample_polynomial() {
    use super::sample_polynomial;

    {
      let y = [0.0, 1.0, 2.0, 3.0, 4.0];

      let x_0 = 0.0.into();
      let y_0 = sample_polynomial(&y, x_0);
      let expected_0 = 0.0.into();
      assert!(y_0.approx_eq(expected_0, EPSILON));

      let x_1 = 1.0.into();
      let y_1 = sample_polynomial(&y, x_1);
      let expected_1 = 10.0.into();
      assert!(y_1.approx_eq(expected_1, EPSILON));

      let x_2 = (-1.0).into();
      let y_2 = sample_polynomial(&y, x_2);
      let expected_2 = 2.0.into();
      assert!(y_2.approx_eq(expected_2, EPSILON));

      let x_3 = 2.5.into();
      let y_3 = sample_polynomial(&y, x_3);
      let expected_3 = 218.125.into();
      assert!(y_3.approx_eq(expected_3, EPSILON));
    }

    {
      let y = [19.0, 2.3, 0.0, 8.3, 69.420];

      let x_0 = 0.0.into();
      let y_0 = sample_polynomial(&y, x_0);
      let expected_0 = 19.0.into();
      assert!(y_0.approx_eq(expected_0, EPSILON));

      let x_1 = 1.0.into();
      let y_1 = sample_polynomial(&y, x_1);
      let expected_1 = 99.02.into();
      assert!(y_1.approx_eq(expected_1, EPSILON));

      let x_2 = (-1.0).into();
      let y_2 = sample_polynomial(&y, x_2);
      let expected_2 = 77.82.into();
      assert!(y_2.approx_eq(expected_2, EPSILON));
    }
  }

  #[test]
  fn aberth() {
    use super::aberth;

    fn unsorted_compare(zs: &[Complex], ws: &[Complex], epsilon: f32) -> bool {
      zs.iter().fold(true, |acc, &z| {
        let w = ws.iter().find(|&&w| z.approx_eq(w, epsilon));
        acc && w.is_some()
      })
    }

    {
      let polynomial = [0.0, 1.0];
      let roots = aberth(&polynomial, EPSILON);
      assert!(roots[0].approx_eq(Complex::ZERO, EPSILON));
    }

    {
      let polynomial = [1.0, 0.0, -1.0];
      let roots = aberth(&polynomial, EPSILON);
      let expected = [1.0.into(), (-1.0).into()];
      assert!(unsorted_compare(&roots, &expected, EPSILON));
    }

    {
      // x^3 -12x^2 + 39x - 28 = 0
      let polynomial = [-28.0, 39.0, -12.0, 1.0];

      let roots = aberth(&polynomial, EPSILON);
      let expected = [7.0.into(), 4.0.into(), 1.0.into()];
      assert!(unsorted_compare(&roots, &expected, EPSILON));
    }
    {
      // 2x^3 - 38x^2 + 228x - 432 = 0
      let polynomial = [-432.0, 228.0, -38.0, 2.0];

      let roots = aberth(&polynomial, EPSILON);
      let expected = [9.0.into(), 6.0.into(), 4.0.into()];
      assert!(unsorted_compare(&roots, &expected, EPSILON));
    }
    {
      // x^3 + 8 = 0
      let polynomial = [8.0, 0.0, 0.0, 1.0];

      let roots = aberth(&polynomial, EPSILON);
      let expected = [
        (-2.0).into(),
        Complex::new(1.0, -3f32.sqrt()),
        Complex::new(1.0, 3f32.sqrt()),
      ];
      assert!(unsorted_compare(&roots, &expected, EPSILON));
    }
    {
      // 11x^9 + 4x^4 + 2x - 1 = 0
      let polynomial = [-1.0, 2.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 11.0];

      let roots = aberth(&polynomial, EPSILON);
      let expected = [
        (0.429326).into(),
        Complex::new(-0.802811, -0.229634),
        Complex::new(-0.802811, 0.229634),
        Complex::new(-0.344895, -0.842594),
        Complex::new(-0.344895, 0.842594),
        Complex::new(0.206720, -0.675070),
        Complex::new(0.206720, 0.675070),
        Complex::new(0.726324, -0.455503),
        Complex::new(0.726324, 0.455503),
      ];
      assert!(unsorted_compare(&roots, &expected, EPSILON));
    }
    {
      // 0 = - 20x^19 + 19x^18 - 18x^17 + 17x^16 - 16x^15
      //     + 15x^14 - 14x^13 + 13x^12 - 12x^11 + 11x^10
      //     - 10x^9  +  9x^8  -  8x^7  +  7x^6  -  6x^5
      //     +  5x^4  -  4x^3  +  3x^2  -  2x    +  1
      let polynomial = [
        1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0, 9.0, -10.0, 11.0, -12.0,
        13.0, -14.0, 15.0, -16.0, 17.0, -18.0, 19.0, -20.0,
      ];

      let roots = aberth(&polynomial, EPSILON);
      // found using wolfram alpha
      let expected = [
        0.834053.into(),
        Complex::new(-0.844061, -0.321794),
        Complex::new(-0.844061, 0.321794),
        Complex::new(-0.684734, -0.550992),
        Complex::new(-0.684734, 0.550992),
        Complex::new(-0.476151, -0.721437),
        Complex::new(-0.476151, 0.721437),
        Complex::new(-0.231844, -0.822470),
        Complex::new(-0.231844, 0.822470),
        Complex::new(0.028207, -0.846944),
        Complex::new(0.028207, 0.846944),
        Complex::new(0.281692, -0.793720),
        Complex::new(0.281692, 0.793720),
        Complex::new(0.506511, -0.668231),
        Complex::new(0.506511, 0.668231),
        Complex::new(0.682933, -0.482160),
        Complex::new(0.682933, 0.482160),
        Complex::new(0.795421, -0.252482),
        Complex::new(0.795421, 0.252482),
      ];
      assert!(unsorted_compare(&roots, &expected, EPSILON));
    }
  }

  #[test]
  fn pascal_triangle() {
    {
      let row: Vec<_> = PascalRowIter::new(0).collect();
      let expected = vec![1];
      assert_eq!(row, expected);
    }
    {
      let row: Vec<_> = PascalRowIter::new(1).collect();
      let expected = vec![1, 1];
      assert_eq!(row, expected);
    }
    {
      let row: Vec<_> = PascalRowIter::new(2).collect();
      let expected = vec![1, 2, 1];
      assert_eq!(row, expected);
    }
    {
      let row: Vec<_> = PascalRowIter::new(3).collect();
      let expected = vec![1, 3, 3, 1];
      assert_eq!(row, expected);
    }
    {
      let row: Vec<_> = PascalRowIter::new(4).collect();
      let expected = vec![1, 4, 6, 4, 1];
      assert_eq!(row, expected);
    }
    {
      let row: Vec<_> = PascalRowIter::new(5).collect();
      let expected = vec![1, 5, 10, 10, 5, 1];
      assert_eq!(row, expected);
    }
    {
      let row: Vec<_> = PascalRowIter::new(6).collect();
      let expected = vec![1, 6, 15, 20, 15, 6, 1];
      assert_eq!(row, expected);
    }
  }
}
