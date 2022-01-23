mod tests;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T: Copy>([T; 2]);
impl<T: Copy> Point<T> {
  #[inline]
  pub fn x(&self) -> T {
    self.0[0]
  }
  #[inline]
  pub fn y(&self) -> T {
    self.0[1]
  }
}

#[derive(Debug, Clone)]
pub enum EdgeSegment {
  Line,
  Quadratic,
  Cubic,
}

#[derive(Debug)]
pub struct Contour {
  edge_segments: Vec<EdgeSegment>,
  points: Vec<Point<f32>>,
  corners: Vec<(usize, usize)>, // (edge_segments index, points index)
}
impl Contour {
  // TODO: These could be iterators.
  pub fn corners(&self) -> Vec<Point<f32>> {
    self
      .corners
      .iter()
      .map(|&(_edge_index, point_index)| self.points[point_index])
      .collect()
  }
  pub fn splines(&self) -> Vec<(&[EdgeSegment], &[Point<f32>])> {
    match self.corners.len() {
      0 => {
        // The contour is fully smooth.
        vec![(&self.edge_segments[..], &self.points[..])]
      },
      _ => {
        // Split contour up on each corner.
        let mut corners = self.corners.iter();
        let first = corners.next().unwrap();
        let mut acc = corners.fold((first.0, first.1, vec![]), |mut acc, corner| {
          acc.2.push((
            &self.edge_segments[acc.0..corner.0],
            &self.points[acc.1..=corner.1],
          ));
          acc.0 = corner.0;
          acc.1 = corner.1;
          acc
        });
        acc
          .2
          .push((&self.edge_segments[acc.0..], &self.points[acc.1..]));
        acc.2
      },
    }
  }
}

#[derive(Debug)]
pub struct Shape {
  contours: Vec<Contour>,
  viewbox: Box<f32>,
}

#[derive(Debug, PartialEq)]
struct Box<T> {
  left: T,
  right: T,
  top: T,
  bottom: T,
}

impl Shape {
  /// Returns a ShapeBuilder.
  pub fn build() -> ShapeBuilder {
    ShapeBuilder::new()
  }

  /// Returns a vec of Contours comprising the Shape.
  /// TODO: could turn this into a lazy iterator.
  pub fn contours(&self) -> &Vec<Contour> {
    &self.contours
  }

  /// Returns an SVG representation of the Shape.
  pub fn svg(&self, draw_corners: bool) -> String {
    svg(self, draw_corners)
  }
}

pub struct ShapeBuilder {
  state: ShapeBuilderState,
  shape: Shape,
  viewbox: Option<Box<f32>>,
}

impl Default for ShapeBuilder {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(PartialEq)]
enum ShapeBuilderState {
  Shape,
  Contour,
}

impl ShapeBuilder {
  pub fn new() -> Self {
    Self {
      state: ShapeBuilderState::Shape,
      shape: Shape {
        contours: vec![],
        viewbox: Box {
          top: 0.0,
          bottom: 0.0,
          left: 0.0,
          right: 0.0,
        },
      },
      viewbox: None,
    }
  }

  /// Set the viewbox dimensions.
  pub fn viewbox(mut self, left: f32, right: f32, top: f32, bottom: f32) -> Self {
    self.viewbox = Some(Box {
      left,
      right,
      top,
      bottom,
    });
    self
  }

  // TODO: add example with smooth splines to check corner checking works.

  /// Create a new Contour.
  pub fn contour(mut self) -> Self {
    self.state = ShapeBuilderState::Contour;
    self.shape.contours.push(Contour {
      edge_segments: vec![],
      corners: vec![],
      points: vec![Point([0.0, 0.0])],
    });
    self
  }

  /// Set the starting coordinates of the current Contour.
  pub fn start(mut self, x: f32, y: f32) -> Self {
    if self.state == ShapeBuilderState::Shape {
      self = self.contour();
    }
    match self.state {
      ShapeBuilderState::Contour => {
        let contour = &mut self.shape.contours.last_mut().unwrap();
        contour.points[0] = Point([x, y]);
      },
      _ => panic!(),
    }
    self
  }

  /// Add a Line to the current Contour.
  pub fn line(mut self, x: f32, y: f32) -> Self {
    if self.state == ShapeBuilderState::Shape {
      self = self.contour();
    }
    match self.state {
      ShapeBuilderState::Contour => {
        let contour = &mut self.shape.contours.last_mut().unwrap();
        contour.edge_segments.push(EdgeSegment::Line);
        contour.points.push(Point([x, y]));
      },
      _ => panic!(),
    }
    self
  }

  /// Add a Quadratic bezier to the current Contour.
  pub fn quadratic(mut self, x1: f32, y1: f32, x: f32, y: f32) -> Self {
    if self.state == ShapeBuilderState::Shape {
      self = self.contour();
    }
    match self.state {
      ShapeBuilderState::Contour => {
        let contour = &mut self.shape.contours.last_mut().unwrap();
        contour.edge_segments.push(EdgeSegment::Quadratic);
        contour.points.push(Point([x1, y1]));
        contour.points.push(Point([x, y]));
      },
      _ => panic!(),
    }
    self
  }

  /// Add a Cubic bezier to the current Contour.
  pub fn cubic(mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) -> Self {
    if self.state == ShapeBuilderState::Shape {
      self = self.contour();
    }
    match self.state {
      ShapeBuilderState::Contour => {
        let contour = &mut self.shape.contours.last_mut().unwrap();
        contour.edge_segments.push(EdgeSegment::Cubic);
        contour.points.push(Point([x1, y1]));
        contour.points.push(Point([x2, y2]));
        contour.points.push(Point([x, y]));
      },
      _ => panic!(),
    }
    self
  }

  /// Generate the Shape.
  pub fn finalise(mut self) -> Shape {
    // Check starting and ending points are equal, otherwise add an edge.
    ensure_closed_contours(&mut self);

    // Cull contours with less than 4 points because they necessarily have zero area.
    cull_degenerate_contours(&mut self);

    // Identify sharp corners in each contours.
    identify_corners(&mut self);

    // Rearrange so that there's a corner at index zero (if there are any corners).
    rearrange(&mut self);

    // Generate viewbox if it wasn't explicitly given.
    ensure_viewbox(&mut self);
    self.shape.viewbox = self.viewbox.unwrap();

    self.shape
  }
}

/// If there's at least one corner, ensure that there's a corner at index 0.
fn rearrange(builder: &mut ShapeBuilder) {
  for contour in builder.shape.contours.iter_mut() {
    if contour.corners.len() > 0 {
      let offset = contour.corners[0];
      contour.corners.iter_mut().for_each(|(e, p)| {
        *e -= offset.0;
        *p -= offset.1;
      });
      let segments = [
        &contour.edge_segments[offset.0..],
        &contour.edge_segments[..offset.0],
      ].concat();
      let points = [&contour.points[offset.1..], &contour.points[..offset.1]].concat();
      contour.points = points;
      contour.edge_segments = segments;
    }
  }
}

fn cull_degenerate_contours(builder: &mut ShapeBuilder) {
  let mut indices = vec![];
  for (i, contour) in builder.shape.contours.iter().enumerate() {
    if contour.points.len() < 4 {
      indices.push(i);
    }
  }
  for &i in indices.iter() {
    builder.shape.contours.remove(i);
  }
}

// Assumes each contour has at least 3 points.
// This won't find corners that are intersections of curves, only endpoints between curves.
fn identify_corners(builder: &mut ShapeBuilder) {
  for contour in builder.shape.contours.iter_mut() {
    let mut points = contour.points.iter().enumerate();

    let last = contour.points.len() - 2; // get the previous to the first point
    let mut a = (last, &contour.points[last]);
    let mut b = points.next().unwrap(); // the first point
    let mut c;

    for edge in contour.edge_segments.iter().enumerate() {
      c = points.next().unwrap();
      if is_corner(*a.1, *b.1, *c.1) {
        contour.corners.push((edge.0, b.0)); // (edge_segments index, points index)
      }
      // find the relevant points for the next iteration of the loop
      match edge.1 {
        EdgeSegment::Line => {
          a = b;
          b = c;
        },
        EdgeSegment::Quadratic => {
          a = c;
          b = points.next().unwrap();
        },
        EdgeSegment::Cubic => {
          a = points.next().unwrap();
          b = points.next().unwrap();
        },
      }
    }
  }
}

fn ensure_closed_contours(builder: &mut ShapeBuilder) {
  for contour in builder.shape.contours.iter_mut() {
    let start_point = *contour.points.first().unwrap();
    let end_point = *contour.points.last().unwrap();
    if end_point != start_point {
      // If not closed then add a Line from the last point to the starting point.
      contour.edge_segments.push(EdgeSegment::Line);
      contour.points.push(start_point);
    }
  }
}

fn ensure_viewbox(builder: &mut ShapeBuilder) {
  if builder.viewbox == None {
    let points = &mut builder
      .shape
      .contours
      .iter()
      .map(|contour| contour.points.iter())
      .flatten();

    if let Some(Point([mut left, mut top])) = points.next() {
      let (mut right, mut bottom) = (left, top);
      for point in points {
        if point.x() < left {
          left = point.x();
        }
        if point.x() > right {
          right = point.x();
        }
        if point.y() < top {
          top = point.y();
        }
        if point.y() > bottom {
          bottom = point.y();
        }
      }
      builder.viewbox = Some(Box {
        left,
        right,
        top,
        bottom,
      });
    } else {
      builder.viewbox = Some(Box {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
      })
    }
  }
}

/// Generate an SVG from a Shape.
pub fn svg(shape: &Shape, draw_corners: bool) -> String {
  let mut svg = String::new();

  let viewbox = &shape.viewbox;
  let (width, height) = (viewbox.right - viewbox.left, viewbox.bottom - viewbox.top);
  svg.push_str(&format!(
    "<svg width='{width}' height='{height}' \
            viewBox='{x_min} {y_min} {width} {height}' \
            fill-rule='nonzero' \
            xmlns='http://www.w3.org/2000/svg'>",
    x_min = viewbox.left,
    y_min = viewbox.top,
  ));
  svg.push_str("<path d='");
  let svg_base_len = svg.len();

  for contour in shape.contours.iter() {
    let mut points = contour.points.iter();
    {
      // Starting coordinates.
      let Point([x, y]) = points.next().unwrap();
      svg.push_str(&format!("M{x},{y} "));
    }
    for edge_segment in contour.edge_segments.iter() {
      match edge_segment {
        EdgeSegment::Line => {
          let Point([x, y]) = points.next().unwrap();
          svg.push_str(&format!("L{x},{y} "));
        },
        EdgeSegment::Quadratic => {
          let Point([x1, y1]) = points.next().unwrap();
          let Point([x, y]) = points.next().unwrap();
          svg.push_str(&format!("Q{x1},{y1},{x},{y} "));
        },
        EdgeSegment::Cubic => {
          let Point([x1, y1]) = points.next().unwrap();
          let Point([x2, y2]) = points.next().unwrap();
          let Point([x, y]) = points.next().unwrap();
          svg.push_str(&format!("C{x1},{y1},{x2},{y2},{x},{y} "));
        },
      }
    }
  }
  if svg.len() > svg_base_len {
    svg.pop(); // Get rid of trailing whitespace.
  }
  svg.push_str("'/>");
  // Draw red circles at sharp corners.
  if draw_corners {
    for contour in shape.contours.iter() {
      for (corner, _) in contour.corners.iter() {
        let point = contour.points[*corner];
        svg.push_str(&format!(
          "<circle cx='{:?}' fill='red' cy='{:?}' r='2'/>",
          point.x(),
          point.y()
        ));
      }
    }
  }
  svg.push_str("</svg>");
  svg
}

/// The determinant of a 2 by 2 matrix.
/// Computes the oriented area of the parallelogram formed by the pair of vectors that constitute
/// the rows of the matrix.
#[inline]
fn det(m: [[f32; 2]; 2]) -> f32 {
  m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

/// The dot product of a pair of 2D vectors.
#[inline]
fn dot(a: [f32; 2], b: [f32; 2]) -> f32 {
  a[0] * b[0] + a[1] * b[1]
}

/// The magnitude of a 2D vector.
#[inline]
fn mag(a: [f32; 2]) -> f32 {
  (a[0] * a[0] + a[1] * a[1]).sqrt()
}

/// The unit vector in the direction of a 2D vector.
#[inline]
fn normalize(a: [f32; 2]) -> [f32; 2] {
  let mag_a = mag(a);
  [a[0] / mag_a, a[1] / mag_a]
}

const CORNER_THRESH: f32 = 0.05; // approx 3 degrees.
/// Compare the vector A->B and B->C to see if there is a sharp corner at Point B.
/// The const `CORNER_THRESH` is a small deflection (in radians) that will be permissible when
/// considering whether the two vectors constitute a "straight" line.
#[inline]
fn is_corner(a: Point<f32>, b: Point<f32>, c: Point<f32>) -> bool {
  let ab = [(b.x() - a.x()), (b.y() - a.y())];
  let bc = [(c.x() - b.x()), (c.y() - b.y())];
  // return false if the two vectors are not both almost parallel and in the same direction.
  #[rustfmt::skip]
  return !(
    dot(ab, bc) > 0.0
    && (det([ab, bc]) / (mag(ab) * mag(bc))).abs() <= CORNER_THRESH
  );
}
// Two other methods might be to use just the dot product, or to normalise both vectors, then
// convert them into polar coordinates to check the deflection.
