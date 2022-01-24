pub mod colour;

use crate::shape::*;
use colour::{
  Colour::{Magenta, White, Yellow},
  SplineColour,
};

#[derive(Debug)]
pub struct ColouredShape {
  shape: Shape,
  colours: Vec<Vec<SplineColour>>,
}

pub fn colour(shape: Shape) -> ColouredShape {
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
  pub fn svg(self) -> String {
    "".to_string()
  }
}

const WHITE: SplineColour = SplineColour::from_colour(White);
const MAGENTA: SplineColour = SplineColour::from_colour(Magenta);
const YELLOW: SplineColour = SplineColour::from_colour(Yellow);
