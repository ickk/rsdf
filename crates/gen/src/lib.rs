#![allow(unused)]

mod math;
mod segment;
mod memo;
mod channels;
mod contour;
mod shape;
mod spline;
mod image;
pub use math::*;
pub use segment::*;
pub use memo::*;
pub use channels::*;
pub use contour::*;
pub use shape::*;
pub use spline::*;
pub use image::*;

// distanceColor
// TODO: cleanup, unit test
const MAX_DISTANCE: f32 = 10.0;
const MAX_COLOUR: f32 = 256.0;
#[inline]
fn distance_color(distance: f32) -> u8 {
  let distance = distance.clamp(-MAX_DISTANCE, MAX_DISTANCE);
  (( (distance+MAX_DISTANCE) / (2.0*MAX_DISTANCE) * MAX_COLOUR) - 1.0) as u8
}

pub fn do_thing() {
  use Segment::*;

  println!("doing_thing");

  let point_a = Point {x: 1.0, y: 1.0};
  let point_b = Point {x: 9.0, y: 1.0};
  let point_c = Point {x: 5.0, y: 9.0};

  let vec_ab = point_a.vector_to(point_b);
  let vec_bc = point_b.vector_to(point_c);
  let vec_ca = point_c.vector_to(point_a);

  let ray_a = (vec_ca.norm() + -vec_ab.norm()).norm();
  let ray_b = (vec_ab.norm() + -vec_bc.norm()).norm();
  let ray_c = (vec_bc.norm() + -vec_ca.norm()).norm();

  let line_ab = Line {start: point_a, end: point_b};
  let line_bc = Line {start: point_b, end: point_c};
  let line_ca = Line {start: point_c, end: point_a};

  let channels_ab: Channels = 0b101.into();
  let channels_bc: Channels = 0b110.into();
  let channels_ca: Channels = 0b011.into();

  let corner_rays_ab = CornerRays {start: ray_a, end: ray_b};
  let corner_rays_bc = CornerRays {start: ray_b, end: ray_c};
  let corner_rays_ca = CornerRays {start: ray_c, end: ray_a};

  let contour = Contour {
    segments: vec![line_ab.clone(), line_bc.clone(), line_ca.clone()],
    corners: Memo::Value(vec![0, 1, 2]),
    corner_rays: Memo::Value(vec![
      corner_rays_ab.clone(), corner_rays_bc.clone(), corner_rays_ca.clone()
    ]),
    channels: Memo::Value(vec![channels_ab, channels_bc, channels_ca]),
  };

  let shape = Shape {
    contours: vec![contour],
  };

  let mut image = Image::new("test_image.png", [10, 10]);

  for y in 0..image.height {
    for x in 0..image.width {
      let point: Point = (x as f32, y as f32).into();
      let val = shape.sample(point);
      let val = distance_color(val);
      image.set_pixel([x, y], [val, 0, 0]);
    }
  }

  image.flush();

  panic!()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn do_thingy() {
    do_thing()
  }
}
