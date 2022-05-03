pub mod coloured_shape;
pub mod shape;

pub use coloured_shape::ColouredShape;
pub use shape::Shape;

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

fn algorithm_7_gen_pixel(coords: [usize; 2], shape: &ColouredShape) -> [u8; 3] {
  let mut distance_red = 256;
  let mut distance_green = 256;
  let mut distance_blue = 256;

  for contour in shape.contours() {

  }

  [0, 0, 0]
}

fn signed_distance() {}

fn signed_pseudo_distance() {}
