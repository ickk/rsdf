use crate::math::{self, Point};
use crate::Image;
use crate::shape::{
  Shape,
  EdgeSegment,
};
use crate::coloured_shape::{
  ColouredShape,
  ColouredSpline,
  spline_colour::{BLUE, GREEN, RED},
};

  // let shape = Shape::build()
  //   .viewbox(0.0, 200.0, 0.0, 200.0)
  //   .contour()
  //   .start(50.0, 50.0)
  //   .line(150.0, 50.0)
  //   .line(150.0, 150.0)
  //   .line(50.0, 150.0)
  //   .finalise();

pub fn construct_image(image: &mut Image, shape: Shape) {
  let height = (shape.viewbox.bottom - shape.viewbox.top) as f32;
  let width = (shape.viewbox.right - shape.viewbox.left) as f32;
  let left = shape.viewbox.left as f32;
  let top = shape.viewbox.top as f32;
  let image_width = image.width as f32;
  let image_height = image.height as f32;
  let image_to_shape_coords = |x, y| -> Point<f32> {
    Point([
      left + width * (x as f32 + 0.5) / image_width,
      top + height * (y as f32 + 0.5) / image_height,
    ])
  };

  let shape = ColouredShape::from_shape(shape);
  println!("{}", shape.svg());

  for y in 0..image.height {
    for x in 0..image.width {
      let point = image_to_shape_coords(x, y);
      image.set_pixel([x, y], generate_pixel(point, &shape));
    }
  }
}

fn generate_pixel(point: Point<f32>, shape: &ColouredShape) -> [u8; 3] {
  let first_contour = shape.contours().next().unwrap();
  let first_corner_rays = shape.corner_rays().next().unwrap();

  let mut red_dist = f32::INFINITY;
  let mut red_spline = None;
  let mut green_dist = f32::INFINITY;
  let mut green_spline = None;
  let mut blue_dist = f32::INFINITY;
  let mut blue_spline = None;
  for (ColouredSpline{spline, colour}, corner_rays) in first_contour.splines()
  .zip(first_corner_rays) {
    let dist = spline_sdistance(&spline, point, corner_rays);
    // if (*colour & RED).as_bool() {
      if dist.abs() < red_dist.abs() {
        red_dist = dist;
        red_spline = Some(spline);
      }
    // }
    if (*colour & GREEN).as_bool() {
      if dist.abs() < green_dist.abs() {
        green_dist = dist;
        green_spline = Some(spline);
      }
    }
    if (*colour & BLUE).as_bool() {
      if dist.abs() < blue_dist.abs() {
        blue_dist = dist;
        blue_spline = Some(spline);
      }
    }
  }

  // [dist_color(red_dist), dist_color(green_dist), dist_color(blue_dist)]
  [dist_color(red_dist), 0, 0]
}

// distanceColor
const MAX_DISTANCE: f32 = 15.0;
const MAX_COLOUR: f32 = 256.0;
#[inline]
fn dist_color(distance: f32) -> u8 {
  let distance = distance.clamp(-MAX_DISTANCE, MAX_DISTANCE);
  (( (distance+MAX_DISTANCE) / (2.0*MAX_DISTANCE) * MAX_COLOUR) - 1.0) as u8
}

// Signed distance of Point to spline
fn spline_sdistance(
  spline: &(&[EdgeSegment], &[Point<f32>]),
  point: Point<f32>,
  corner_rays: (&Point<f32>, &Point<f32>),
) -> f32 {
  let mut ps = spline.1.iter();
  let mut next_p = ps.next().unwrap();

  let mut smallest_dist = f32::INFINITY;
  for (i, segment) in spline.0.iter().enumerate() {
    match segment {
      EdgeSegment::Line => {
        let p = next_p;
        next_p = ps.next().unwrap();
        let dist_to_line = line_sdistance([*p, *next_p], point);
        if dist_to_line.abs() < smallest_dist.abs() {
          // if NOT (first segment AND point is left of the corner-ray
          // OR if last segment AND point is right of the corner-ray)
          if !(i == spline.0.len()-1 && math::det([(point-*p).0, (*corner_rays.0-*p).0]) <= 0.0
          || i == 0 && math::det([(*corner_rays.1-*next_p).0, (point-*next_p).0]) < 0.0) {
            smallest_dist = dist_to_line;
          }
        }
      },
      EdgeSegment::Quadratic => unimplemented!(),
      EdgeSegment::Cubic => unimplemented!(),
    }
  }

  smallest_dist
}
fn line_sdistance(
  line: [Point<f32>; 2],
  point: Point<f32>,
) -> f32 {
  #[rustfmt::skip]
  let mut t = math::dot((point - line[0]).0, (line[1] - line[0]).0) /
              math::dot((line[1] - line[0]).0, (line[1] - line[0]).0);
  t = t.clamp(0.0, 1.0);
  let point_on_line = line[0] + t * (line[1] - line[0]);
  let dist = (point - point_on_line).abs();

  let vec_t = line[1] - line[0];
  let vec_p = point - point_on_line;
  let sign = 1.0f32.copysign(math::det([vec_t.0, vec_p.0]));

  sign * dist
}

fn pseudo_line_sdistance(
  line: [Point<f32>; 2],
  point: Point<f32>,
) -> f32 {
    #[rustfmt::skip]
  let mut t = math::dot((point - line[0]).0, (line[1] - line[0]).0) /
              math::dot((line[1] - line[0]).0, (line[1] - line[0]).0);
  let point_on_line = line[0] + t * (line[1] - line[0]);
  let dist = (point - point_on_line).abs();

  let vec_t = line[1] - line[0];
  let vec_p = point - point_on_line;
  let sign = 1.0f32.copysign(math::det([vec_t.0, vec_p.0]));

  sign * dist
}

// unit tests
// given edge, point -> get distance
// given edge, point -> get pseudodistance
