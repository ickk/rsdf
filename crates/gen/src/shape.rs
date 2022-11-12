use crate::*;

pub struct Shape {
  pub contours: Vec<Contour>,
}

impl Shape {
  pub fn sample(&self, point: Point) -> [f32; 3] {
    let mut red_distance = f32::INFINITY;
    let mut red_signed_pseudo_distance = f32::INFINITY;
    let mut red_orthogonality = 0.;

    let mut green_distance = f32::INFINITY;
    let mut green_signed_pseudo_distance = f32::INFINITY;
    let mut green_orthogonality = 0.;

    let mut blue_distance = f32::INFINITY;
    let mut blue_signed_pseudo_distance = f32::INFINITY;
    let mut blue_orthogonality = 0.;

    for contour in self.contours.iter() {
      for spline in contour.splines() {
        let spline_distance = spline.distance_to(point);

        if (spline.channels & Channels::new(0b100)).as_bool()
          && (spline_distance.distance.abs() < red_distance
            || spline_distance.distance == red_distance
              && (spline_distance.orthogonality > red_orthogonality))
        {
          red_distance = spline_distance.distance;
          red_orthogonality = spline_distance.orthogonality;
          red_signed_pseudo_distance = spline_distance.signed_pseudo_distance;
        }

        if (spline.channels & Channels::new(0b010)).as_bool()
          && (spline_distance.distance < green_distance
            || spline_distance.distance == green_distance
              && (spline_distance.orthogonality > green_orthogonality))
        {
          green_distance = spline_distance.distance;
          green_orthogonality = spline_distance.orthogonality;
          green_signed_pseudo_distance = spline_distance.signed_pseudo_distance;
        }

        if (spline.channels & Channels::new(0b001)).as_bool()
          && (spline_distance.distance < blue_distance
            || spline_distance.distance == blue_distance
              && (spline_distance.orthogonality > blue_orthogonality))
        {
          blue_distance = spline_distance.distance;
          blue_orthogonality = spline_distance.orthogonality;
          blue_signed_pseudo_distance = spline_distance.signed_pseudo_distance;
        }
      }
    }

    [
      red_signed_pseudo_distance,
      green_signed_pseudo_distance,
      blue_signed_pseudo_distance,
    ]
  }
}
