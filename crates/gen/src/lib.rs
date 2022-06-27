#![allow(unused)]

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

impl Segment {
  // TODO: impl quad & cubic, unit test
  // TODO: break `t` into own method. store t and if first or last segment in
  // spline, call appropriate ray methods.

  // TODO: unit test
  fn closest_param_t(&self, point: Point) -> f32 {
    match self {
      &Line{start, end} => {
        Vector::from_points(start, point).dot(Vector::from_points(start, end))
        / Vector::from_points(start, end).dot(Vector::from_points(start, end))
      },
      _ => unimplemented!(),
    }
  }

  // TODO: unit test
  fn distance_to_point_from_t(&self, point: Point, t: f32) -> f32 {
    match self {
      &Line{start, end} => {
        Vector::from_points(
          start + (t * Vector::from_points(start, end)),
          point
        ).abs()
      },

      &QuadBezier{..} => unimplemented!(),
      &CubicBezier{..} => unimplemented!(),
    }
  }

  fn distance_to(&self, point: Point) -> f32 {
    self.distance_to_point_from_t(point, self.closest_param_t(point).clamp(0.0, 1.0))
  }

  // TODO: unit test
  fn pseudo_distance_to(&self, point: Point) -> f32 {
    // this is true for Line, maybe not for others
    self.distance_to_point_from_t(point, self.closest_param_t(point))
  }

  fn inside_ray_start(&self, ray: Vector, point: Point) -> bool {
    match self {
      &Line{start, ..} => {
        Vector::from_points(start, point).signed_area(ray) >= 0.0
      },
      _ => unimplemented!(),
    }
  }

  fn inside_ray_end(&self, ray: Vector, point: Point) -> bool {
    match self {
      &Line{end, ..} => {
        Vector::from_points(end, point).signed_area(ray) <= 0.0
      },
      _ => unimplemented!(),
    }
  }
}

pub use Segment::{Line, QuadBezier, CubicBezier }; //TODO

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

#[derive(Debug, PartialEq, Clone)]
pub struct Spline<'a> {
  segments: &'a [Segment],
  channels: Channels,
  corner_rays: &'a CornerRays,
}

// TODO
impl Spline<'_> {
  fn distance_to(&self, position: Point) -> f32 {
    let mut selected_dist = f32::INFINITY;
    let mut selected_segment = None;
    let mut selected_t = None; // simple optimisation

    for (s, segment) in self.segments.iter().enumerate() {
      let t = segment.closest_param_t(position);
      let dist = segment.distance_to_point_from_t(position, t.clamp(0.0, 1.0));
      debug_assert!(dist >= 0.0, "dist must be an absolute value, but was found to be {dist}");
      if dist < selected_dist {
        selected_dist = dist;
        selected_segment = Some(s);
        selected_t = Some(t);
      }
    }

    if match selected_segment {
      Some(0) => {
        selected_t.unwrap() < 0.0
        && self.segments[selected_segment.unwrap()].inside_ray_start(self.corner_rays.start, position)
      },
      Some(x) if x == self.segments.len() => {
        selected_t.unwrap() > 1.0
        && self.segments[selected_segment.unwrap()].inside_ray_end(self.corner_rays.end, position)
      },
      _ => true,
    } {
      selected_dist
    } else {
      f32::INFINITY
    }
  }

  // TODO: unit test
  fn signed_pseudo_distance_to(&self, position: Point) -> f32 {
    let mut selected_dist = f32::INFINITY;
    let mut selected_segment = None;
    let mut selected_t = None;

    for (s, segment) in self.segments.iter().enumerate() {
      let t = segment.closest_param_t(position);
      let dist = segment.distance_to_point_from_t(position, t.clamp(0.0, 1.0));
      debug_assert!(dist >= 0.0, "dist must be an absolute value, but was found to be {dist}");
      if dist < selected_dist {
        selected_dist = dist;
        selected_segment = Some(s);
        selected_t = Some(t);
      }
    }

    match selected_segment {
      Some(x) if x == self.segments.len() || x == 0 => {
        self.segments[selected_segment.unwrap()].pseudo_distance_to(position)
      },
      _ => selected_dist
    }
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
    // let mut min_dist = f32::INFINITY;
    // let mut closest_spline = None;
    let mut closest_spline = Some(self.contours.iter().next().unwrap().splines().next().unwrap());
    let mut min_dist = closest_spline.clone().unwrap().distance_to(position);
    for contour in self.contours.iter() {
      for spline in contour.splines() {
        eprintln!("spline: {spline:?}");
        let dist = spline.distance_to(position);
        eprintln!("dist: {dist}");
        if dist < min_dist
        // && (spline.channels & 0b100).as_bool() {
          {
          min_dist = dist;
          closest_spline = Some(spline);
        }
      }
    }
    match closest_spline {
      // Some(spline) => spline.signed_pseudo_distance_to(position),
      Some(spline) => spline.distance_to(position),
      _ => f32::INFINITY,
    }
      // .expect(&format!("Couldn't find closest spline for position: {position:?}"))
      // .signed_pseudo_distance_to(position)
  }
}

// TODO: cleanup, unit test
use png;
use std::io::BufWriter;
use std::fs::File;

pub struct Image<'a> {
  encoder: png::Encoder<'a, BufWriter<File>>,
  data: Vec<u8>,
  width: usize,
  height: usize,
}

impl Image<'_> {
  fn new(path: &str, size: [usize; 2]) -> Self {
    let file = File::create(path).unwrap();
    let buf_writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(buf_writer, size[0] as u32, size[1] as u32);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let data_length = size[0] * size[1] * 3;
    let mut data = Vec::with_capacity(data_length);
    // initialise data
    data.extend(std::iter::repeat(0).take(data_length));

    Self {
      data,
      encoder,
      width: size[0],
      height: size[1],
    }
  }

  #[inline]
  fn set_pixel(&mut self, coords: [usize; 2], val: [u8; 3]) {
    debug_assert!(
      coords[0] < self.width && coords[1] < self.height,
      "coordinates given were outside the dimensions of the image"
    );
    let location = (coords[1] * self.width + coords[0]) * 3;
    self.data[location] = val[0];
    self.data[location + 1] = val[1];
    self.data[location + 2] = val[2];
  }

  fn flush(self) {
    let mut writer = self.encoder.write_header().unwrap();
    writer.write_image_data(&self.data).unwrap();
  }
}

// distanceColor
// TODO: cleanup, unit test
const MAX_DISTANCE: f32 = 10.0;
const MAX_COLOUR: f32 = 256.0;
#[inline]
fn distance_color(distance: f32) -> u8 {
  let distance = distance.clamp(-MAX_DISTANCE, MAX_DISTANCE);
  (( (distance+MAX_DISTANCE) / (2.0*MAX_DISTANCE) * MAX_COLOUR) - 1.0) as u8
}

pub fn do_thing() {
  println!("doing_thing");

  let point_a = Point {x: 1.0, y: 1.0};
  let point_b = Point {x: 9.0, y: 1.0};
  let point_c = Point {x: 5.0, y: 9.0};

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

  let shape = Shape {
    contours: vec![contour],
  };

  let mut image = Image::new("test_image.png", [10, 10]);

  for y in 0..image.height {
    for x in 0..image.width {
      let point: Point = (x as f32, y as f32).into();
      let val = shape.sample(point);
      let val = distance_color(val);
      image.set_pixel([x, y], [val, 0, 0]);
    }
  }

  image.flush();

  panic!()
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

  #[test]
  fn segment_line_distance_to() {
    let line = Line {
      start: (0.0, 0.0).into(),
      end: (2.0, 4.0).into(),
    };

    assert_eq!(0.0, line.distance_to((0.0, 0.0).into()));
    assert_eq!(0.0, line.distance_to((0.5, 1.0).into()));
    assert_eq!(0.0, line.distance_to((1.0, 2.0).into()));
    assert_eq!(0.0, line.distance_to((2.0, 4.0).into()));
    assert_eq!(5.0f32.sqrt(), line.distance_to((3.0, 1.0).into()));
    assert_eq!(5.0f32.sqrt(), line.distance_to((-1.0, 3.0).into()));
    assert_eq!(5.0f32.sqrt(), line.distance_to((-1.0, -2.0).into()));
    assert_eq!(5.0f32.sqrt(), line.distance_to((3.0, 6.0).into()));
    assert_eq!(1.0, line.distance_to((0.0, -1.0).into()));
    assert_eq!(2.0, line.distance_to((-2.0, 0.0).into()));
  }

  #[test]
  fn segment_line_inside_ray_start() {
    let line = Line {
      start: (0.0, 0.0).into(),
      end: (10.0, 0.0).into(),
    };

    let ray = Vector::from((-1.0, 1.0)).norm();
    assert!(line.inside_ray_start(ray, (0.0, 1.0).into()));
    assert!(line.inside_ray_start(ray, (-1.0, 1.0).into()));
    assert!(!line.inside_ray_start(ray, (-1.0, 0.0).into()));
    assert!(!line.inside_ray_start(ray, (-1.01, 1.0).into()));
  }

  #[test]
  fn segment_line_inside_ray_end() {
    let line = Line {
      start: (0.0, 0.0).into(),
      end: (10.0, 0.0).into(),
    };

    let ray = Vector::from((2.0, 1.0)).norm();
    assert!(line.inside_ray_end(ray, (10.0, 1.0).into()));
    assert!(line.inside_ray_end(ray, (12.0, 1.0).into()));
    assert!(!line.inside_ray_end(ray, (12.01, 1.0).into()));
    assert!(!line.inside_ray_end(ray, (12.00, 0.0).into()));
  }

  #[test]
  fn spline_distance_to() {
    let line = Line {
      start: (0.0, 0.0).into(),
      end: (10.0, 0.0).into(),
    };

    let ray = Vector::from((2.0, 1.0)).norm();
    assert!(line.inside_ray_end(ray, (10.0, 1.0).into()));
    assert!(line.inside_ray_end(ray, (12.0, 1.0).into()));
    assert!(!line.inside_ray_end(ray, (12.01, 1.0).into()));
    assert!(!line.inside_ray_end(ray, (12.00, 0.0).into()));
  }

  #[test]
  fn do_thingy() {
    do_thing()
  }
}
