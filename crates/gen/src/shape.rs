use crate::*;

pub struct Shape {
  pub contours: Vec<Contour>,
}

impl Shape {
  pub fn sample(&self, position: Point) -> f32 {
    // let mut min_dist = f32::INFINITY;
    // let mut closest_spline = None;
    let mut closest_spline = Some(self.contours.first().unwrap().splines().next().unwrap());
    let mut min_dist = closest_spline.clone().unwrap().distance_to(position);
    for contour in self.contours.iter() {
      for spline in contour.splines() {
        eprintln!("spline: {spline:?}");
        let dist = spline.distance_to(position);
        eprintln!("dist: {dist}");
        if dist < min_dist
        // && (spline.channels & 0b100).as_bool() {
        {
          min_dist = dist;
          closest_spline = Some(spline);
        }
      }
    }
    match closest_spline {
      // Some(spline) => spline.signed_pseudo_distance_to(position),
      Some(spline) => spline.distance_to(position),
      _ => f32::INFINITY,
    }
    // .expect(&format!("Couldn't find closest spline for position: {position:?}"))
    // .signed_pseudo_distance_to(position)
  }
}
