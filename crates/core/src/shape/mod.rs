pub mod colour;
pub mod contour;
pub use colour::{Colour, Colour::*};
pub use contour::*;

use crate::*;

pub struct Shape {
  pub contours: Vec<Contour>,
}
// /*
impl Shape {
  pub fn sample(&self, point: Point) -> [f32; 3] {
    let mut red_dist = f32::INFINITY;
    let mut red_orth = 0.;
    let mut red_spline = None;

    let mut green_dist = f32::INFINITY;
    let mut green_orth = 0.;
    let mut green_spline = None;

    let mut blue_dist = f32::INFINITY;
    let mut blue_orth = 0.;

    let mut red_pseudo_dist = None;
    let mut green_pseudo_dist = None;

    for contour in self.contours.iter() {
      for spline in contour.splines() {
        let (dist, orth) = contour.spline_distance(spline, point);

        if (spline.colour.unwrap() & Red) != Black
          && (dist.abs() < red_dist.abs()
            || dist == red_dist && (orth > red_orth))
        {
          red_dist = dist;
          red_orth = orth;
          red_spline = Some(spline);
        }

        if (spline.colour.unwrap() & Green) != Black
          && (dist.abs() < green_dist.abs()
            || dist == green_dist && (orth > green_orth))
        {
          green_dist = dist;
          green_orth = orth;
          green_spline = Some(spline);
        }
      }

      red_pseudo_dist =
        Some(contour.spline_pseudo_distance(red_spline.unwrap(), point));
      green_pseudo_dist =
        Some(contour.spline_pseudo_distance(green_spline.unwrap(), point));
    }

    // [red_dist, green_dist, -f32::INFINITY]

    [
      red_pseudo_dist.unwrap(),
      green_pseudo_dist.unwrap(),
      -f32::INFINITY,
    ]
  }
}
// */
