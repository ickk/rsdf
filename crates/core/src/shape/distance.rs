use crate::*;
use std::ops::Range;

impl Shape {
  /// Calculate the signed distance and orthogonality of a [`Point`] from a
  /// [`Spline`]
  pub fn spline_distance_orthogonality(
    &self,
    segments_range: Range<usize>,
    point: Point,
  ) -> ((/* dist */ f32, /* orth */ f32), /* end_bias */ Bias) {
    let mut selected_dist = f32::INFINITY;
    // initial values don't matter since the first distance will always be set
    let mut selected_segment = None;
    let mut selected_t = f32::NAN;

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

    // this bias corrects artifacts caused by the pseudo-distance of a spline
    // looping back on itself
    let bias = if selected_t <= 0f32 {
      Bias::Start
    } else if selected_t >= 1f32 {
      Bias::End
    } else {
      Bias::Centre
    };

    ((signed_dist, orthogonality.abs()), bias)
  }

  /// Calculate the signed pseudo distance of a [`Point`] from a [`Spline`]
  pub fn spline_pseudo_distance(
    &self,
    segments_range: Range<usize>,
    point: Point,
    bias: Bias,
  ) -> f32 {
    let mut selected_dist = f32::INFINITY;
    let mut selected_segment = None;
    let mut selected_t = f32::NAN;
    // If there's only one segment in this spline
    if segments_range.len() == 1 {
      let segment_ref = self.segments[segments_range.start];
      let segment = self.get_segment(segment_ref);

      let (dist, t) = match bias {
        Bias::Start => segment.pseudo_distance(point, ..=0f32),
        Bias::End => segment.pseudo_distance(point, 1f32..),
        Bias::Centre => segment.pseudo_distance(point, 0f32..=1f32),
      };
      selected_dist = dist;
      selected_t = t;
      selected_segment = Some(segment);
    }
    // Otherwise we've got a multi-segment spline
    else {
      for (i, &segment_ref) in
        self.segments[segments_range.clone()].iter().enumerate()
      {
        let segment = self.get_segment(segment_ref);
        let (dist, t) = if i == 0 {
          // first
          if !matches!(bias, Bias::Start) {
            segment.pseudo_distance(point, 0f32..=1f32)
          } else {
            segment.pseudo_distance(point, ..=1f32)
          }
        } else if i == segments_range.len() - 1 {
          // last
          if !matches!(bias, Bias::End) {
            segment.pseudo_distance(point, 0f32..=1f32)
          } else {
            segment.pseudo_distance(point, 0f32..)
          }
        } else {
          // middle
          segment.pseudo_distance(point, ..)
        };
        if dist < selected_dist {
          selected_dist = dist;
          selected_segment = Some(segment);
          selected_t = t;
        }
      }
    }

    let selected_segment = selected_segment.unwrap();
    let sign = selected_segment
      .sample_derivative(selected_t)
      .signed_area(point - selected_segment.sample(selected_t));

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
    use SegmentKind::*;

    let points = vec![
      (5., -1.).into(),
      (4., 1.).into(),
      (3., 3.).into(),
      (1., 1.).into(),
      (0., 0.).into(),
      (5., -1.).into(),
    ];
    let segments = vec![
      SegmentRef {
        kind: Line,
        points_index: 0,
      },
      SegmentRef {
        kind: QuadBezier,
        points_index: 1,
      },
      SegmentRef {
        kind: Line,
        points_index: 3,
      },
      SegmentRef {
        kind: Line,
        points_index: 4,
      },
    ];
    let splines = vec![
      Spline {
        segments_range: 0..3,
        colour: Magenta,
      },
      Spline {
        segments_range: 3..4,
        colour: Yellow,
      },
    ];
    let contours = vec![Contour { spline_range: 0..2 }];
    let shape = Shape {
      points,
      segments,
      splines,
      contours,
    };

    let Spline {
      segments_range,
      colour: _,
    } = shape.splines[0].clone();

    {
      let point = (0., 0.).into();
      let dist = shape.spline_pseudo_distance(
        segments_range.clone(),
        point,
        Bias::Centre,
      );
      let expected = 0.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., 1.).into();
      let dist = shape.spline_pseudo_distance(
        segments_range.clone(),
        point,
        Bias::Centre,
      );
      let expected = -SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., -1.).into();
      // lies exactly on the curve so the sign is undefined
      let dist = shape.spline_pseudo_distance(
        segments_range.clone(),
        point,
        Bias::Centre,
      );
      let expected = 0.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (0.5, 1.5).into();
      let dist = shape.spline_pseudo_distance(
        segments_range.clone(),
        point,
        Bias::Centre,
      );
      let expected = -SQRT_2 / 2.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 3.).into();
      let dist = shape.spline_pseudo_distance(
        segments_range.clone(),
        point,
        Bias::Centre,
      );
      let expected = -1.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 1.5).into();
      let dist = shape.spline_pseudo_distance(
        segments_range.clone(),
        point,
        Bias::Centre,
      );
      let expected = 0.5;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (5., 0.).into();
      let dist = shape.spline_pseudo_distance(
        segments_range.clone(),
        point,
        Bias::Centre,
      );
      let expected = -1. / 5f32.sqrt();
      assert_approx_eq!(f32, dist, expected);
    }
  }

  #[test]
  fn spline_distance_orthogonality() {
    use super::*;
    use std::f32::consts::SQRT_2;
    use SegmentKind::*;

    let points = vec![
      (5., -1.).into(),
      (4., 1.).into(),
      (3., 3.).into(),
      (1., 1.).into(),
      (0., 0.).into(),
      (5., -1.).into(),
    ];
    let segments = vec![
      SegmentRef {
        kind: Line,
        points_index: 0,
      },
      SegmentRef {
        kind: QuadBezier,
        points_index: 1,
      },
      SegmentRef {
        kind: Line,
        points_index: 3,
      },
      SegmentRef {
        kind: Line,
        points_index: 4,
      },
    ];
    let splines = vec![
      Spline {
        segments_range: 0..3,
        colour: Magenta,
      },
      Spline {
        segments_range: 3..4,
        colour: Yellow,
      },
    ];
    let contours = vec![Contour { spline_range: 0..2 }];
    let shape = Shape {
      points,
      segments,
      splines,
      contours,
    };

    let Spline {
      segments_range,
      colour: _,
    } = shape.splines[0].clone();

    {
      let point = (0., 0.).into();
      let ((dist, _), _) =
        shape.spline_distance_orthogonality(segments_range.clone(), point);
      let expected = 0.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., 1.).into();
      let ((dist, _), _) =
        shape.spline_distance_orthogonality(segments_range.clone(), point);
      let expected = -SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (-1., -1.).into();
      // lies exactly on the curve so the sign is undefined
      let dist = shape
        .spline_distance_orthogonality(segments_range.clone(), point)
        .0
         .0
        .abs();
      let expected = SQRT_2;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (0.5, 1.5).into();
      let ((dist, _), _) =
        shape.spline_distance_orthogonality(segments_range.clone(), point);
      let expected = -SQRT_2 / 2.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 3.).into();
      let ((dist, _), _) =
        shape.spline_distance_orthogonality(segments_range.clone(), point);
      let expected = -1.;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (2.75, 1.5).into();
      let ((dist, _), _) =
        shape.spline_distance_orthogonality(segments_range.clone(), point);
      let expected = 0.5;
      assert_approx_eq!(f32, dist, expected);
    }
    {
      let point = (5., 0.).into();
      let ((dist, _), _) =
        shape.spline_distance_orthogonality(segments_range.clone(), point);
      let expected = -1. / 5f32.sqrt();
      assert_approx_eq!(f32, dist, expected);
    }
  }
}
