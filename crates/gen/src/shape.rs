use crate::*;

pub struct Shape {
  pub contours: Vec<Contour>,
}

impl Shape {
  pub fn sample(&self, position: Point) -> (f32, Channels) {
    // eprintln!("----sample----");
    // let mut min_dist = None;
    // let mut closest_spline = None;
    let mut closest_spline = self.contours.first().unwrap().splines().next().unwrap();
    let mut min_dist = closest_spline.clone().distance_to(position);
    // eprintln!(" min_dist: {min_dist:?}");
    for contour in self.contours.iter() {
      // eprintln!("---contour---");
      // eprintln!("no. splines: {:?}", contour.splines().count());
      for spline in contour.splines() {
        // eprintln!("--spline--");
        // eprintln!("  spline: {spline:?}");
        let dist = spline.distance_to(position);
        // eprintln!("  dist: {dist:?}");
        if dist.dist < min_dist.dist {
        // && (spline.channels & 0b100).as_bool() {
          min_dist = dist;
          closest_spline = spline;
          // eprintln!(" min_dist: {min_dist:?}");
        } else if dist.dist == min_dist.dist {
          if dist.orthogonality > min_dist.orthogonality {
            min_dist = dist;
            closest_spline = spline;
          }
        }
      }
    }
      // Some(spline) => spline.signed_pseudo_distance_to(position),
    (min_dist.sdist, closest_spline.channels)
    // .expect(&format!("Couldn't find closest spline for position: {position:?}"))
    // .signed_pseudo_distance_to(position)
  }
}
