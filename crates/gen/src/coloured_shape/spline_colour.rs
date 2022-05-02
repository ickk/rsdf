#[derive(Copy, Clone)]
pub struct SplineColour {
  inner: u8,
}

pub const WHITE: SplineColour = SplineColour::from_colour(Colour::White);
pub const MAGENTA: SplineColour = SplineColour::from_colour(Colour::Magenta);
pub const YELLOW: SplineColour = SplineColour::from_colour(Colour::Yellow);

impl SplineColour {
  #[rustfmt::skip]
  pub const fn from_colour(colour: Colour) -> Self {
    match colour {
      Colour::Red     => Self { inner: 0b100 },
      Colour::Yellow  => Self { inner: 0b110 },
      Colour::Green   => Self { inner: 0b010 },
      Colour::Cyan    => Self { inner: 0b011 },
      Colour::Blue    => Self { inner: 0b001 },
      Colour::Magenta => Self { inner: 0b101 },
      Colour::White   => Self { inner: 0b111 },
      Colour::Black   => Self { inner: 0b000 },
    }
  }

  pub const fn colour(&self) -> Colour {
    match self.inner {
      0b100 => Colour::Red,
      0b110 => Colour::Yellow,
      0b010 => Colour::Green,
      0b011 => Colour::Cyan,
      0b001 => Colour::Blue,
      0b101 => Colour::Magenta,
      0b111 => Colour::White,
      0b000 => Colour::Black,
      _ => unreachable!(), // Self can only be created from a Colour & the inner field is private.
    }
  }
}

impl std::ops::BitXorAssign for SplineColour {
  #[inline]
  fn bitxor_assign(&mut self, rhs: Self) {
    self.inner ^= rhs.inner
  }
}

use std::fmt::{self, Debug};
impl Debug for SplineColour {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(&format!("{:?}", self.colour()))
  }
}

#[derive(Debug)]
pub enum Colour {
  Red,
  Yellow,
  Green,
  Cyan,
  Blue,
  Magenta,
  White,
  Black,
}
