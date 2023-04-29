use std::ops::Sub;

use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
  pub inner: Vector,
}

impl Point {
  #[inline]
  pub const fn new(x: f32, y: f32) -> Self {
    Point {
      inner: Vector { x, y },
    }
  }

  #[inline]
  pub fn as_vector(self) -> Vector {
    self.inner
  }
}

impl Sub<Point> for Point {
  type Output = Vector;

  #[inline]
  fn sub(self, rhs: Point) -> Vector {
    Vector::from_points(rhs, self)
  }
}

impl From<(f32, f32)> for Point {
  #[inline]
  fn from(value: (f32, f32)) -> Self {
    Point::new(value.0, value.1)
  }
}

impl std::ops::Add<Vector> for Point {
  type Output = Point;

  #[inline]
  fn add(self, rhs: Vector) -> Point {
    Point {
      inner: self.inner + rhs,
    }
  }
}

impl std::ops::Sub<Vector> for Point {
  type Output = Point;

  #[inline]
  fn sub(self, rhs: Vector) -> Point {
    Point {
      inner: self.inner - rhs,
    }
  }
}

#[cfg(any(test, doc_test))]
impl float_cmp::ApproxEq for Point {
  type Margin = float_cmp::F32Margin;

  fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
    let margin = margin.into();
    self.inner.approx_eq(other.inner, margin)
  }
}

#[cfg(any(test, doctest))]
mod tests {
  use super::*;

  #[test]
  fn sub() {
    let a = Point::new(1.0, 2.0);
    let b = Point::new(5.5, 1.5);

    assert_eq!(Vector { x: 4.5, y: -0.5 }, b - a);
  }

  #[test]
  fn add_vector() {
    let v = Vector { x: 1.0, y: 3.5 };
    let p = Point::new(0.0, 0.0);
    assert_eq!(p + v, Point::new(1.0, 3.5));

    let v = Vector { x: 1.0, y: 3.5 };
    let p = Point::new(5.0, 2.0);
    assert_eq!(p + v, Point::new(6.0, 5.5));
  }

  #[test]
  fn sub_vector() {
    let v = Vector { x: 1.0, y: 3.5 };
    let p = Point::new(0.0, 0.0);
    assert_eq!(p - v, Point::new(-1.0, -3.5));

    let v = Vector { x: 1.0, y: 3.5 };
    let p = Point::new(5.0, 2.0);
    assert_eq!(p - v, Point::new(4.0, -1.5));
  }
}
