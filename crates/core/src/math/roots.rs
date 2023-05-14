use arrayvec::ArrayVec;
use std::ops::RangeBounds;

/// The threshold used to decide when a root has been found
pub const EPSILON: f32 = 0.0001;

/// Find real roots in the given range
///
/// `polynomial`: array of coefficients in ascending order by degree.
/// `range`: filter the roots found to this range.
pub fn roots_in_range<const TERMS: usize, R: RangeBounds<f32>>(
  polynomial: &[f32; TERMS],
  range: R,
) -> ArrayVec<f32, TERMS> {
  aberth::aberth(polynomial, EPSILON)
    .unwrap()
    .iter()
    .filter(|root| root.im.abs() <= EPSILON && range.contains(&root.re))
    .map(|root| root.re)
    .collect()
}

/// Find a zero of a twice differentiable function
///
/// `x` is the initial guess, `f` is the function and `df` & `ddf` are the
/// first and second derivatives.
pub fn halleys_method(
  mut x: f32,
  f: impl Fn(f32) -> f32,
  df: impl Fn(f32) -> f32,
  ddf: impl Fn(f32) -> f32,
) -> f32 {
  for _ in 0..100 {
    let fx = f(x);
    if fx.abs() < 0.001 {
      return x;
    }
    let dfx = df(x);
    let ddfx = ddf(x);
    // let denom = 2. * dfx * dfx - fx * ddfx;
    // if float_cmp::approx_eq!(f32, denom, 0f32) {
    //    dbg!(x, fx, dfx, ddfx, denom);
    // }
    x -= (2. * fx * dfx) / (2. * dfx * dfx - fx * ddfx);
  }
  dbg!(x);
  x
}

#[cfg(any(test, doctest))]
mod tests {
  use super::*;

  #[test]
  fn range() {
    {
      // x^3 -12x^2 + 39x - 28 = 0
      let polynomial = [-28., 39., -12., 1.];
      let range = 0.0..10.0;
      let expected = vec![1.0, 4.0, 7.0];

      let roots = {
        let mut r = roots_in_range(&polynomial, range);
        // order for assert_eq
        r.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // lazy precision fix
        r.iter_mut().for_each(|z| *z = z.round());

        r
      };

      assert_eq!(&roots[..], &expected[..]);
    }
    {
      // x^3 -12x^2 + 39x - 28 = 0
      let polynomial = [-28., 39., -12., 1.];
      let range = 1.0..=4.0;
      let expected = vec![1.0, 4.0];

      let roots = {
        let mut r = roots_in_range(&polynomial, range);
        r.sort_by(|a, b| a.partial_cmp(b).unwrap());
        r.iter_mut().for_each(|z| *z = z.round());
        r
      };

      assert_eq!(&roots[..], &expected[..]);
    }
    {
      // 2x^3 - 38x^2 + 228x - 432 = 0
      let polynomial = [-432., 228., -38., 2.];
      let range = 0.0..8.0;
      let expected = vec![4.0, 6.0];

      let roots = {
        let mut r = roots_in_range(&polynomial, range);
        r.sort_by(|a, b| a.partial_cmp(b).unwrap());
        r.iter_mut().for_each(|z| *z = z.round());
        r
      };

      assert_eq!(&roots[..], &expected[..]);
    }
  }
}
