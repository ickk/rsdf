#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Channels {
  inner: u8,
}

impl Channels {
  #[inline]
  pub fn as_bool(&self) -> bool {
    self.inner != 0
  }
}

impl From<u8> for Channels {
  fn from(value: u8) -> Self {
    Self { inner: value }
  }
}

impl std::ops::BitAnd for Channels {
  type Output = Self;

  fn bitand(self, rhs: Self) -> Self {
    Self {
      inner: self.inner & rhs.inner,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn channels_as_bool() {
    assert!(Channels { inner: 0b100 }.as_bool());
    assert!(!Channels { inner: 0b000 }.as_bool());
  }

  #[test]
  fn channels_bitand_u8_into() {
    assert_eq!(Channels::from(0b001), Channels::from(0b101) & 0b001.into());
    assert_eq!(Channels::from(0b000), Channels::from(0b100) & 0b001.into());
  }

  #[test]
  fn channels_bitand() {
    assert_eq!(
      Channels::from(0b101),
      Channels::from(0b111) & Channels::from(0b101)
    );
    assert_eq!(
      Channels::from(0b000),
      Channels::from(0b010) & Channels::from(0b101)
    );
  }
}
