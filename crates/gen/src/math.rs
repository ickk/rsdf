#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}

impl Point {
  pub fn vector_to(self, end: Point) -> Vector {
    Vector::from_points(self, end)
  }
}

impl From<(f32, f32)> for Point {
  fn from(value: (f32, f32)) -> Self {
    Point {x: value.0, y: value.1}
  }
}

// TODO: unit test
impl std::ops::Add<Vector> for Point {
  type Output = Self;

  fn add(self, rhs: Vector) -> Self {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

// TODO: unit test
impl std::ops::Sub<Vector> for Point {
  type Output = Self;

  fn sub(self, rhs: Vector) -> Self {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector {
  pub x: f32,
  pub y: f32,
}

impl From<(f32, f32)> for Vector {
  fn from(value: (f32, f32)) -> Self {
    Vector {x: value.0, y: value.1}
  }
}

impl Vector {
  pub fn abs(self) -> f32 {
    (self.x*self.x + self.y*self.y).sqrt()
  }

  pub fn norm(self) -> Self {
    self / self.abs()
  }

  pub fn from_points(start: Point, end: Point) -> Self {
    Self {
      x: end.x - start.x,
      y: end.y - start.y,
    }
  }

  /// The dot product of a pair of vectors.
  // TODO: unit test
  pub fn dot(self, rhs: Vector) -> f32 {
    self.x * rhs.x + self.y * rhs.y
  }

  /// The determinant of the matrix formed by the pair of vectors.
  // TODO: unit test
  pub fn det(a: Vector, b: Vector) -> f32 {
    a.x * b.y - a.y * b.x
  }

  /// An alias for `Vector::det(self, b)`.
  ///
  /// Geometrically this gives the signed area of the parallelogram described
  /// by the pair of vectors.
  ///
  /// If the `b` is counter-clockwise to `self` then the result is
  /// positive, otherwise the result is negative. The area is zero when the
  /// vectors are parallel.
  pub fn signed_area(self, b: Vector) -> f32 {
    Vector::det(self, b)
  }
}

impl std::ops::Div<f32> for Vector {
  type Output = Self;

  fn div(self, rhs: f32) -> Self {
    Self {
      x: self.x / rhs,
      y: self.y / rhs,
    }
  }
}

// TODO: unit test
impl std::ops::Mul<f32> for Vector {
  type Output = Self;

  fn mul(self, rhs: f32) -> Self {
    Self {
      x: self.x * rhs,
      y: self.y * rhs,
    }
  }
}

// TODO: unit test
impl std::ops::Mul<Vector> for f32 {
  type Output = Vector;

  fn mul(self, rhs: Vector) -> Vector {
    Vector {
      x: self * rhs.x,
      y: self * rhs.y,
    }
  }
}


impl std::ops::Add for Vector {
  type Output = Self;

  fn add(self, rhs: Vector) -> Self {
    Self {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    }
  }
}

impl std::ops::Sub for Vector {
  type Output = Self;

  fn sub(self, rhs: Vector) -> Self {
    Self {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    }
  }
}

impl std::ops::Neg for Vector {
  type Output = Self;

  fn neg(self) -> Self {
    Self {
      x: -self.x,
      y: -self.y,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn points_vector_to() {
    let a = Point {x: 1.0, y: 2.0};
    let b = Point {x: 5.5, y: 1.5};

    assert_eq!(Vector{x: 4.5, y: -0.5}, a.vector_to(b));
  }

  #[test]
  fn vector_from_points() {
    let a = Point {x: 1.0, y: 2.0};
    let b = Point {x: 5.5, y: 1.5};

    assert_eq!(Vector{x: 4.5, y: -0.5}, Vector::from_points(a, b));
  }

  #[test]
  fn vector_from_f32s() {
    assert_eq!(Vector{x: 3.2, y: -2.3}, Vector::from((3.2, -2.3)));
  }

  #[test]
  fn vector_add() {
    let a: Vector = (1.0, 2.0).into();
    let b: Vector = (4.0, -3.0).into();

    assert_eq!(Vector::from((5.0, -1.0)), a + b);
  }

  #[test]
  fn vector_sub() {
    let a = Vector {x: 1.0, y: 2.0};
    let b = Vector {x: 4.0, y: -3.0};

    assert_eq!(Vector{x: -3.0, y: 5.0}, a - b);
  }

  fn vector_divf32() {
    let mut v = Vector {x: 1.0, y: 2.0};
    v = v / 2.0;

    assert_eq!(Vector{x: 0.5, y: 1.0}, v);
  }

  #[test]
  fn vector_neg() {
    let v = Vector {x: 1.0, y: 2.0};
    assert_eq!(Vector{x: -1.0, y: -2.0}, -v);
  }

  #[test]
  fn vector_abs() {
    let v = Vector {x: 1.0, y: 0.0};
    assert_eq!(1.0, v.abs());

    let v = Vector {x: 0.0, y: 3.5};
    assert_eq!(3.5, v.abs());

    let v = Vector {x: 1.0, y: 1.0};
    assert_eq!(2.0f32.sqrt(), v.abs());
  }

  #[test]
  fn vector_norm() {
    let v = Vector {x: 53.2, y: 0.0};
    assert_eq!(Vector {x: 1.0, y: 0.0}, v.norm());

    let v = Vector {x: 1.0, y: 1.0};
    assert_eq!(Vector {x: 1.0/2.0f32.sqrt(), y: 1.0/2.0f32.sqrt()}, v.norm());
  }
}
