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
  pub fn vector_to(self, end: Point) -> Vector {
    Vector::from_points(self, end)
  }

  #[inline]
  pub fn as_vector(self) -> Vector {
    Vector {
      x: self.x,
      y: self.y,
    }
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vector_to() {
    let a = Point { x: 1.0, y: 2.0 };
    let b = Point { x: 5.5, y: 1.5 };

    assert_eq!(Vector { x: 4.5, y: -0.5 }, a.vector_to(b));
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
