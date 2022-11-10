use crate::*;

pub struct Shape {
  pub contours: Vec<Contour>,
}

impl Shape {
  pub fn sample(&self, position: Point) -> [f32; 3] {
    let mut closest_spline = self.contours.first().unwrap().splines().next().unwrap();
    let mut min_dist = closest_spline.clone().distance_to(position);

    let mut red_dist = f32::INFINITY;
    let mut red_sdist = f32::INFINITY;
    let mut red_ortho = 0.;
    let mut red_spline = &self.contours.first().unwrap().splines().next().unwrap();

    let mut green_dist = f32::INFINITY;
    let mut green_sdist = f32::INFINITY;
    let mut green_ortho = 0.;
    let mut green_spline = &self.contours.first().unwrap().splines().next().unwrap();

    let mut blue_dist = f32::INFINITY;
    let mut blue_sdist = f32::INFINITY;
    let mut blue_ortho = 0.;
    let mut blue_spline = &self.contours.first().unwrap().splines().next().unwrap();

    for contour in self.contours.iter() {
      for spline in contour.splines() {
        let dist = spline.distance_to(position);

        if (spline.channels & 0b100.into()).as_bool()
        && (dist.dist < red_dist || dist.dist == red_dist && (dist.orthogonality > red_ortho)) {
          red_dist = dist.dist;
          red_ortho = dist.orthogonality;
          red_sdist = dist.sdist;
          red_spline = &spline;
        }

        if (spline.channels & 0b010.into()).as_bool()
        && (dist.dist < green_dist || dist.dist == green_dist && (dist.orthogonality > green_ortho)) {
          green_dist = dist.dist;
          green_ortho = dist.orthogonality;
          green_sdist = dist.sdist;
          green_spline = &spline;
        }

        if (spline.channels & 0b001.into()).as_bool()
        && (dist.dist < blue_dist || dist.dist == blue_dist && (dist.orthogonality > blue_ortho)) {
          blue_dist = dist.dist;
          blue_ortho = dist.orthogonality;
          blue_sdist = dist.sdist;
          blue_spline = &spline;
        }
      }
    }

    [red_sdist, green_sdist, blue_sdist]
  }
}
