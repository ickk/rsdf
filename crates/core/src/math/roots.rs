use arrayvec::ArrayVec;
use std::ops::RangeBounds;

pub const EPSILON: f32 = 0.0001;

/// find real roots in the given range
pub fn roots_in_range<const TERMS: usize, R: RangeBounds<f32>>(
  polynomial: &[f32; TERMS],
  range: R,
) -> ArrayVec<f32, TERMS> {
  aberth::aberth(&polynomial, EPSILON)
    .unwrap()
    .iter()
    .filter(|root| root.im.abs() <= EPSILON && range.contains(&root.re))
    .map(|root| root.re)
    .collect()
}
