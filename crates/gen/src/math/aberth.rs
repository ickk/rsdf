use super::*;
use arrayvec::ArrayVec;

/// Find all of the roots of a polynomial using Aberth's method.
///
/// Polynomial of the form f(x) = a + b*x + c*x^2 + d*x^3 + ...
/// `coefficients` is a slice containing the coefficients [a, b, c, d, ...]
/// When two successive iterations produce roots with less than `epsilon`
/// delta, the roots are returned.
fn aberth<const TERMS: usize>(
  coefficients: &[f32; TERMS],
  epsilon: f32,
) -> ArrayVec<Complex, TERMS> {
  let dydx = &derivative(coefficients);
  let mut zs = initial_guesses(coefficients);
  let mut new_zs = zs.clone();

  'iteration: for _ in 0..1000 {
    for i in 0..zs.len() {
      let p_of_z = sample_polynomial(coefficients, zs[i]);
      let dydx_of_z = sample_polynomial(dydx, zs[i]);
      let sum = (0..zs.len())
        .filter(|&k| k != i)
        .fold(Complex::ZERO, |acc, k| acc + 1.0 / (zs[i] - zs[k]));

      new_zs[i] = zs[i] + p_of_z / (p_of_z * sum - dydx_of_z);
    }
    std::mem::swap(&mut zs, &mut new_zs);

    for (&z, &new_z) in std::iter::zip(&zs, &new_zs) {
      if !z.approx_eq(new_z, epsilon) {
        continue 'iteration;
      }
    }
    return zs;
  }
  panic!("Failed to converge.");
}

// TODO
fn initial_guesses<const TERMS: usize>(
  coefficients: &[f32; TERMS],
) -> ArrayVec<Complex, TERMS> {
  let mut guesses = ArrayVec::new();
  for i in 1..coefficients.len() {
    guesses.push((i as f32).into())
  }
  guesses
}

/// Return the value of the polynomial at some value of `x`.
///
/// Polynomial of the form f(x) = a + b*x + c*x^2 + d*x^3 + ...
/// `coefficients` is a slice containing the coefficients [a, b, c, d, ...]
fn sample_polynomial(coefficients: &[f32], x: Complex) -> Complex {
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
fn derivative<const TERMS: usize>(
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
  }
}
