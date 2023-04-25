use crate::*;

/// A measure of the "orthogonality" of two vectors.
///
/// Since the area of the parallelogram defined by two normalised vectors is at
/// a maximum (1) when they are orthogonal and a minimum (0) when they are
/// parallel, we use this area as the measure.
#[inline]
fn orthogonality(a: Vector, b: Vector) -> f32 {
  Vector::area(a.norm(), b.norm())
}
