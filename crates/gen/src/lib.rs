pub mod coloured_shape;
pub mod math;
pub mod shape;

pub use coloured_shape::{
  spline_colour::{BLUE, GREEN, RED},
  ColouredShape,
};
pub use shape::Shape;

use math::Point;
use shape::EdgeSegment;

use png;
use std::io::BufWriter;
use std::iter;
use std::{fs::File, mem::discriminant};

use crate::shape::ShapeBuilder;
struct Image<'a> {
  encoder: png::Encoder<'a, BufWriter<File>>,
  data: Vec<u8>,
  width: usize,
  height: usize,
}

impl Image<'_> {
  fn new(path: &str, size: [usize; 2]) -> Self {
    let file = File::create(path).unwrap();
    let buf_writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(buf_writer, size[0] as u32, size[1] as u32);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let data_length = size[0] * size[1] * 3;
    let mut data = Vec::with_capacity(data_length);
    // initialise data
    data.extend(iter::repeat(0).take(data_length));

    Self {
      data,
      encoder,
      width: size[0],
      height: size[1],
    }
  }

  #[inline]
  fn set_pixel(&mut self, coords: [usize; 2], val: [u8; 3]) {
    debug_assert!(
      coords[0] < self.width && coords[1] < self.height,
      "coordinates given were outside the dimensions of the image"
    );
    let location = (coords[1] * self.width + coords[0]) * 3;
    self.data[location] = val[0];
    self.data[location + 1] = val[1];
    self.data[location + 2] = val[2];
  }

  fn flush(self) {
    let mut writer = self.encoder.write_header().unwrap();
    writer.write_image_data(&self.data).unwrap();
  }
}

pub fn do_thing() {
  println!("doing_thing");
  let mut image = Image::new("test_image.png", [3, 4]);
  image.set_pixel([1, 1], [255, 0, 255]);
  image.set_pixel([1, 3], [255, 255, 255]);

  image.flush();

  let shape = Shape::build()
    .viewbox(0.0, 200.0, 0.0, 200.0)
    .contour()
      .start(45.0, 145.0)
      .line(45.0, 155.0)
      .line(55.0, 155.0)
      .line(55.0, 145.0)
    .finalise();

  let mut image = Image::new("test_image2.png", [100, 100]);
  _algorithm_1(&mut image, shape);
  image.flush();
}

// Construct image
fn _algorithm_1(image: &mut Image, shape: Shape) {
  let height = (shape.viewbox.bottom - shape.viewbox.top) as f32;
  let width = (shape.viewbox.right - shape.viewbox.left) as f32;
  let left = shape.viewbox.left as f32;
  let top = shape.viewbox.top as f32;
  let image_width = image.width as f32;
  let image_height = image.height as f32;
  let image_to_shape_coords = |x, y| -> [f32; 2] {
    [
      left + width * (x as f32 + 0.5) / image_width,
      top + height * (y as f32 + 0.5) / image_height,
    ]
  };

  let shape = ColouredShape::from_shape(shape);
  println!("{}", shape.svg());

  for y in 0..image.height {
    for x in 0..image.width {
      image.set_pixel([x, y], _algorithm_7(image_to_shape_coords(x, y), &shape))
    }
  }
}

// Closest Edge
// fn algorithm_2(coords: [f32; 2], shape: &Shape) -> usize {
//   let d_min = f32::INFINITY;
//   let e_min: Option<usize> = None;
//   for contour in shape.contours() {
//     for (s, spline) in contour.splines().enumerate() {
//       distance = spline_signed_distance(coords, )
//     }
//   }
// }

// distanceColor
const _MAX_DISTANCE: f32 = 10.0;
const _MAX_COLOUR: u8 = 127;
#[inline]
fn distance_color(distance: f32) -> u8 {
  ((distance / (2.0 * _MAX_DISTANCE) + 0.5) * _MAX_COLOUR as f32) as u8
}

// Gen pixel
fn _algorithm_7(coords: [f32; 2], shape: &ColouredShape) -> [u8; 3] {
  // eprintln!("!!coords: {coords:?}");
  let mut dist_red = f32::INFINITY;
  let mut spline_red = Some(0);
  let mut contour_red = Some(0);
  let mut dist_green = 100.0; //f32::INFINITY;
  let mut spline_green = Some(0);
  let mut contour_green = Some(0);
  let mut dist_blue = f32::INFINITY;
  let mut spline_blue = Some(0);
  let mut contour_blue = Some(0);

  for (c, contour) in shape.contours().enumerate() {
    for (s, spline) in contour.splines().enumerate() {
      let distance = spline_signed_distance(&spline.spline, coords);
      // let outside_contour =
      if (*spline.colour ^ RED).as_bool()
      && (distance < dist_red)
      && distance > 0.0
      {
        dist_red = distance;
        spline_red = Some(s);
        contour_red = Some(c);
        // eprintln!("c: {c:?}, s: {s:?}");
      }
    //   if (*spline.colour ^ GREEN).as_bool() && (distance < dist_green)
    //   && distance > 0.0 {
    //     dist_green = distance;
    //     spline_green = Some(s);
    //     contour_green = Some(c);
    //     // eprintln!("c: {c:?}, s: {s:?}");
    //   }
    //   if (*spline.colour ^ BLUE).as_bool() && (distance < dist_blue)
    //   && distance > 0.0 {
    //     dist_blue = distance;
    //     spline_blue = Some(s);
    //     contour_blue = Some(c);
    //     // eprintln!("c: {c:?}, s: {s:?}");
    //   }
    }
  }
  // eprintln!("cr: {contour_red:?}, sr: {spline_red:?}");
  // let cr = shape.get_contour(contour_red.unwrap()).unwrap();
  // let sr = cr.get_spline(spline_red.unwrap()).unwrap();
  // eprintln!("cr: {cr:?}, \n sr: {sr:?}");

  dist_red = spline_signed_pseudo_distance(
    &shape
      .get_contour(contour_red.unwrap())
      .unwrap()
      .get_spline(spline_red.unwrap())
      .unwrap()
      .spline,
    Point(coords),
  );
  dist_green = spline_signed_pseudo_distance(
    &shape
      .get_contour(contour_green.unwrap())
      .unwrap()
      .get_spline(spline_green.unwrap())
      .unwrap()
      .spline,
    Point(coords),
  );
  dist_blue = spline_signed_pseudo_distance(
    &shape
      .get_contour(contour_blue.unwrap())
      .unwrap()
      .get_spline(spline_blue.unwrap())
      .unwrap()
      .spline,
    Point(coords),
  );
  [distance_color(dist_red), 0, 0]
}

// fn cmp() {}

// Signed distance of Point to spline
fn spline_signed_distance(spline: &(&[EdgeSegment], &[Point<f32>]), coords: [f32; 2]) -> f32 {
  let mut points = spline.1.iter();
  let mut next_point = points.next().unwrap();

  let mut dist = f32::INFINITY;
  for segment in spline.0 {
    match segment {
      EdgeSegment::Line => {
        let point = next_point;
        next_point = points.next().unwrap();
        let dist_to_line = _signed_line_distance([*point, *next_point], Point(coords));
        if dist_to_line < dist {
          dist = dist_to_line;
        }
      },
      EdgeSegment::Quadratic => unimplemented!(),
      EdgeSegment::Cubic => unimplemented!(),
    }
  }

  dist
}

fn spline_signed_pseudo_distance(
  spline: &(&[EdgeSegment], &[Point<f32>]),
  pnt: Point<f32>,
) -> f32 {
  let mut points = spline.1.iter();
  let mut next_point = points.next();

  let mut dist = f32::INFINITY;
  for segment in spline.0 {
    match segment {
      EdgeSegment::Line => {
        let point = *next_point.unwrap();
        next_point = points.next();
        // eprintln!("segment: {segment:?}, point: {point:?}, next_point: {next_point:?}");
        let dist_to_line = _signed_line_pseudo_distance([point, *next_point.unwrap()], pnt);
        if dist_to_line < dist {
          dist = dist_to_line;
        }
      },
      EdgeSegment::Quadratic => unimplemented!(),
      EdgeSegment::Cubic => unimplemented!(),
    }
  }

  dist
}

fn _signed_line_distance(line: [Point<f32>; 2], point: Point<f32>) -> f32 {
  // TODO: Impl `math::dot` on Point.
  #[rustfmt::skip]
  let t = (
    math::dot((point - line[0]).0, (line[1] - line[0]).0) /
    math::dot((line[1] - line[0]).0, (line[1] - line[0]).0)
  );
  if t > 1.0 {

  }
  let t = t.clamp(0.0, 1.0);
  let point_on_line = line[0] + t * (line[1] - line[0]);
  let dist = (point - point_on_line).abs();

  let vec_t = line[1] - line[0];
  let vec_p = point - point_on_line;
  let sign = 1.0f32.copysign(math::det([vec_t.0, vec_p.0]));

  sign * dist
}

fn _signed_line_pseudo_distance(line: [Point<f32>; 2], point: Point<f32>) -> f32 {
  #[rustfmt::skip]
  let t = math::dot((point - line[0]).0, (line[1] - line[0]).0) /
          math::dot((line[1] - line[0]).0, (line[1] - line[0]).0);
  let point_on_line = line[0] + t * (line[1] - line[0]);
  let dist = (point - point_on_line).abs();

  let vec_t = line[1] - line[0];
  let vec_p = point - point_on_line;
  let sign = 1.0f32.copysign(math::det([vec_t.0, vec_p.0]));

  sign * dist
}

// TODO: impl `Segment` trait with distance(auto impl) & pseudo_distance(manual impl) methods
