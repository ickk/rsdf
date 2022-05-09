pub mod spline_colour;
mod svg;

use crate::math::{self, Point};
use crate::shape::*;
use spline_colour::*;

#[derive(Debug)]
pub struct ColouredShape {
  shape: Shape,
  colours: Vec<Vec<SplineColour>>,
  pub corner_rays: Vec<Vec<Point<f32>>>,
}

#[derive(Debug)]
pub struct ColouredContour<'a> {
  contour: &'a Contour,
  colours: &'a Vec<SplineColour>,
}

#[derive(Debug)]
pub struct ColouredSpline<'a> {
  spline: (&'a [EdgeSegment], &'a [Point<f32>]),
  colour: &'a SplineColour,
}

impl ColouredShape {
  pub fn from_shape(shape: Shape) -> Self {
    let shape = colour_shape(shape);
    let shape = compute_corner_rays(shape);

    shape
  }
  pub fn svg(&self) -> String {
    svg::svg(self)
  }
  pub fn contours(&self) -> impl Iterator<Item = ColouredContour> {
    self
      .shape
      .contours()
      .zip(self.colours.iter())
      .map(|(contour, colours)| ColouredContour { contour, colours })
  }
}

impl ColouredContour<'_> {
  pub fn splines(&self) -> impl Iterator<Item = ColouredSpline> {
    self
      .contour
      .splines()
      .into_iter()
      .zip(self.colours.iter())
      .map(|(spline, colour)| ColouredSpline { spline, colour })
  }

  pub fn get_spline(&self, index: usize) -> Option<ColouredSpline> {
    Some(ColouredSpline {
      spline: self.contour.get_spline(index)?,
      colour: self.colours.get(index)?,
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
    corner_rays: vec![], // this is uninitialised! could use an option to memoize value
  }
}

// TODO: merge ColouredShape with shape, and use Option types to memoize data

fn compute_corner_rays(mut coloured_shape: ColouredShape) -> ColouredShape {
  let shape = &coloured_shape.shape;
  for contour in shape.contours() {
    let mut exts = Vec::with_capacity(contour.corners.len());
    for corner in contour.corners.iter() {
      let before = *contour.points.get(corner.1 - 1).or(contour.points.get(contour.points.len()-2))
        .unwrap();
      let after = *contour.points.get(corner.1 + 1).or(contour.points.get(1)).unwrap();
      // Define two vectors, vec_a & vec_b, that extend from `corner` to `before` and from `corner`
      // to `after` respectively.
      let vec_a = before - contour.points[corner.1];
      let vec_b = after - contour.points[corner.1];
      // Create a ray leading out from the corner that divides the plane.
      // the sign of the ray is determined by whether the angle between the vectors is less-than
      // or greater-than pi. This makes the ray always point out from the contour.
      let ray = (vec_a.normalize() + vec_b.normalize()).normalize();
      let direction = 1.0f32.copysign(math::det([vec_a.0, vec_b.0]));

      exts.push(direction * ray);
    }
    coloured_shape.corner_rays.push(exts);
  }
  coloured_shape
}

// TODO: determine directionality of corner_extension. Positive should point out from the contour.
