#![allow(unused)]

use super::Point;

/// The determinant of a 2 by 2 matrix.
/// Computes the oriented area of the parallelogram formed by the pair of vectors that constitute
/// the rows of the matrix.
#[inline]
pub fn det(m: [[f32; 2]; 2]) -> f32 {
  m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

/// The dot product of a pair of 2D vectors.
#[inline]
pub fn dot(a: [f32; 2], b: [f32; 2]) -> f32 {
  a[0] * b[0] + a[1] * b[1]
}

/// The magnitude of a 2D vector.
#[inline]
pub fn mag(a: [f32; 2]) -> f32 {
  (a[0] * a[0] + a[1] * a[1]).sqrt()
}

/// The unit vector in the direction of a 2D vector.
#[inline]
pub fn normalize(a: [f32; 2]) -> [f32; 2] {
  let mag_a = mag(a);
  [a[0] / mag_a, a[1] / mag_a]
}

const CORNER_THRESH: f32 = 0.05; // approx 3 degrees.
/// Compare the vector A->B and B->C to see if there is a sharp corner at Point B.
/// The const `CORNER_THRESH` is a small deflection (in radians) that will be permissible when
/// considering whether the two vectors constitute a "straight" line.
#[inline]
pub fn is_corner(a: Point<f32>, b: Point<f32>, c: Point<f32>) -> bool {
  let ab = [(b.x() - a.x()), (b.y() - a.y())];
  let bc = [(c.x() - b.x()), (c.y() - b.y())];
  // return false if the two vectors are not both almost parallel and in the same direction.
  #[rustfmt::skip]
  return !(
    dot(ab, bc) > 0.0
    && (det([ab, bc]) / (mag(ab) * mag(bc))).abs() <= CORNER_THRESH
  );
}
// Two other methods might be to use just the dot product, or to normalise both vectors, then
// convert them into polar coordinates to check the deflection.
