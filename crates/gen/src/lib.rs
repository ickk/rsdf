pub mod coloured_shape;
pub mod shape;
pub mod math;

pub use coloured_shape::ColouredShape;
pub use shape::Shape;

use shape::EdgeSegment;
use math::Point;

use png;
use std::fs::File;
use std::io::BufWriter;
use std::iter;
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
    debug_assert!(coords[0] < self.width && coords[1] < self.height,
      "coordinates given were outside the dimensions of the image");
    let location = (coords[1]*self.width + coords[0]) * 3;
    self.data[location] = val[0];
    self.data[location+1] = val[1];
    self.data[location+2] = val[2];
  }

  fn flush(self) {
    let mut writer = self.encoder.write_header().unwrap();
    writer.write_image_data(&self.data).unwrap();
  }
}

pub fn do_thing() {
  println!("doing_thing");
  let mut image = Image::new("test_image.png", [3, 4]);
  image.set_pixel([1,1], [255,0,255]);
  image.set_pixel([1,3], [255,255,255]);

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
      left + width * (x as f32 + 0.5)/image_width,
      top + height * (y as f32 + 0.5)/image_height,
    ]
  };

  let shape = ColouredShape::from_shape(shape);

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
const _MAX_COLOUR: u8 = 255;
#[inline]
fn _distance_color(distance: f32) -> u8 {
  ((distance/(2.0*_MAX_DISTANCE) + 0.5)*_MAX_COLOUR as f32) as u8
}

// Gen pixel
fn _algorithm_7(_coords: [f32; 2], shape: &ColouredShape) -> [u8; 3] {
  let mut _distance_red = 256;
  let mut _distance_green = 256;
  let mut _distance_blue = 256;

  for contour in shape.contours() {
    for _spline in contour.splines() {
      // let distance = edge_signed_distance(coords, edge);
    }
  }

  [0, 0, 0]
}

// Signed distance of Point to spline
// fn spline_distance(spline: (&[EdgeSegment], &[Point<f32>])) {}

// fn spline_pseudo_distance() {}

fn _segment_pseudo_distance(segment: (&EdgeSegment, &[Point<f32>]), point: [f32; 2]) {

}

fn _line_distance(line: [Point<f32>; 2], point: Point<f32>) -> f32 {
  // TODO: Impl `math::dot` on Point.
  #[rustfmt::skip]
  let t = (
    math::dot((point - line[0]).0, (line[1] - line[0]).0) /
    math::dot((line[1] - line[0]).0, (line[1] - line[0]).0)
  ).clamp(0.0, 1.0);

  let point_on_line = line[0] + t * (line[1] - line[0]);

  (point - point_on_line).abs()
}

fn _line_pseudo_distance(line: [Point<f32>; 2], point: Point<f32>) -> f32 {
  #[rustfmt::skip]
  let t = math::dot((point - line[0]).0, (line[1] - line[0]).0) /
          math::dot((line[1] - line[0]).0, (line[1] - line[0]).0);

  let point_on_line = line[0] + t * (line[1] - line[0]);

  (point - point_on_line).abs()
}

fn _signed_line_distance() {}
// TODO: impl `Segment` trait with distance(auto impl) & pseudo_distance(manual impl) methods

// TODO: Shape should divide corners as section 2.4 describes.
