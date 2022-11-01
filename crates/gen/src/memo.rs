#[derive(PartialEq, Eq)]
pub enum Memo<T> {
  Uninitialised,
  Value(T),
}
// TODO: impl Deref for Memo
// TODO: put generating closure into Memo::Uninitialised
// could be called 'Lazy' instead?

impl<T> Memo<T> {
  pub fn is_uninitialised(&self) -> bool {
    matches!(self, Memo::Uninitialised)
  }
  pub fn is_initialised(&self) -> bool {
    !matches!(self, Memo::Uninitialised)
  }
  pub fn unwrap(&self) -> &T {
    match self {
      Memo::Uninitialised => panic!("Memo was unwrapped while Uninitialised"),
      Memo::Value(value) => value,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn memo_unwrap_unitialised() {
    let memo: Memo<usize> = Memo::Uninitialised;
    memo.unwrap();
  }

  #[test]
  fn memo() {
    let memo: Memo<usize> = Memo::Uninitialised;
    assert!(memo.is_uninitialised());
    assert!(!memo.is_initialised());

    let memo: Memo<usize> = Memo::Value(1);
    assert!(!memo.is_uninitialised());
    assert!(memo.is_initialised());
    assert_eq!(memo.unwrap(), &1);
  }
}
