#![allow(unused)]

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
  pub x: f32,
  pub y: f32,
}

impl Point {
  pub const ORIGIN: Self = Self {x: 0.0, y: 0.0};

  pub fn vector_to(self, end: Point) -> Vector {
    Vector::from_points(self, end)
  }
}

impl From<(f32, f32)> for Point {
  fn from(value: (f32, f32)) -> Self {
    Point {x: value.0, y: value.1}
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
  pub const ZERO: Self = Self {x: 0.0, y: 0.0};

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

#[derive(Debug, PartialEq, Clone)]
pub enum Segment {
  Line {
    start: Point,
    end: Point,
  },
  QuadBezier {
    start: Point,
    control: Point,
    end: Point,
  },
  CubicBezier {
    start: Point,
    control_1: Point,
    control_2: Point,
    end: Point,
  },
}

pub use Segment::{ Line, QuadBezier, CubicBezier }; //TODO

#[derive(PartialEq)]
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

impl std::ops::BitAnd<u8> for Channels {
  type Output = Self;

  fn bitand(self, rhs: u8) -> Self {
    Self { inner: self.inner & rhs }
  }
}

impl std::ops::BitAnd for Channels {
  type Output = Self;

  fn bitand(self, rhs: Self) -> Self {
    Self { inner: self.inner & rhs.inner }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CornerRays {
  start: Vector,
  end: Vector,
}

impl From<(Vector, Vector)> for CornerRays {
  fn from(vectors: (Vector, Vector)) -> Self {
    Self {
      start: vectors.0,
      end: vectors.1,
    }
  }
}

impl From<((f32, f32), (f32, f32))> for CornerRays {
  fn from(vectors: ((f32, f32), (f32, f32))) -> Self {
    Self {
      start: vectors.0.into(),
      end: vectors.1.into(),
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct Spline<'a> {
  segments: &'a [Segment],
  channels: Channels,
  corner_rays: &'a CornerRays,
}

// TODO
impl Spline<'_> {
  fn distance_to(&self, position: Point) -> f32 {
    for segment in self.segments.iter() {
      match segment {
        line @ Line{..} => unimplemented!(),
        quad @ QuadBezier{..} => unimplemented!(),
        cubic @ CubicBezier{..} => unimplemented!(),
      }
    }
    0.0
  }
  fn pseudo_distance_to(&self, position: Point) -> f32 {
    0.0
  }
}

pub struct Contour {
  segments: Vec<Segment>,
  corners: Memo<Vec<usize>>,
  corner_rays: Memo<Vec<CornerRays>>,
  channels: Memo<Vec<Channels>>,
  // TODO:
  // Kind: enum{FullySmooth, TearDrop, General}
  // remove corners, add spline_indices
}

impl Contour {
  /// Iterator over splines in the Contour. If the shape is fully smooth then returns an empty
  /// iterator.
  /// Note: Assumes that if there are any corners, then at least one of the corners must be at
  /// segment[0].
  pub fn splines(&self) -> impl Iterator<Item = Spline> {
    use itertools::{ Itertools, izip };

    izip!(
      std::iter::once(0)
        .chain(self.corners.unwrap().iter().copied())
        .chain(std::iter::once(self.segments.len()))
        .dedup()
        .tuple_windows(),
      self.channels.unwrap().iter().copied(),
      self.corner_rays.unwrap().iter(),
    )
      .map(|((index, index_1), channels, corner_rays)| {
        Spline {
          segments: &self.segments[index..index_1],
          channels,
          corner_rays,
        }
      })
  }
}
// TODO: generate channels, rays, corners
// remove Memo. Use ShapeBuilder that returns a well-formed Shape on `.finalise()`.

pub struct Shape {
  contours: Vec<Contour>,
}

impl Shape {
  pub fn sample(&self, position: Point) -> f32 {
    let mut min_dist = f32::INFINITY;
    let mut closest_spline = None;
    for contour in self.contours.iter() {
      for spline in contour.splines() {
        let dist = spline.distance_to(position);
        if dist < min_dist
        && (spline.channels & 0b100).as_bool() {
          min_dist = dist;
          closest_spline = Some(spline);
        }
      }
    }
    closest_spline
      .expect(&format!("Couldn't find closest spline for position: {position:?}"))
      .pseudo_distance_to(position)
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

  #[test]
  fn contour_splines_empty() {
    let contour = Contour {
      segments: vec![],
      corners: Memo::Value(vec![]),
      channels: Memo::Value(vec![]),
      corner_rays: Memo::Value(vec![]),
    };
    let splines = contour.splines().collect::<Vec<_>>();
    let expected: Vec<Spline> = vec![];
    assert_eq!(splines, expected);
  }

  #[test]
  fn contour_splines() {
    let point_a = Point {x: 0.0, y: 0.0};
    let point_b = Point {x: 1.0, y: 0.0};
    let point_c = Point {x: 0.5, y: 1.0};

    let vec_ab = point_a.vector_to(point_b);
    let vec_bc = point_b.vector_to(point_c);
    let vec_ca = point_c.vector_to(point_a);

    let ray_a = (vec_ca.norm() + -vec_ab.norm()).norm();
    let ray_b = (vec_ab.norm() + -vec_bc.norm()).norm();
    let ray_c = (vec_bc.norm() + -vec_ca.norm()).norm();

    let line_ab = Line {start: point_a, end: point_b};
    let line_bc = Line {start: point_b, end: point_c};
    let line_ca = Line {start: point_c, end: point_a};

    let channels_ab: Channels = 0b101.into();
    let channels_bc: Channels = 0b110.into();
    let channels_ca: Channels = 0b011.into();

    let corner_rays_ab = CornerRays {start: ray_a, end: ray_b};
    let corner_rays_bc = CornerRays {start: ray_b, end: ray_c};
    let corner_rays_ca = CornerRays {start: ray_c, end: ray_a};

    let contour = Contour {
      segments: vec![line_ab.clone(), line_bc.clone(), line_ca.clone()],
      corners: Memo::Value(vec![0, 1, 2]),
      corner_rays: Memo::Value(vec![
        corner_rays_ab.clone(), corner_rays_bc.clone(), corner_rays_ca.clone()
      ]),
      channels: Memo::Value(vec![channels_ab, channels_bc, channels_ca]),
    };

    let slice_ab = &[line_ab][..];
    let slice_bc = &[line_bc][..];
    let slice_ca = &[line_ca][..];

    let splines = contour.splines().collect::<Vec<_>>();
    let expected: Vec<Spline> = vec![
      Spline {
        segments: slice_ab,
        channels: channels_ab,
        corner_rays: &corner_rays_ab,
      },
      Spline {
        segments: slice_bc,
        channels: channels_bc,
        corner_rays: &corner_rays_bc,
      },
      Spline {
        segments: slice_ca,
        channels: channels_ca,
        corner_rays: &corner_rays_ca,
      },
    ];

    assert_eq!(splines, expected);
  }

  #[test]
  fn contour_splines_fully_smooth() {
    // No corners
    let point_a = Point {x: 0.0, y: 0.0};
    let point_b = Point {x: 1.0, y: 0.0};
    let point_c = Point {x: 0.5, y: 1.0};

    let vec_ab = Vector::from_points(point_a, point_b);
    let vec_ca = Vector::from_points(point_c, point_a);

    let ray_a = (vec_ca.norm() + -vec_ab.norm()).norm();

    let line_ab = Line {start: point_a, end: point_b};
    let line_bc = Line {start: point_b, end: point_c};
    let line_ca = Line {start: point_c, end: point_a};

    let channels: Channels = 0b111.into();
    let corner_rays = CornerRays {start: ray_a, end: ray_a};
    let slice = &[line_ab.clone(), line_bc.clone(), line_ca.clone()][..];

    let contour = Contour {
      segments: vec![line_ab, line_bc, line_ca],
      corners: Memo::Value(vec![]),
      corner_rays: Memo::Value(vec![corner_rays.clone()]),
      channels: Memo::Value(vec![channels]),
    };

    let splines = contour.splines().collect::<Vec<_>>();
    let expected: Vec<Spline> = vec![
      Spline {
        segments: slice,
        channels,
        corner_rays: &corner_rays,
      }
    ];

    assert_eq!(splines, expected);
  }

  #[test]
  fn channels_as_bool() {
    assert!(Channels{inner: 0b100}.as_bool());
    assert!(!Channels{inner: 0b000}.as_bool());
  }

  #[test]
  fn channels_bitand_u8() {
    assert_eq!(Channels::from(0b001), Channels::from(0b101)&0b001);
    assert_eq!(Channels::from(0b000), Channels::from(0b100)&0b001);
  }

  #[test]
  fn channels_bitand() {
    assert_eq!(Channels::from(0b101), Channels::from(0b111)&Channels::from(0b101));
    assert_eq!(Channels::from(0b000), Channels::from(0b010)&Channels::from(0b101));
  }
}
