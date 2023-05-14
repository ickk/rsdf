#![allow(clippy::new_without_default)]

use rsdf_core::*;

pub struct ShapeBuilder {
  shape: Shape,
}

impl ShapeBuilder {
  pub fn new() -> Self {
    Self {
      shape: Shape {
        points: vec![],
        segments: vec![],
        splines: vec![],
        contours: vec![],
      },
    }
  }

  pub fn build(self) -> Shape {
    self.shape
  }

  pub fn contour(self, start_point: impl Into<Point>) -> ContourBuilder {
    ContourBuilder::new(self.shape, start_point)
  }
}

pub struct ContourBuilder {
  shape: Shape,
  current_spline: Spline,
}

impl ContourBuilder {
  fn new(mut shape: Shape, start_point: impl Into<Point>) -> Self {
    shape.points.push(start_point.into());
    let spline_len = shape.splines.len();
    shape.contours.push(Contour {
      spline_range: spline_len..spline_len,
    });

    let segments_len = shape.segments.len();
    ContourBuilder {
      shape,
      current_spline: Spline {
        segments_range: segments_len..segments_len,
        colour: Colour::Magenta,
      },
    }
  }

  pub fn line(mut self, end_point: impl Into<Point>) -> Self {
    self.shape.points.push(end_point.into());
    self.shape.segments.push(SegmentRef {
      kind: SegmentKind::Line,
      points_index: self.shape.points.len() - 2,
    });
    self.check_for_and_create_new_spline();
    self
  }

  pub fn quadratic_bezier(
    mut self,
    control_point: impl Into<Point>,
    end_point: impl Into<Point>,
  ) -> Self {
    self.shape.points.push(control_point.into());
    self.shape.points.push(end_point.into());
    self.shape.segments.push(SegmentRef {
      kind: SegmentKind::QuadBezier,
      points_index: self.shape.points.len() - 3,
    });
    self.check_for_and_create_new_spline();
    self
  }

  pub fn cubic_bezier(
    mut self,
    control_point_1: impl Into<Point>,
    control_point_2: impl Into<Point>,
    end_point: impl Into<Point>,
  ) -> Self {
    self.shape.points.push(control_point_1.into());
    self.shape.points.push(control_point_2.into());
    self.shape.points.push(end_point.into());
    self.shape.segments.push(SegmentRef {
      kind: SegmentKind::CubicBezier,
      points_index: self.shape.points.len() - 4,
    });
    self.check_for_and_create_new_spline();
    self
  }

  pub fn elliptical_arc(
    mut self,
    rx: f32,
    ry: f32,
    phi: f32,
    large_arc: bool,
    sweep_ccw: bool,
    end: impl Into<Point>,
  ) -> Self {
    let start = *self.shape.points.last().unwrap();
    let end = end.into();
    let endpoint = elliptical_arc::EndpointParam {
      start,
      rx,
      ry,
      phi,
      large_arc,
      sweep_ccw,
      end,
    };
    let centre = elliptical_arc::CentreParam::from(endpoint);
    // centre.delta += 0.01;
    let centre_ps = centre.to_ps();

    self.shape.points.push(centre_ps[0]);
    self.shape.points.push(centre_ps[1]);
    self.shape.points.push(centre_ps[2]);
    self.shape.points.push(centre_ps[3]);
    self.shape.points.push(end);
    self.shape.segments.push(SegmentRef {
      kind: SegmentKind::EllipticalArc,
      points_index: self.shape.points.len() - 5,
    });
    self.check_for_and_create_new_spline();
    self
  }

  pub fn end_contour(mut self) -> ShapeBuilder {
    // finish spline
    self.current_spline.segments_range.end = self.shape.segments.len();
    self.shape.splines.push(self.current_spline.clone());
    let (first_point, last_point) = {
      // TODO: ensure contour is closed
      let first_spline_i =
        self.shape.contours.last().unwrap().spline_range.start;
      let first_segment_i =
        self.shape.splines[first_spline_i].segments_range.start;
      let first_segment = self.shape.segments[first_segment_i];
      let first_point = self.shape.get_segment(first_segment).sample(0f32);
      let segments_len = self.shape.segments.len();
      let last_segment = self.shape.segments[segments_len - 1];
      let last_point = self.shape.get_segment(last_segment).sample(1f32);
      (first_point, last_point)
    };
    let mut shape = if !float_cmp::approx_eq!(Point, first_point, last_point) {
      self.line(first_point).shape
    } else {
      self.shape
    };

    // check to see if the first & last spline are continuous
    // if !self.is_sharp_corner(segments_len - 1, first_segment_i) {
    // todo!() // adjust colour of spline as appropriate
    // }

    let contour = shape.contours.last_mut().unwrap();
    contour.spline_range.end = shape.splines.len();

    ShapeBuilder { shape }
  }

  fn is_sharp_corner(
    &self,
    segment_index_a: usize,
    segment_index_b: usize,
  ) -> bool {
    let segment_a = self.shape.segments[segment_index_a];
    let segment_b = self.shape.segments[segment_index_b];
    let d1 = self
      .shape
      .get_segment(segment_a)
      .sample_derivative(1.0)
      .norm();
    let d2 = self
      .shape
      .get_segment(segment_b)
      .sample_derivative(0.0)
      .norm();
    !float_cmp::approx_eq!(Vector, d1, d2)
  }

  fn check_for_and_create_new_spline(&mut self) {
    let segments_len = self.shape.segments.len();
    // check we even have more than one segment in this spline yet
    if segments_len > self.current_spline.segments_range.start + 1
      && self.is_sharp_corner(segments_len - 2, segments_len - 1)
    {
      // finish old spline
      self.current_spline.segments_range.end = segments_len - 1;
      self.shape.splines.push(self.current_spline.clone());
      // create new spline
      self.current_spline.segments_range = segments_len - 1..segments_len;
      self.current_spline.colour =
        if self.current_spline.colour == Colour::Magenta {
          Colour::Yellow
        } else {
          self.current_spline.colour ^ Colour::Magenta
        }
    }
  }
}
