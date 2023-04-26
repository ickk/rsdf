use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum Segment {
  Line,
  QuadBezier,
  CubicBezier,
}

#[derive(Debug, Clone, Copy)]
pub struct Spline {
  len: usize,
  index: usize,
}

// A Contour is a path describing a closed region of space.
//
// Sharp corners are assumed to be located at the boundary points of adjacent
// splines.
pub struct Contour {
  /// A buffer with all of the points in it.
  point_buffer: Vec<Point>,
  /// A buffer containing Segment kind and an index pairs. Where the index
  /// points into the point_buffer to the first of the Segment's points.
  segments: Vec<(Segment, /* `point_buffer` index */ usize)>,
  /// A buffer containing the splines
  splines: Vec<Spline>,
  /// A buffer containing the colours corresponding to the respective Spline.
  ///
  /// Might not be computed.
  edge_colours: Option<Vec<Colour>>,
  // TODO: add a flag for fully-smooth. Otherwise there's an ambiguity
  // between teardrop and fully-smooth contours.
}

impl Contour {
  pub fn spline_distance(&self, spline_index: usize, point: Point) -> f32 {
    todo!()
  }
}

fn sample_line(p: &[Point; 2], t: f32) -> Point {
  p[0] + t * (p[1] - p[0])
}

#[rustfmt::skip]
fn sample_quad_bezier(p: &[Point; 3], t: f32) -> Point {
  p[0]
    + 2f32*t
      * (p[1] - p[0])
    + t*t
      * (p[2].as_vector() - 2f32*p[1].as_vector() + p[0].as_vector())
}

#[rustfmt::skip]
fn sample_cubic_bezier(p: &[Point; 4], t: f32) -> Point {
  p[0]
    + 3f32*t
      * (p[1] - p[0])
    + 3f32*t*t
      * (p[2].as_vector() - 2f32*p[1].as_vector() + p[0].as_vector())
    + t.powi(3)
      * (p[3].as_vector() - 3f32*p[2].as_vector() + 3f32*p[1].as_vector() - p[0].as_vector())
}

#[cfg(any(test, doctest))]
mod tests {
  #[test]
  fn sample_line() {
    use super::*;
    {
      let line = [(0.0, 0.0).into(), (4.0, 0.0).into()];
      {
        let t = 0.0;
        let result = sample_line(&line, t);
        let expected = (0.0, 0.0).into();
        assert_eq!(result, expected);
      }
      {
        let t = 0.5;
        let result = sample_line(&line, t);
        let expected = (2.0, 0.0).into();
        assert_eq!(result, expected);
      }
      {
        let t = 1.0;
        let result = sample_line(&line, t);
        let expected = (4.0, 0.0).into();
        assert_eq!(result, expected);
      }
      {
        let t = -1.0;
        let result = sample_line(&line, t);
        let expected = (-4.0, 0.0).into();
        assert_eq!(result, expected);
      }
    }
  }

  #[test]
  fn sample_quad_bezier() {
    use super::*;
    {
      let quad = [(0.0, 0.0).into(), (2.0, 2.0).into(), (4.0, 0.0).into()];
      {
        let t = 0.0;
        let result = sample_quad_bezier(&quad, t);
        let expected = (0.0, 0.0).into();
        assert_eq!(result, expected);
      }
      {
        let t = 0.5;
        let result = sample_quad_bezier(&quad, t);
        let expected = (2.0, 1.0).into();
        assert_eq!(result, expected);
      }
      {
        let t = 1.0;
        let result = sample_quad_bezier(&quad, t);
        let expected = (4.0, 0.0).into();
        assert_eq!(result, expected);
      }
    }
  }

  #[test]
  fn sample_cubic_bezier() {
    use super::*;
    {
      let cubic = [
        (0.0, 0.0).into(),
        (1.0, 2.0).into(),
        (3.0, 2.0).into(),
        (4.0, 0.0).into(),
      ];
      {
        let t = 0.0;
        let result = sample_cubic_bezier(&cubic, t);
        let expected = (0.0, 0.0).into();
        assert_eq!(result, expected);
      }
      {
        let t = 0.5;
        let result = sample_cubic_bezier(&cubic, t);
        let expected = (2.0, 1.5).into();
        assert_eq!(result, expected);
      }
      {
        let t = 1.0;
        let result = sample_cubic_bezier(&cubic, t);
        let expected = (4.0, 0.0).into();
        assert_eq!(result, expected);
      }
    }
  }
}
