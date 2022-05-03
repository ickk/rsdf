pub mod spline_colour;
mod svg;

use crate::shape::*;
use spline_colour::*;

#[derive(Debug)]
pub struct ColouredShape {
  shape: Shape,
  colours: Vec<Vec<SplineColour>>,
}

#[derive(Debug)]
pub struct ColouredContour<'a> {
  contour: &'a Contour,
  colours: &'a Vec<SplineColour>,
}

#[derive(Debug)]
pub struct ColouredSpline<'a> {
  spline: (&'a [EdgeSegment], &'a [Point<f32>]),
  colours: &'a SplineColour,
}

impl ColouredShape {
  pub fn from_shape(shape: Shape) -> Self {
    colour_shape(shape)
  }
  pub fn svg(&self) -> String {
    svg::svg(self)
  }
  pub fn contours(&self) -> impl Iterator<Item = ColouredContour> {
    self.shape.contours().zip(self.colours.iter())
      .map(|(contour, colours)| {
        ColouredContour { contour, colours }
      })
  }
}

impl ColouredContour<'_> {
  pub fn splines(&self) -> impl Iterator<Item = ColouredSpline> {
    self.contour.splines().into_iter().zip(self.colours.iter())
      .map(|(spline, colours)| {
        ColouredSpline { spline, colours }
      })
  }
}

fn colour_shape(shape: Shape) -> ColouredShape {
  let mut shape_colours: Vec<Vec<SplineColour>> = vec![];

  for contour in shape.contours() {
    let mut colours: Vec<SplineColour> = Vec::with_capacity(contour.corners.len());

    match contour.corners.len() {
      0 => {
        // fully smooth shape
        colours.push(WHITE);
      },
      1 => {
        // TODO: teardrop special case
        unimplemented!()
      },
      _ => {
        // General case; there's at least two edges
        colours.push(MAGENTA);
        let mut current = YELLOW;
        for _spline in contour.splines().iter().skip(1) {
          colours.push(current);
          current ^= MAGENTA;
        }
      },
    }

    shape_colours.push(colours);
  }

  ColouredShape {
    shape,
    colours: shape_colours,
  }
}
