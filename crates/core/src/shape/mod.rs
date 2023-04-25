pub mod colour;
pub mod contour;
pub use colour::*;
pub use contour::*;

use crate::*;

pub struct Shape {
  pub contours: Vec<Contour>,
}

impl Shape {
  pub fn sample(&self, point: Point) -> [f32; 3] {
    todo!()
  }
}
