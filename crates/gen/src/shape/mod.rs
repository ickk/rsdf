use std::iter::Map;

mod tests;

// #[derive(Debug)]
// pub struct CircleVec<T> {
//   vec:Vec<T>
// }

// impl<'a, T> CircleVec<T>
// where T: 'a {
//   pub fn from_vec(vec: Vec<T>) -> Self {
//     Self { vec }
//   }
// }

// impl<'a, T> std::ops::Index<usize> for CircleVec<T>
// where T: 'a {
//   type Output = T;

//   fn index(&self, index: usize) -> &Self::Output {
//     &self.vec[index % self.vec.len()]
//   }
// }

// unsafe impl<'a, T> std::slice::SliceIndex<usize> for CircleVec<T>
// where T: 'a {
//   type Output = T;

//   fn index(&self, range: std::ops::Range<usize>) {
//     //&self.vec[index % self.vec.len()]
//   }
// }

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T: Copy>([T; 2]);
impl<T: Copy> Point<T> {
  #[inline]
  pub fn x(&self) -> T { self.0[0] }
  #[inline]
  pub fn y(&self) -> T { self.0[1] }
}

#[derive(Debug, PartialEq)]
struct Box<T> {
  left: T,
  right: T,
  top: T,
  bottom: T,
}

#[derive(Debug)]
enum EdgeSegment {
  Line,
  Quadratic,
  Cubic,
}

#[derive(Debug)]
pub struct Contour {
  edge_segments: Vec<EdgeSegment>,
  corners: Vec<usize>,
  // // edges: Vec<Ranges>, views into groups of edge_segments that form one contiguous smooth edge.
  // corners: Vec<usize> // indices into edge_segments, denotes a sharp corner after the segment.
  // then we can call a `edges()/splines() -> iter(T)` method (chain two slice iterators if crosses
  // boundary).
  points: Vec<Point<f32>>,
}
impl Contour {
  pub fn corners(&self) -> Vec<Point<f32>> {
    self.corners.iter().map(|&idx| { self.points[idx] }).collect()
  }
}


#[inline]
fn det(m: [[f32; 2]; 2]) -> f32 {
  // The determinant of a 2 by 2 matrix.
  m[0][0]*m[1][1] - m[0][1]*m[1][0]
}

#[inline]
fn dot(a: [f32; 2], b: [f32; 2]) -> f32 {
  // The dot product of a pair of 2D vectors.
  a[0]*b[0] + a[1]*b[1]
}

#[inline]
fn mag(a: [f32; 2]) -> f32 {
  // The magnitude of a 2D vector.
  (a[0]*a[0] + a[1]*a[1]).sqrt()
}

#[inline]
fn normalize(a: [f32; 2]) -> [f32; 2] {
  // The unit vector in the direction of a 2D vector.
  let mag_a = mag(a);
  [a[0]/mag_a, a[1]/mag_a]
}


const corner_threshold: f32 = 0.05; // approx 3 degrees.
const cos_corner_threshold: f32 = 1.0-0.5*(corner_threshold*corner_threshold); //small angle approx
#[inline]
fn is_corner(a: Point<f32>, b: Point<f32>, c: Point<f32>) -> bool {
  // Compare the vector A->B and B->C to see if there is a sharp corner at Point B.
  // The const `theta_threshold` is a small deflection (in radians) that will be permissible when
  // considering whether the two vectors constitute a "straight" line.
  let ab = [(b.x()-a.x()), (b.y()-a.y())];
  let bc = [(c.x()-b.x()), (c.y()-b.y())];
  // Check the angle between the two vectors is using the dot product:
  // AB dot BC = |AB||BC|cos(theta).
  // => (AB dot BC)/|AB||BC|  =  1 - (theta^2)/2, as theta -> 0, and
  //   -(AB dot BC)/|AB||BC|  =  1 - (theta^2)/2, as theta -> pi.
  let lhs = dot(ab, bc)/(mag(ab)*mag(bc));
  // return false if the two vectors are not both almost parallel and in the same direction.
  return !(
    lhs.abs() >= cos_corner_threshold // parallel.
    && lhs >= 0.0 // codirectional (angle is less than pi/2).
  )
}
#[inline]
fn iscorner(a: Point<f32>, b: Point<f32>, c: Point<f32>) -> bool {
  let ab = [(b.x()-a.x()), (b.y()-a.y())];
  let bc = [(c.x()-b.x()), (c.y()-b.y())];
  // return false if the two vectors are not both almost parallel and in the same direction.
  return !(
    dot(ab, bc) > 0.0
    && det([ab, bc])/(mag(ab)*mag(bc)) <= corner_threshold
  )
}

#[derive(Debug)]
pub struct Shape {
  contours: Vec<Contour>,
  viewbox: Box<f32>,
}

impl Shape {
  /// Returns a ShapeBuilder.
  pub fn build() -> ShapeBuilder {
    ShapeBuilder::new()
  }

  pub fn contours(&self) -> &Vec<Contour> {
    &self.contours
  }

  /// Returns an SVG representation of the Shape.
  pub fn svg(&self) -> String {
    svg(self)
  }
}

pub struct ShapeBuilder {
  state: ShapeBuilderState,
  shape: Shape,
  viewbox: Option<Box<f32>>,
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

  //TODO corner adding algorithms adds corners when there's only 1 edge_segment. should skip first
  // check.

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
        // check if it's a sharp corner
        let len = contour.points.len();
        let a = contour.points[len-1- (len>1) as usize];
        let b = contour.points[len-1];
        let c = Point([x, y]);
        if is_corner(a, b, c) {
          contour.corners.push(len-1)
        }
        // Add line
        contour.edge_segments.push(EdgeSegment::Line);
        contour.points.push(c);
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
        // check if it's a sharp corner
        let len = contour.points.len();
        let a = contour.points[len-1- (len>1) as usize];
        let b = contour.points[len-1];
        let c = Point([x1, y1]);
        if is_corner(a, b, c) {
          contour.corners.push(len-1)
        }
        // Add bezier
        contour.edge_segments.push(EdgeSegment::Quadratic);
        contour.points.push(c);
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
        // check if it's a sharp corner
        let len = contour.points.len();
        let a = contour.points[len-1- (len>1) as usize];
        let b = contour.points[len-1];
        let c = Point([x1, y1]);
        if is_corner(a, b, c) {
          contour.corners.push(len-1)
        }
        // Add bezier
        contour.edge_segments.push(EdgeSegment::Cubic);
        contour.points.push(c);
        contour.points.push(Point([x2, y2]));
        contour.points.push(Point([x, y]));
      },
      _ => panic!(),
    }
    self
  }

  /// Generate the Shape.
  pub fn finalise(mut self) -> Shape {
    for contour in self.shape.contours.iter_mut() {
      // Check starting and ending points are equal.
      let start_point = *contour.points.first().unwrap();
      let end_point = *contour.points.last().unwrap();
      eprintln!("start,end: {start_point:?}, {end_point:?}");
      eprintln!("start==end: {:?}", start_point==end_point);
      if end_point != start_point {
        // If not then add a Line from the last point to the starting point.
        contour.edge_segments.push(EdgeSegment::Line);
        contour.points.push(start_point);
      }
      // check if the first and last EdgeSegment form a sharp corner.
      let len = contour.points.len();
      let a = contour.points[len-1];
      let b = contour.points[0];
      let c = contour.points[(len>1) as usize];
      if is_corner(a, b, c) {
        contour.corners.push(0);
      }
    }

    // Generate viewbox if it wasn't explicitly given.
    if self.viewbox == None {
      let points = &mut self
        .shape
        .contours
        .iter()
        .map(|contour| contour.points.iter())
        .flatten();

      if let Some(Point([mut left, mut top])) = points.next()
      {
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
        self.viewbox = Some(Box {
          left,
          right,
          top,
          bottom,
        });
      } else {
        self.viewbox = Some(Box {
          left: 0.0,
          right: 0.0,
          top: 0.0,
          bottom: 0.0,
        })
      }
    }
    self.shape.viewbox = self.viewbox.unwrap();

    self.shape
  }
}

/// Generate an SVG from a Shape
pub fn svg(shape: &Shape) -> String {
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
  svg.push_str("</svg>");
  svg
}

impl Default for ShapeBuilder {
  fn default() -> Self {
    Self::new()
  }
}
