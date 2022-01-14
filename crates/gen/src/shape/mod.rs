mod tests;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point<T> {
  x: T,
  y: T,
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
struct Contour {
  edge_segments: Vec<EdgeSegment>,
  points: Vec<Point<f32>>,
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

  /// Create a new Contour.
  pub fn contour(mut self) -> Self {
    self.state = ShapeBuilderState::Contour;
    self.shape.contours.push(Contour {
      edge_segments: vec![],
      points: vec![Point { x: 0.0, y: 0.0 }],
    });
    self
  }

  /// Set the starting coordinates of the current Contour.
  pub fn start(mut self, x: f32, y: f32) -> Self {
    match self.state {
      ShapeBuilderState::Contour => {
        let contour = &mut self.shape.contours.last_mut().unwrap();
        contour.points[0] = Point { x, y };
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
        contour.points.push(Point { x, y });
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
        contour.points.push(Point { x: x1, y: y1 });
        contour.points.push(Point { x, y });
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
        contour.points.push(Point { x: x1, y: y1 });
        contour.points.push(Point { x: x2, y: y2 });
        contour.points.push(Point { x, y });
      },
      _ => panic!(),
    }
    self
  }

  /// Generate the Shape.
  pub fn finalise(mut self) -> Shape {
    // Check starting and ending points are equal.
    for contour in self.shape.contours.iter_mut() {
      let start_point = *contour.points.first().unwrap();
      let end_point = *contour.points.last().unwrap();
      if end_point != start_point {
        // If not then add a Line from the last point to the starting point.
        contour.edge_segments.push(EdgeSegment::Line);
        contour.points.push(start_point);
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

      if let Some(Point {
        x: mut left,
        y: mut top,
      }) = points.next()
      {
        let (mut right, mut bottom) = (left, top);
        for point in points {
          if point.x < left {
            left = point.x;
          }
          if point.x > right {
            right = point.x;
          }
          if point.y < top {
            top = point.y;
          }
          if point.y > bottom {
            bottom = point.y;
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
    // Starting coordinates.
    {
      let Point { x, y } = points.next().unwrap();
      svg.push_str(&format!("M{x},{y} "));
    }
    for edge_segment in contour.edge_segments.iter() {
      match edge_segment {
        EdgeSegment::Line => {
          let Point { x, y } = points.next().unwrap();
          svg.push_str(&format!("L{x},{y} "));
        },
        EdgeSegment::Quadratic => {
          let Point { x: x1, y: y1 } = points.next().unwrap();
          let Point { x, y } = points.next().unwrap();
          svg.push_str(&format!("Q{x1},{y1},{x},{y} "));
        },
        EdgeSegment::Cubic => {
          let Point { x: x1, y: y1 } = points.next().unwrap();
          let Point { x: x2, y: y2 } = points.next().unwrap();
          let Point { x, y } = points.next().unwrap();
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
