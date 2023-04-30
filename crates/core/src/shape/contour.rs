use crate::*;

// SegmentKind implicitly gives the length
pub type SegmentIndex = (SegmentKind, /* points index */ usize);

/// A reference to a segment in the Contour
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Segment<'contour> {
  Line(&'contour [Point]),
  QuadBezier(&'contour [Point]),
  CubicBezier(&'contour [Point]),
}

type SplineIndex = (/* length */ usize, /* segments index */ usize);

/// A reference to a spline in the Contour
#[derive(Debug, Clone, Copy)]
pub struct Spline<'contour> {
  pub segments: &'contour [SegmentIndex],
  pub colour: Option<Colour>,
}

impl Spline<'_> {
  pub fn len(&self) -> usize {
    self.segments.len()
  }
}

// A Contour is a path describing a closed region of space.
//
// Sharp corners are assumed to be located at the boundary points of adjacent
// splines.
pub struct Contour {
  /// A buffer containing the points
  pub points: Vec<Point>,
  /// A buffer containing references to the segments
  pub segments: Vec<SegmentIndex>,
  /// A buffer containing references to the splines
  pub splines: Vec<SplineIndex>,
  /// A buffer containing the colours corresponding to the respective Spline.
  ///
  /// Might not be computed.
  pub spline_colours: Option<Vec<Colour>>,
  // TODO: add a flag for fully-smooth. Otherwise there's an ambiguity
  // between teardrop and fully-smooth contours.
}

impl<'contour> Contour {
  #[inline]
  fn get_segment(&self, (kind, i): SegmentIndex) -> Segment {
    match kind {
      SegmentKind::Line => Segment::Line(&self.points[i..i + 2]),
      SegmentKind::QuadBezier => Segment::QuadBezier(&self.points[i..i + 3]),
      SegmentKind::CubicBezier => Segment::CubicBezier(&self.points[i..i + 4]),
    }
  }

  #[inline]
  fn get_spline(&self, i: usize) -> Spline {
    let (length, index) = self.splines[i];
    Spline {
      segments: &self.segments[index..index + length],
      colour: self.spline_colours.as_deref().map(|cs| cs[i]),
    }
  }

  #[inline]
  pub fn segments(
    &'contour self,
    spline: Spline<'contour>,
  ) -> impl Iterator<Item = Segment> + 'contour {
    spline
      .segments
      .iter()
      .map(|segment_index| self.get_segment(*segment_index))
  }

  pub fn splines(&self) -> impl Iterator<Item = Spline> {
    (0..self.splines.len()).map(|i| self.get_spline(i))
  }

  /// Calculate the signed distance to the spline
  pub fn spline_distance(
    &self,
    spline: Spline,
    point: Point,
  ) -> (/* dist */ f32, /* orth */ f32) {
    use Segment::*;
    let mut selected_dist = f32::INFINITY;
    // initial values don't matter since the first distance will always be set
    let mut selected_segment = None;
    let mut selected_t = 0.0;

    for segment in self.segments(spline) {
      let (dist, t) = match segment {
        Line(ps) => line_distance(ps, point),
        QuadBezier(ps) => quad_bezier_distance(ps, point),
        CubicBezier(ps) => cubic_bezier_distance(ps, point),
      };
      if dist < selected_dist {
        selected_dist = dist;
        selected_segment = Some(segment);
        selected_t = t;
      }
    }

    // unwrap is okay since the selected segment will be always be set assuming
    // any dist < infinity are found above.
    let orthogonality = match selected_segment.unwrap() {
      Line(ps) => sample_line_direction(ps, selected_t.clamp(0., 1.))
        .signed_area(
          (point - sample_line(ps, selected_t.clamp(0., 1.))).norm(),
        ),
      QuadBezier(ps) => {
        sample_quad_bezier_direction(ps, selected_t.clamp(0., 1.)).signed_area(
          (point - sample_quad_bezier(ps, selected_t.clamp(0., 1.))).norm(),
        )
      },
      CubicBezier(ps) => {
        sample_cubic_bezier_direction(ps, selected_t.clamp(0., 1.))
          .signed_area(
            (point - sample_cubic_bezier(ps, selected_t.clamp(0., 1.))).norm(),
          )
      },
    };

    // kind of redundant
    let signed_dist = selected_dist.copysign(orthogonality);

    (signed_dist, orthogonality.abs())
  }

  /// Calculate the signed pseudo distance to the spline
  pub fn spline_pseudo_distance(&self, spline: Spline, point: Point) -> f32 {
    use Segment::*;
    let mut selected_dist = f32::INFINITY;
    let mut selected_segment = None;
    let mut selected_t = 0.0;

    if spline.len() > 1 {
      for (i, segment) in self.segments(spline).enumerate() {
        // start of the spline
        if i == 0 {
          let (dist, t) = match segment {
            Line(ps) => line_pseudo_distance(ps, point, ..=1f32),
            QuadBezier(ps) => quad_bezier_pseudo_distance(ps, point, ..=1f32),
            CubicBezier(ps) => {
              cubic_bezier_pseudo_distance(ps, point, ..=1f32)
            },
          };
          if dist < selected_dist {
            selected_dist = dist;
            selected_segment = Some(segment);
            selected_t = t;
          }
        }
        // end of the spline
        else if i == spline.len() - 1 {
          let (dist, t) = match segment {
            Line(ps) => line_pseudo_distance(ps, point, 0f32..),
            QuadBezier(ps) => quad_bezier_pseudo_distance(ps, point, 0f32..),
            CubicBezier(ps) => cubic_bezier_pseudo_distance(ps, point, 0f32..),
          };
          if dist < selected_dist {
            selected_dist = dist;
            selected_segment = Some(segment);
            selected_t = t;
          }
        }
        // middle of the spline
        let (dist, t) = match segment {
          Line(ps) => line_distance(ps, point),
          QuadBezier(ps) => quad_bezier_distance(ps, point),
          CubicBezier(ps) => cubic_bezier_distance(ps, point),
        };
        if dist < selected_dist {
          selected_dist = dist;
          selected_segment = Some(segment);
          selected_t = t;
        }
      }
    } else {
      // There's only one segment in this spline
      let segment = self.segments(spline).next().unwrap();
      let (dist, t) = match segment {
        Line(ps) => line_pseudo_distance(ps, point, ..),
        QuadBezier(ps) => quad_bezier_pseudo_distance(ps, point, ..),
        CubicBezier(ps) => cubic_bezier_pseudo_distance(ps, point, ..),
      };
      if dist < selected_dist {
        selected_dist = dist;
        selected_segment = Some(segment);
        selected_t = t;
      }
    }

    let sign = match selected_segment.unwrap() {
      Line(ps) => sample_line_direction(ps, selected_t)
        .signed_area(point - sample_line(ps, selected_t)),
      QuadBezier(ps) => sample_quad_bezier_direction(ps, selected_t)
        .signed_area(point - sample_quad_bezier(ps, selected_t)),
      CubicBezier(ps) => sample_cubic_bezier_direction(ps, selected_t)
        .signed_area(point - sample_cubic_bezier(ps, selected_t)),
    };

    selected_dist.copysign(sign)
  }
}

#[cfg(any(test, doctest))]
mod tests {
  use float_cmp::assert_approx_eq;

  #[test]
  fn spline_pseudo_distance() {
    use super::*;
    use std::f32::consts::SQRT_2;

    let contour = Contour {
      points: vec![
        (5., -1.).into(),
        (4., 1.).into(),
        (3., 3.).into(),
        (1., 1.).into(),
        (0., 0.).into(),
        (5., -1.).into(),
      ],
      segments: vec![
        (SegmentKind::Line, 0),
        (SegmentKind::QuadBezier, 1),
        (SegmentKind::Line, 3),
        (SegmentKind::Line, 4),
      ],
      splines: vec![(3, 0), (1, 3)],
      spline_colours: None,
    };

    let spline = contour.splines().next().unwrap();

    {
      let point = (0., 0.).into();
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = 0.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., 1.).into();
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = -SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., -1.).into();
      // lies exactly on the curve so the sign is undefined
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = 0.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (0.5, 1.5).into();
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = -SQRT_2 / 2.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 3.).into();
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = -1.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 1.5).into();
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = 0.5;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (5., 0.).into();
      let dist = contour.spline_pseudo_distance(spline, point);
      let expected = -1. / 5f32.sqrt();
      assert_approx_eq!(f32, dist, expected);
    }
  }

  #[test]
  fn spline_distance() {
    use super::*;
    use std::f32::consts::SQRT_2;

    let contour = Contour {
      points: vec![
        (5., -1.).into(),
        (4., 1.).into(),
        (3., 3.).into(),
        (1., 1.).into(),
        (0., 0.).into(),
        (5., -1.).into(),
      ],
      segments: vec![
        (SegmentKind::Line, 0),
        (SegmentKind::QuadBezier, 1),
        (SegmentKind::Line, 3),
        (SegmentKind::Line, 4),
      ],
      splines: vec![(3, 0), (1, 3)],
      spline_colours: None,
    };

    let spline = contour.splines().next().unwrap();

    {
      let point = (0., 0.).into();
      let (dist, _) = contour.spline_distance(spline, point);
      let expected = 0.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., 1.).into();
      let (dist, _) = contour.spline_distance(spline, point);
      let expected = -SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., -1.).into();
      // lies exactly on the curve so the sign is undefined
      let dist = contour.spline_distance(spline, point).0.abs();
      let expected = SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (0.5, 1.5).into();
      let (dist, _) = contour.spline_distance(spline, point);
      let expected = -SQRT_2 / 2.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 3.).into();
      let (dist, _) = contour.spline_distance(spline, point);
      let expected = -1.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 1.5).into();
      let (dist, _) = contour.spline_distance(spline, point);
      let expected = 0.5;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (5., 0.).into();
      let (dist, _) = contour.spline_distance(spline, point);
      let expected = -1. / 5f32.sqrt();
      assert_approx_eq!(f32, dist, expected);
    }
  }

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
      spline_colours: None,
    };

    {
      let result: Vec<_> = contour.segments(contour.get_spline(0)).collect();

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
      let result: Vec<_> = contour.segments(contour.get_spline(1)).collect();

      let s4 = [(6., 6.).into(), (7., 7.).into()];
      let s5 = [(7., 7.).into(), (0., 0.).into()];

      let expected = vec![Segment::Line(&s4), Segment::Line(&s5)];

      assert_eq!(result, expected);
    }
  }
}
