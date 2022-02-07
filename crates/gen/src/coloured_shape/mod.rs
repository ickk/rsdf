pub mod spline_colour;
mod svg;

use crate::shape::*;
use spline_colour::*;

#[derive(Debug)]
pub struct ColouredShape {
  shape: Shape,
  colours: Vec<Vec<SplineColour>>,
}

pub fn colour_shape(shape: Shape) -> ColouredShape {
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

impl ColouredShape {
  pub fn from_shape(shape: Shape) -> Self {
    colour_shape(shape)
  }
  pub fn svg(&self) -> String {
    svg::svg(self)
  }
  fn _splines(&self) -> () {
    // TODO: I think this requires zip, which is nightly-only :(
    unimplemented!()
  }
}

