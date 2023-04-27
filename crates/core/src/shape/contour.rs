use crate::*;

/// The kind of a segment
#[derive(Debug, Clone, Copy)]
enum SegmentKind {
  Line,
  QuadBezier,
  CubicBezier,
}

type SegmentIndex = (SegmentKind, /* points index */ usize);

/// A reference to a segment in the Contour
#[derive(Debug, Clone, Copy, PartialEq)]
enum Segment<'contour> {
  Line(&'contour [Point]),
  QuadBezier(&'contour [Point]),
  CubicBezier(&'contour [Point]),
}

type SplineIndex = (/* length */ usize, /* segments index */ usize);

/// A reference to a spline in the Contour
#[derive(Debug, Clone, Copy)]
struct Spline<'contour> {
  segments: &'contour [SegmentIndex],
}

// A Contour is a path describing a closed region of space.
//
// Sharp corners are assumed to be located at the boundary points of adjacent
// splines.
pub struct Contour {
  /// A buffer containing the points
  points: Vec<Point>,
  /// A buffer containing references to the segments
  segments: Vec<SegmentIndex>,
  /// A buffer containing references to the splines
  splines: Vec<SplineIndex>,
  /// A buffer containing the colours corresponding to the respective Spline.
  ///
  /// Might not be computed.
  edge_colours: Option<Vec<Colour>>,
  // TODO: add a flag for fully-smooth. Otherwise there's an ambiguity
  // between teardrop and fully-smooth contours.
}

impl Contour {
  fn get_segment(&self, (kind, i): SegmentIndex) -> Segment {
    match kind {
      SegmentKind::Line => Segment::Line(&self.points[i..i + 2]),
      SegmentKind::QuadBezier => Segment::QuadBezier(&self.points[i..i + 3]),
      SegmentKind::CubicBezier => Segment::CubicBezier(&self.points[i..i + 4]),
    }
  }

  fn segments<'contour>(
    &'contour self,
    spline: Spline<'contour>,
  ) -> impl Iterator<Item = Segment> + 'contour {
    spline
      .segments
      .iter()
      .map(|segment_index| self.get_segment(*segment_index))
  }

  fn get_spline(&self, (length, index): SplineIndex) -> Spline {
    Spline {
      segments: &self.segments[index..index + length],
    }
  }

  fn splines(&self) -> impl Iterator<Item = Spline> {
    self
      .splines
      .iter()
      .map(|&spline_index| self.get_spline(spline_index))
  }

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
  fn contour_get_spline_segments() {
    use super::*;
    let contour = Contour {
      points: vec![
        (0., 0.).into(),
        (1., 1.).into(),
        (2., 2.).into(),
        (3., 3.).into(),
        (4., 4.).into(),
        (5., 5.).into(),
        (6., 6.).into(),
        (7., 7.).into(),
        (0., 0.).into(),
      ],
      segments: vec![
        (SegmentKind::Line, 0),
        (SegmentKind::QuadBezier, 1),
        (SegmentKind::CubicBezier, 3),
        (SegmentKind::Line, 6),
        (SegmentKind::Line, 7),
      ],
      splines: vec![(3, 0), (2, 3)],
      edge_colours: None,
    };

    {
      let result: Vec<_> = contour
        .segments(contour.get_spline(contour.splines[0]))
        .collect();

      let s1 = [(0., 0.).into(), (1., 1.).into()];
      let s2 = [(1., 1.).into(), (2., 2.).into(), (3., 3.).into()];
      let s3 = [
        (3., 3.).into(),
        (4., 4.).into(),
        (5., 5.).into(),
        (6., 6.).into(),
      ];

      let expected = vec![
        Segment::Line(&s1),
        Segment::QuadBezier(&s2),
        Segment::CubicBezier(&s3),
      ];

      assert_eq!(result, expected);
    }

    {
      let result: Vec<_> = contour
        .segments(contour.get_spline(contour.splines[1]))
        .collect();

      let s4 = [(6., 6.).into(), (7., 7.).into()];
      let s5 = [(7., 7.).into(), (0., 0.).into()];

      let expected = vec![
        Segment::Line(&s4),
        Segment::Line(&s5),
      ];

      assert_eq!(result, expected);
    }
  }

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
