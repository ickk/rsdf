use num_derive::FromPrimitive;

/// Basic type supporting bitwise binary operations on colour channels
#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Colour {
  Black   = 0b000,
  Red     = 0b001,
  Green   = 0b010,
  Blue    = 0b100,
  Yellow  = 0b011,
  Cyan    = 0b110,
  Magenta = 0b101,
  White   = 0b111,
}

impl std::ops::BitAnd for Colour {
  type Output = Self;

  #[inline]
  fn bitand(self, rhs: Colour) -> Colour {
    unsafe {
      num_traits::FromPrimitive::from_u8(self as u8 & rhs as u8)
        .unwrap_unchecked()
    }
  }
}

impl std::ops::BitOr for Colour {
  type Output = Self;

  #[inline]
  fn bitor(self, rhs: Colour) -> Colour {
    unsafe {
      num_traits::FromPrimitive::from_u8(self as u8 | rhs as u8)
        .unwrap_unchecked()
    }
  }
}

impl std::ops::BitXor for Colour {
  type Output = Self;

  #[inline]
  fn bitxor(self, rhs: Colour) -> Colour {
    unsafe {
      num_traits::FromPrimitive::from_u8(self as u8 ^ rhs as u8)
        .unwrap_unchecked()
    }
  }
}

impl std::ops::Not for Colour {
  type Output = Self;

  #[inline]
  fn not(self) -> Colour {
    unsafe {
      num_traits::FromPrimitive::from_u8(!(self as u8) & 0b111)
        .unwrap_unchecked()
    }
  }
}

#[cfg(any(test, doctest))]
mod tests {
  use super::Colour::*;

  #[test]
  fn bitand() {
    assert_eq!(Red, Red & Red);
    assert_eq!(Black, Red & Green);
    assert_eq!(Red, Red & Yellow);
    assert_eq!(Black, White & Black);
    assert_eq!(Magenta, White & Magenta);
    assert_eq!(Blue, Blue & Cyan);
    assert_eq!(Blue, Cyan & Magenta);
  }

  #[test]
  fn bitor() {
    assert_eq!(Red, Red | Red);
    assert_eq!(Yellow, Red | Green);
    assert_eq!(Yellow, Red | Yellow);
    assert_eq!(White, White | Black);
    assert_eq!(White, White | Magenta);
    assert_eq!(Cyan, Blue | Cyan);
    assert_eq!(White, Cyan | Magenta);
  }

  #[test]
  fn bitxor() {
    assert_eq!(Black, Red ^ Red);
    assert_eq!(Yellow, Red ^ Green);
    assert_eq!(Green, Red ^ Yellow);
    assert_eq!(White, White ^ Black);
    assert_eq!(Green, White ^ Magenta);
    assert_eq!(Green, Blue ^ Cyan);
    assert_eq!(Yellow, Cyan ^ Magenta);
  }

  #[test]
  fn not() {
    assert_eq!(Cyan, !Red);
    assert_eq!(Magenta, !Green);
    assert_eq!(Yellow, !Blue);
    assert_eq!(Red, !Cyan);
    assert_eq!(Green, !Magenta);
    assert_eq!(Blue, !Yellow);
    assert_eq!(White, !Black);
    assert_eq!(Black, !White);
  }
}
