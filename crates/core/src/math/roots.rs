use arrayvec::ArrayVec;
use std::ops::RangeBounds;

/// The threshold used to decide when a root has been found
pub const EPSILON: f32 = 0.0001;

/// Find real roots in the given range
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
