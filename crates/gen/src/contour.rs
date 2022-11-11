use crate::*;

pub struct Contour {
  pub segments: Vec<Segment>,
  pub corners: Option<Vec<usize>>,
  pub channels: Option<Vec<Channels>>,
  // TODO:
  // Kind: enum{FullySmooth, TearDrop, General}
  // remove corners, add spline_indices
}

impl Contour {
  /// Iterator over splines in the Contour. If the shape is fully smooth then returns an empty
  /// iterator.
  /// Note: Assumes that if there are any corners, then at least one of the corners must be at
  /// segment[0].
  pub fn splines(&self) -> impl Iterator<Item = Spline> {
    use itertools::{izip, Itertools};

    izip!(
      std::iter::once(0)
        .chain(self.corners.as_ref().unwrap().iter().copied())
        .chain(std::iter::once(self.segments.len()))
        .dedup()
        .tuple_windows(),
      self.channels.as_ref().unwrap().iter().copied(),
    )
    .map(|((index, index_1), channels)| Spline {
      segments: &self.segments[index..index_1],
      channels,
    })
  }
}
// remove Memo. Use ShapeBuilder that returns a well-formed Shape on `.finalise()`.

#[cfg(test)]
mod tests {
  use super::*;
  use Segment::*;

  #[test]
  fn contour_splines_empty() {
    let contour = Contour {
      segments: vec![],
      corners: Some(vec![]),
      channels: Some(vec![]),
    };
    let splines = contour.splines().collect::<Vec<_>>();
    let expected: Vec<Spline> = vec![];
    assert_eq!(splines, expected);
  }

  #[test]
  fn contour_splines() {
    let point_a = Point { x: 0.0, y: 0.0 };
    let point_b = Point { x: 1.0, y: 0.0 };
    let point_c = Point { x: 0.5, y: 1.0 };

    let vec_ab = point_a.vector_to(point_b);
    let vec_bc = point_b.vector_to(point_c);
    let vec_ca = point_c.vector_to(point_a);

    let line_ab = Line {
      start: point_a,
      end: point_b,
    };
    let line_bc = Line {
      start: point_b,
      end: point_c,
    };
    let line_ca = Line {
      start: point_c,
      end: point_a,
    };

    let channels_ab: Channels = 0b101.into();
    let channels_bc: Channels = 0b110.into();
    let channels_ca: Channels = 0b011.into();

    let contour = Contour {
      segments: vec![line_ab.clone(), line_bc.clone(), line_ca.clone()],
      corners: Some(vec![0, 1, 2]),
      channels: Some(vec![channels_ab, channels_bc, channels_ca]),
    };

    let slice_ab = &[line_ab][..];
    let slice_bc = &[line_bc][..];
    let slice_ca = &[line_ca][..];

    let splines = contour.splines().collect::<Vec<_>>();
    let expected: Vec<Spline> = vec![
      Spline {
        segments: slice_ab,
        channels: channels_ab,
      },
      Spline {
        segments: slice_bc,
        channels: channels_bc,
      },
      Spline {
        segments: slice_ca,
        channels: channels_ca,
      },
    ];

    assert_eq!(splines, expected);
  }

  #[test]
  fn contour_splines_fully_smooth() {
    // No corners
    let point_a = Point { x: 0.0, y: 0.0 };
    let point_b = Point { x: 1.0, y: 0.0 };
    let point_c = Point { x: 0.5, y: 1.0 };

    let vec_ab = Vector::from_points(point_a, point_b);
    let vec_ca = Vector::from_points(point_c, point_a);

    let line_ab = Line {
      start: point_a,
      end: point_b,
    };
    let line_bc = Line {
      start: point_b,
      end: point_c,
    };
    let line_ca = Line {
      start: point_c,
      end: point_a,
    };

    let channels: Channels = 0b111.into();
    let slice = &[line_ab.clone(), line_bc.clone(), line_ca.clone()][..];

    let contour = Contour {
      segments: vec![line_ab, line_bc, line_ca],
      corners: Some(vec![]),
      channels: Some(vec![channels]),
    };

    let splines = contour.splines().collect::<Vec<_>>();
    let expected: Vec<Spline> = vec![Spline {
      segments: slice,
      channels,
    }];

    assert_eq!(splines, expected);
  }
}
