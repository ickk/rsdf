use std::ops::Sub;

use super::*;

/// A point in 2D space
#[derive(Copy, Clone, PartialEq)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}

impl std::fmt::Debug for Point {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    formatter
      .debug_tuple("Point")
      .field(&self.x)
      .field(&self.y)
      .finish()
  }
}

impl Point {
  /// The origin
  pub const ZERO: Point = Point { x: 0f32, y: 0f32 };

  /// Create a `Point` from a pair of `x` and `y` coordinates
  #[inline]
  pub const fn new(x: f32, y: f32) -> Self {
    Point { x, y }
  }

  /// Return a `Vector` containing the same components as the point
  #[inline]
  pub fn as_vector(self) -> Vector {
    Vector {
      x: self.x,
      y: self.y,
    }
  }
}

impl Sub<Point> for Point {
  type Output = Vector;

  #[inline]
  fn sub(self, rhs: Point) -> Vector {
    Vector {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
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
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl std::ops::Sub<Vector> for Point {
  type Output = Point;

  #[inline]
  fn sub(self, rhs: Vector) -> Point {
    Point {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

impl float_cmp::ApproxEq for Point {
  type Margin = float_cmp::F32Margin;

  fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
    let margin = margin.into();
    self.x.approx_eq(other.x, margin) && self.y.approx_eq(other.y, margin)
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
