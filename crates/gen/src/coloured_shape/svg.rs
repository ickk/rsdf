use super::*;

pub(super) fn svg(coloured_shape: &ColouredShape) -> String {
  let mut svg = String::new();
  let shape = &coloured_shape.shape;

  let viewbox = &shape.viewbox;
  let (width, height) = (viewbox.right - viewbox.left, viewbox.bottom - viewbox.top);
  svg.push_str(&format!(
    "<svg width='{width}' height='{height}' \
            viewBox='{x_min} {y_min} {width} {height}' \
            fill='transparent' \
            stroke='white' \
            style='background-color:black' \
            xmlns='http://www.w3.org/2000/svg'>",
    x_min = viewbox.left,
    y_min = viewbox.top,
  ));

  // Build 4 strings, one for each possible colour (cyan, magenta, yellow) and normal for white/black
  let mut str_normal = String::new();
  let mut str_cyan = String::new();
  let mut str_magenta = String::new();
  let mut str_yellow = String::new();
  str_normal.push_str("<path stroke='currentcolor' d='");
  str_cyan.push_str("<path stroke='cyan' d='");
  str_magenta.push_str("<path stroke='magenta' d='");
  str_yellow.push_str("<path stroke='yellow' d='");

  let mut contours_colours = coloured_shape.colours.iter();
  for contour in shape.contours().iter() {
    let mut colours = contours_colours.next().unwrap().iter();

    for (segments, points) in contour.splines() {
      let colour = colours.next().unwrap().colour();
      let selected_str = match colour {
        Colour::Cyan => &mut str_cyan,
        Colour::Magenta => &mut str_magenta,
        Colour::Yellow => &mut str_yellow,
        _ => &mut str_normal,
      };

      let mut points = points.iter();
      {
        // starting coordinates.
        let Point([x, y]) = points.next().unwrap();
        selected_str.push_str(&format!("M{x},{y} "));
      }

      for segment in segments.iter() {
        match segment {
          EdgeSegment::Line => {
            let Point([x, y]) = points.next().unwrap();
            selected_str.push_str(&format!("L{x},{y} "));
          },
          EdgeSegment::Quadratic => {
            let Point([x1, y1]) = points.next().unwrap();
            let Point([x, y]) = points.next().unwrap();
            selected_str.push_str(&format!("Q{x1},{y1},{x},{y} "));
          },
          EdgeSegment::Cubic => {
            let Point([x1, y1]) = points.next().unwrap();
            let Point([x2, y2]) = points.next().unwrap();
            let Point([x, y]) = points.next().unwrap();
            selected_str.push_str(&format!("C{x1},{y1},{x2},{y2},{x},{y} "));
          },
        }
      }
    }
  }
  str_normal.push_str("'/>");
  str_cyan.push_str("'/>");
  str_magenta.push_str("'/>");
  str_yellow.push_str("'/>");
  svg.push_str(&str_normal);
  svg.push_str(&str_cyan);
  svg.push_str(&str_magenta);
  svg.push_str(&str_yellow);
  svg.push_str("</svg>");
  svg
}
