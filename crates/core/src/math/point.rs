use std::ops::Sub;

use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}

impl Point {
  #[inline]
  pub const fn new(x: f32, y: f32) -> Self {
    Point { x, y }
  }


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
    Vector::from_points(rhs, self)
  }
}

impl From<(f32, f32)> for Point {
  #[inline]
  fn from(value: (f32, f32)) -> Self {
    Point {
      x: value.0,
      y: value.1,
    }
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

#[cfg(any(test, doctest))]
mod tests {
  use super::*;

  #[test]
  fn sub() {
    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 5.5, y: 1.5 };

    assert_eq!(Vector { x: 4.5, y: -0.5 }, b - a);
  }

  #[test]
  fn add_vector() {
    let v = Vector { x: 1.0, y: 3.5 };
    let p = Point { x: 0.0, y: 0.0 };
    assert_eq!(p + v, Point { x: 1.0, y: 3.5 });

    let v = Vector { x: 1.0, y: 3.5 };
    let p = Point { x: 5.0, y: 2.0 };
    assert_eq!(p + v, Point { x: 6.0, y: 5.5 });
  }

  #[test]
  fn sub_vector() {
    let v = Vector { x: 1.0, y: 3.5 };
    let p = Point { x: 0.0, y: 0.0 };
    assert_eq!(p - v, Point { x: -1.0, y: -3.5 });

    let v = Vector { x: 1.0, y: 3.5 };
    let p = Point { x: 5.0, y: 2.0 };
    assert_eq!(p - v, Point { x: 4.0, y: -1.5 });
  }
}