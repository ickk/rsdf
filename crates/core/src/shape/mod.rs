pub mod colour;
pub mod contour;
pub mod primitives;
pub use colour::{Colour, Colour::*};
pub use contour::*;
pub use primitives::*;

use crate::*;

pub struct Shape {
  pub contours: Vec<Contour>,
}

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
    let mut blue_spline = None;

    let mut red_pseudo_dist = -f32::INFINITY;
    let mut green_pseudo_dist = -f32::INFINITY;
    let mut blue_pseudo_dist = -f32::INFINITY;

    // for contour in self.contours.iter() {
    if let Some(contour) = self.contours.first() {
      for spline in contour.splines() {
        let (dist, orth) = contour.spline_distance(spline, point);

        if (spline.colour.unwrap() & Red == Red)
          && (dist.abs() < red_dist.abs()
            || (dist.abs() == red_dist.abs() && orth.abs() >= red_orth))
        {
          red_dist = dist;
          red_orth = orth.abs();
          red_spline = Some(spline);
        }

        if (spline.colour.unwrap() & Green == Green)
          && (dist.abs() < green_dist.abs()
            || (dist.abs() == green_dist.abs() && orth.abs() >= green_orth))
        {
          green_dist = dist;
          green_orth = orth.abs();
          // }
          green_spline = Some(spline);
        }

        if (spline.colour.unwrap() & Blue == Blue)
          && (dist.abs() < blue_dist.abs()
            || (dist.abs() == blue_dist.abs() && orth.abs() >= blue_orth))
        {
          blue_dist = dist;
          blue_orth = orth.abs();
          blue_spline = Some(spline);
        }
      }

      red_pseudo_dist = red_spline.map_or(-f32::INFINITY, |spline| {
        contour.spline_pseudo_distance(spline, point)
      });
      green_pseudo_dist = green_spline.map_or(-f32::INFINITY, |spline| {
        contour.spline_pseudo_distance(spline, point)
      });
      blue_pseudo_dist = blue_spline.map_or(-f32::INFINITY, |spline| {
        contour.spline_pseudo_distance(spline, point)
      });
    }

    [
      // red_dist,
      // green_dist,
      // blue_dist,
      // red_orth * 10.0,
      // green_orth * 10.0,
      // blue_orth * 10.0,
      red_pseudo_dist,
      green_pseudo_dist,
      blue_pseudo_dist,
    ]
  }
}
