#![allow(unused)]

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector {
  pub x: f32,
  pub y: f32,
}

impl Vector {
  pub fn from(start: Point, end: Point) -> Self {
    Self {
      x: end.x - start.x,
      y: end.y - start.y,
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

#[derive(Debug, PartialEq)]
pub enum Segment {
  Line(Line),
  QuadBezier(QuadBezier),
  CubicBezier(CubicBezier),
}

// pub use Segment::{Line, QuadBezier, CubicBezier}; //TODO

pub trait SegmentType {}

#[derive(Debug, PartialEq)]
pub struct Line {
  start: Point,
  end: Point,
}

impl SegmentType for Line {}

#[derive(Debug, PartialEq)]
pub struct QuadBezier {
  start: Point,
  control: Point,
  end: Point,
}

impl SegmentType for QuadBezier {}

#[derive(Debug, PartialEq)]
pub struct CubicBezier {
  start: Point,
  control_1: Point,
  control_2: Point,
  end: Point,
}

impl SegmentType for CubicBezier {}

#[derive(PartialEq)]
pub enum Memo<T> {
  Uninitialised,
  Value(T),
}
// TODO: impl Deref for Memo
// TODO: put generator closure into Memo::Uninitialised
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

#[derive(Debug, PartialEq)]
pub struct Spline<'a> {
  segments: &'a [Segment],
}

impl<'a> From<&'a [Segment]> for Spline<'a> {
  fn from(value: &'a [Segment]) -> Self {
    Self { segments: value }
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Channel {
  inner: u8,
}
impl From<u8> for Channel {
  fn from(value: u8) -> Self {
    Self { inner: value }
  }
}

// TODO
impl Spline<'_> {
  fn distance_to(&self, position: Point) -> f32 {
    0.0
  }
  fn pseudo_distance_to(&self, position: Point) -> f32 {
    0.0
  }
}

pub struct Contour {
  segments: Vec<Segment>,
  corners: Memo<Vec<usize>>,
  channels: Memo<Vec<Channel>>,
  // Kind: enum{FullySmooth, TearDrop, General}
  // remove corners, add spline_indices
}

impl Contour {
  /// Iterator over splines in the Contour. If the shape is fully smooth then returns an empty
  /// iterator.
  /// Note: Assumes that if there are any corners, then at least one of the corners must be at
  /// segment[0].
  pub fn splines(&self) -> impl Iterator<Item = (Spline, Channel)> { //Iterator<Item = Spline> {
    use itertools::Itertools;

    let corners = self.corners.unwrap();
    std::iter::once(0)
      .chain(corners.iter().copied())
      .chain(std::iter::once(self.segments.len()))
      .dedup()
      .tuple_windows()
      .map(|(index, index_1)| {
        Spline { segments: &self.segments[index..index_1] }
      })
      .zip(self.channels.unwrap().iter().copied())
  }
}
// TODO: generate channels


pub struct Shape {
  contours: Vec<Contour>,
}

// impl Shape {
//   pub fn sample(&self, position: Point) -> f32 {
//     let mut min_dist = f32::INFINITY;
//     let mut closest_spline = None;
//     for contour in self.contours.iter() {
//       for spline in contour.splines() {
//         let dist = spline.distance_to(position);
//         if dist < min_dist
//         && (spline.channels & 0b100) != 0 {
//           min_dist = dist;
//           closest_spline = Some(spline);
//         }
//       }
//     }
//     closest_spline
//       .expect(&format!("Couldn't find closest spline for position: {position:?}"))
//       .pseudo_distance_to(position)
//   }
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_vector_from_points() {
    let a = Point {x: 1.0, y: 2.0};
    let b = Point {x: 5.5, y: 1.5};

    assert_eq!(Vector{x: 4.5, y: -0.5}, Vector::from(a, b));
  }

  #[test]
  fn add_vectors() {
    let a = Vector {x: 1.0, y: 2.0};
    let b = Vector {x: 4.0, y: -3.0};

    assert_eq!(Vector{x: 5.0, y: -1.0}, a + b);
  }

  #[test]
  fn sub_vectors() {
    let a = Vector {x: 1.0, y: 2.0};
    let b = Vector {x: 4.0, y: -3.0};

    assert_eq!(Vector{x: -3.0, y: 5.0}, a - b);
  }

  #[test]
  #[should_panic]
  fn unwrap_unitialised_memo() {
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

  #[test]
  fn empty_contour_splines() {
    let contour = Contour {
      segments: vec![],
      corners: Memo::Value(vec![]),
      channels: Memo::Value(vec![]),
    };
    let splines = contour.splines().collect::<Vec<_>>();
    let expected: Vec<(Spline, Channel)> = vec![];
    assert_eq!(splines, expected);
  }

  #[test]
  fn contour_splines() {
    let contour = Contour {
      segments: vec![
        Segment::Line((Line {
          start: Point {x: 0.0, y: 0.0},
          end: Point {x: 1.0, y: 0.0},
        })),
        Segment::Line((Line {
          start: Point {x: 1.0, y: 0.0},
          end: Point {x: 0.5, y: 1.0},
        })),
        Segment::Line((Line {
          start: Point {x: 0.5, y: 1.0},
          end: Point {x: 0.0, y: 0.0},
        })),
      ],
      corners: Memo::Value(vec![0, 1, 2]),
      channels: Memo::Value(vec![0b101.into(), 0b110.into(), 0b011.into()]),
    };

    let splines = contour.splines().collect::<Vec<_>>();
    let expected: Vec<(Spline, Channel)> = vec![
      (
        [Segment::Line((Line {
          start: Point {x: 0.0, y: 0.0},
          end: Point {x: 1.0, y: 0.0},
        }))][..].into(),
        0b101.into(),
      ),
      (
        [Segment::Line((Line {
          start: Point {x: 1.0, y: 0.0},
          end: Point {x: 0.5, y: 1.0},
        }))][..].into(),
        0b110.into(),
      ),
      (
        [Segment::Line((Line {
          start: Point {x: 0.5, y: 1.0},
          end: Point {x: 0.0, y: 0.0},
        }))][..].into(),
        0b011.into(),
      ),
    ];

    assert_eq!(splines, expected);
  }

  #[test]
  fn contour_splines_fully_smooth() {
    // No corners
    let contour = Contour {
      segments: vec![
        Segment::Line((Line {
          start: Point {x: 0.0, y: 0.0},
          end: Point {x: 1.0, y: 0.0},
        })),
        Segment::Line((Line {
          start: Point {x: 1.0, y: 0.0},
          end: Point {x: 0.5, y: 1.0},
        })),
        Segment::Line((Line {
          start: Point {x: 0.5, y: 1.0},
          end: Point {x: 0.0, y: 0.0},
        })),
      ],
      corners: Memo::Value(vec![]),
      channels: Memo::Value(vec![0b111.into()]),
    };

    let splines = contour.splines().collect::<Vec<_>>();
    let expected: Vec<(Spline, Channel)> = vec![(
      [
        Segment::Line((Line {
          start: Point {x: 0.0, y: 0.0},
          end: Point {x: 1.0, y: 0.0},
        })),
        Segment::Line((Line {
          start: Point {x: 1.0, y: 0.0},
          end: Point {x: 0.5, y: 1.0},
        })),
        Segment::Line((Line {
          start: Point {x: 0.5, y: 1.0},
          end: Point {x: 0.0, y: 0.0},
        })),
      ][..].into(),
      0b111.into(),
    )];

    assert_eq!(splines, expected);
  }
}
