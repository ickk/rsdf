use super::{EdgeSegment, Shape};
use crate::math::Point;

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
      for (_, corner_point) in contour.corners.iter() {
        let point = contour.points[*corner_point];
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
