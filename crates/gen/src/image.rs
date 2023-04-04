use std::fs::File;
use std::io::BufWriter;

pub struct Image<'a> {
  encoder: png::Encoder<'a, BufWriter<File>>,
  data: Vec<u8>,
  pub width: usize,
  pub height: usize,
}

impl Image<'_> {
  pub fn new(path: &str, size: [usize; 2]) -> Self {
    let file = File::create(path).unwrap();
    let buf_writer = BufWriter::new(file);
    let mut encoder =
      png::Encoder::new(buf_writer, size[0] as u32, size[1] as u32);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let data_length = size[0] * size[1] * 3;
    let data = vec![0; data_length];

    Self {
      data,
      encoder,
      width: size[0],
      height: size[1],
    }
  }

  #[inline]
  pub fn set_pixel(&mut self, coords: [usize; 2], val: [u8; 3]) {
    debug_assert!(
      coords[0] < self.width && coords[1] < self.height,
      "coordinates given were outside the dimensions of the image"
    );
    let location = (coords[1] * self.width + coords[0]) * 3;
    self.data[location] = val[0];
    self.data[location + 1] = val[1];
    self.data[location + 2] = val[2];
  }

  pub fn flush(self) {
    let mut writer = self.encoder.write_header().unwrap();
    writer.write_image_data(&self.data).unwrap();
  }
}
