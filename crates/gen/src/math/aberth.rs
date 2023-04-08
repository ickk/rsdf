use super::*;
use arrayvec::ArrayVec;

// DEGREE = TERMS - 1
// TERMS: NonZeroUsize
// -> [f32; DEGREE]
fn aberth<const TERMS: usize>(
  coefficients: &[f32; TERMS],
) -> ArrayVec<Complex, TERMS> {
  let dydx = derivative(coefficients);

  // initial guesses
  let mut zs = initial_guesses(coefficients);

  for _ in 0..20 {
    // TODO: most of this could be left at iterators.
    // Only `roots` is reused / indexed
    let p_of_zs: ArrayVec<Complex, TERMS> = zs
      .iter()
      .map(|&z| sample_polynomial(coefficients, z))
      .collect();
    let dydx_of_zs: ArrayVec<Complex, TERMS> =
      zs.iter().map(|&z| sample_polynomial(&dydx, z)).collect();

    let sum_terms: ArrayVec<Complex, TERMS> = (0..zs.len())
      .map(|i| {
        (0..zs.len())
          .filter(|&k| k != i)
          .fold(Complex::ZERO, |acc, k| acc + 1.0 / (zs[i] - zs[k]))
      })
      .collect();

    let delta_zs: ArrayVec<Complex, TERMS> = (0..zs.len())
      .map(|i| p_of_zs[i] / (p_of_zs[i] * sum_terms[i] - dydx_of_zs[i]))
      .collect();

    zs.iter_mut()
      .zip(delta_zs)
      .for_each(|(z, delta_z)| *z = *z + delta_z);
  }

  zs
}

// TODO
fn initial_guesses<const TERMS: usize>(
  coefficients: &[f32; TERMS],
) -> ArrayVec<Complex, TERMS> {
  let mut guesses = ArrayVec::new();
  for i in 1..coefficients.len() {
    guesses.push((i as f32).into())
  }
  // for (i, _) in coefficients.iter().skip(1).enumerate() {
  //   guesses.push(c.into());
  // }
  guesses
}

/// Return the value of the polynomial at some value of `x`.
///
/// Polynomial of the form f(x) = a + b*x + c*x^2 + d*x^3 + ...
/// `coefficients` is a slice containing the coefficients [a, b, c, d, ...]
fn sample_polynomial(coefficients: &[f32], x: Complex) -> Complex {
  // can probably SIMD

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
///
// TODO: Generic could be simplified with const-generics
// DEGREE = TERMS - 1
// TERMS: NonZeroUsize
// -> [f32; DEGREE]
// The ArrayVec will have capacity 1 larger than required
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

  fn approx_eq(z: Complex, w: Complex) -> bool {
    // dbg!(z, w);
    (z.real - w.real).abs() < 0.0001
      && (z.imaginary - w.imaginary).abs() < 0.0001
  }

  #[test]
  fn polynomial_derivative() {
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
  fn polynomial_sample() {
    {
      let y = [0.0, 1.0, 2.0, 3.0, 4.0];

      let x_0 = 0.0.into();
      let y_0 = sample_polynomial(&y, x_0);
      let expected_0 = 0.0.into();
      assert!(approx_eq(y_0, expected_0));

      let x_1 = 1.0.into();
      let y_1 = sample_polynomial(&y, x_1);
      let expected_1 = 10.0.into();
      assert!(approx_eq(y_1, expected_1));

      let x_2 = (-1.0).into();
      let y_2 = sample_polynomial(&y, x_2);
      let expected_2 = 2.0.into();
      assert!(approx_eq(y_2, expected_2));

      let x_3 = 2.5.into();
      let y_3 = sample_polynomial(&y, x_3);
      let expected_3 = 218.125.into();
      assert!(approx_eq(y_3, expected_3));
    }

    {
      let y = [19.0, 2.3, 0.0, 8.3, 69.420];

      let x_0 = 0.0.into();
      let y_0 = sample_polynomial(&y, x_0);
      let expected_0 = 19.0.into();
      assert!(approx_eq(y_0, expected_0));

      let x_1 = 1.0.into();
      let y_1 = sample_polynomial(&y, x_1);
      let expected_1 = 99.02.into();
      assert!(approx_eq(y_1, expected_1));

      let x_2 = (-1.0).into();
      let y_2 = sample_polynomial(&y, x_2);
      let expected_2 = 77.82.into();
      assert!(approx_eq(y_2, expected_2));
    }
  }

  #[test]
  fn aberth_test() {
    {
      let polynomial = [0.0, 1.0];

      let roots = aberth(&polynomial);

      // let expected = [0.0];
      dbg!(roots);
    }

    {
      let polynomial = [1.0, 0.0, -1.0];

      let roots = aberth(&polynomial);

      // let expected = [1.0, -1.0];
      dbg!(roots);
    }

    {
      // x^3 -12x^2 + 39x - 28 = 0
      let polynomial = [-28.0, 39.0, -12.0, 1.0];

      let roots = aberth(&polynomial);
      // let expected = [7.0, 4.0, 1.0];

      dbg!(roots);
    }
    {
      // 2x^3 - 38x^2 + 228x - 432 = 0
      let polynomial = [-432.0, 228.0, -38.0, 2.0];

      let roots = aberth(&polynomial);
      // let expected = [9.0, 6.0, 4.0];

      dbg!(roots);
    }
    {
      // x^3 + 8 = 0
      let polynomial = [8.0, 0.0, 0.0, 1.0];

      let roots = aberth(&polynomial);
      // let expected = [-2.0, 1 - j*3.sqrt(), 1 + j*3.sqrt()];

      dbg!(roots);
    }
  }
}
