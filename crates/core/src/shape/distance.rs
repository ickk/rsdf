use crate::*;
use std::ops::Range;

impl Shape {
  /// Calculate the signed distance and orthogonality of a [`Point`] from a
  /// [`Spline`]
  pub fn spline_distance_orthogonality(
    &self,
    segments_range: Range<usize>,
    point: Point,
  ) -> (/* dist */ f32, /* orth */ f32) {
    let mut selected_dist = f32::INFINITY;
    // initial values don't matter since the first distance will always be set
    let mut selected_segment = None;
    let mut selected_t = 0f32;

    for &segment_ref in &self.segments[segments_range] {
      let segment = self.get_segment(segment_ref);
      let (dist, t) = segment.distance(point);
      if dist < selected_dist {
        selected_dist = dist;
        selected_segment = Some(segment);
        selected_t = t;
      }
    }

    // unwrap is okay since the selected segment will be always be set assuming
    // any dist < infinity are found above.
    let selected_segment = selected_segment.unwrap();
    let orthogonality = selected_segment
      .sample_derivative(selected_t.clamp(0., 1.))
      .norm()
      .signed_area(
        (point - selected_segment.sample(selected_t.clamp(0., 1.))).norm(),
      );

    // kind of redundant
    let signed_dist = selected_dist.copysign(orthogonality);

    (signed_dist, orthogonality.abs())
  }

  /// Calculate the signed pseudo distance of a [`Point`] from a [`Spline`]
  pub fn spline_pseudo_distance(
    &self,
    segments_range: Range<usize>,
    point: Point,
  ) -> f32 {
    let mut selected_dist = f32::INFINITY;
    let mut selected_segment = None;
    let mut selected_t = 0.0;
    // A place to store the points of extension lines if we need to construct
    // them.
    let mut extension_buf = [Point::ZERO; 2];

    // If there's only one segment in this spline
    if segments_range.len() == 1 {
      let segment_ref = self.segments[segments_range.start];
      let segment = self.get_segment(segment_ref);

      let (mut dist, mut t) = segment.pseudo_distance(point, ..);
      let mut segment_extended = false;
      if t < 0f32 {
        (dist, t) = check_start_extension(segment, point, &mut extension_buf);
        segment_extended = true;
      } else if t > 1f32 {
        (dist, t) = check_end_extension(segment, point, &mut extension_buf);
        segment_extended = true;
      }
      if dist < selected_dist {
        selected_dist = dist;
        selected_t = t;
        if segment_extended {
          selected_segment = Some(Segment::Line(&extension_buf));
        } else {
          selected_segment = Some(segment);
        }
      }
    }
    // Otherwise we've got a multi-segment spline
    else {
      let mut extended = false;

      for (i, &segment_ref)
        in self.segments[segments_range.clone()].iter().enumerate()
      {
        let segment = self.get_segment(segment_ref);

        let (mut dist, mut t);
        let mut segment_extended = false;
        // start of the spline
        if i == 0 {
          (dist, t) = segment.pseudo_distance(point, ..=1f32);
          if t < 0f32 {
            (dist, t) =
              check_start_extension(segment, point, &mut extension_buf);
            segment_extended = true;
          }
        }
        // end of the spline
        else if i == segments_range.len() - 1 {
          (dist, t) = segment.pseudo_distance(point, 0f32..);
          if t > 1f32 {
            (dist, t) =
              check_end_extension(segment, point, &mut extension_buf);
            segment_extended = true;
          }
        }
        // middle of the spline
        else {
          (dist, t) = segment.distance(point);
        }
        if dist < selected_dist {
          selected_dist = dist;
          selected_segment = Some(segment);
          selected_t = t;
          if segment_extended {
            extended = true;
          } else {
            extended = false;
          }
        }
      }
      if extended {
        selected_segment = Some(Segment::Line(&extension_buf));
      }
    }

    let selected_segment = selected_segment.unwrap();
    let sign = selected_segment
      .sample_derivative(selected_t)
      .signed_area(point - selected_segment.sample(selected_t));

    selected_dist.copysign(sign)
  }
}

/// Helper to generate & evaluate the straight line extending from the start
/// of a curve
#[inline]
fn check_start_extension(
  segment: Segment,
  point: Point,
  extension_buf: &mut [Point; 2],
) -> (/* dist */ f32, /* t */ f32) {
  let p0 = segment.sample(0.);
  let p1 = p0 + segment.sample_derivative(0.);
  *extension_buf = [p0, p1];
  Line::pseudo_distance(extension_buf, point, ..)
}

/// Helper to generate & evaluate the straight line extending from the end of
/// a curve
#[inline]
fn check_end_extension(
  segment: Segment,
  point: Point,
  extension_buf: &mut [Point; 2],
) -> (/* dist */ f32, /* t */ f32) {
  let p0 = segment.sample(1.);
  let p1 = p0 + segment.sample_derivative(1.);
  *extension_buf = [p0, p1];
  Line::pseudo_distance(extension_buf, point, ..)
}

/*
#[cfg(any(test, doctest))]
mod tests {
  use float_cmp::assert_approx_eq;

  #[test]
  fn spline_pseudo_distance() {
    use super::*;
    use std::f32::consts::SQRT_2;

    let points = &[
        (5., -1.).into(),
        (4., 1.).into(),
        (3., 3.).into(),
        (1., 1.).into(),
        (0., 0.).into(),
        (5., -1.).into(),
      ];
    let segments = &[
        (SegmentKind::Line, 0),
        (SegmentKind::QuadBezier, 1),
        (SegmentKind::Line, 3),
        (SegmentKind::Line, 4),
      ];
    let splines = &[(3, 0), (1, 3)];
    let spline_colours = &[Magenta, Yellow];

    let contour = Contour { splines };
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

    let points = &[
        (5., -1.).into(),
        (4., 1.).into(),
        (3., 3.).into(),
        (1., 1.).into(),
        (0., 0.).into(),
        (5., -1.).into(),
      ];
    let segments = &[
        (SegmentKind::Line, 0),
        (SegmentKind::QuadBezier, 1),
        (SegmentKind::Line, 3),
        (SegmentKind::Line, 4),
      ];
    let splines = &[(3, 0), (1, 3)];
    let spline_colours = &[Magenta, Yellow];

    let contour = Contour {
      points,
      segments,
      splines,
      spline_colours,
    };
    let spline = contour.splines().next().unwrap();

    {
      let point = (0., 0.).into();
      let (dist, _) = contour.spline_distance_orthogonality(spline, point);
      let expected = 0.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., 1.).into();
      let (dist, _) = contour.spline_distance_orthogonality(spline, point);
      let expected = -SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., -1.).into();
      // lies exactly on the curve so the sign is undefined
      let dist = contour.spline_distance_orthogonality(spline, point).0.abs();
      let expected = SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (0.5, 1.5).into();
      let (dist, _) = contour.spline_distance_orthogonality(spline, point);
      let expected = -SQRT_2 / 2.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 3.).into();
      let (dist, _) = contour.spline_distance_orthogonality(spline, point);
      let expected = -1.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 1.5).into();
      let (dist, _) = contour.spline_distance_orthogonality(spline, point);
      let expected = 0.5;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (5., 0.).into();
      let (dist, _) = contour.spline_distance_orthogonality(spline, point);
      let expected = -1. / 5f32.sqrt();
      assert_approx_eq!(f32, dist, expected);
    }
  }
}
*/
