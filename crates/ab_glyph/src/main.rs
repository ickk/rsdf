use ::ab_glyph::{Font, FontRef, Glyph};
use ::rsdf_ab_glyph::FontExtRsdf;
use ::rsdf_core::Image;

fn main() {
  let font =
    FontRef::try_from_slice(include_bytes!("../../../fonts/Noto/NotoSerifJP-Regular.otf"))
      .unwrap();

  let glyph: Glyph = font.glyph_id('及').with_scale(96.0);

  if let Some(q) = font.outline_glyph(glyph.clone()) {
    let bounds = q.px_bounds();

    let mut image = Image::new(
      "ab_glyph_example_reference.png",
      [bounds.width() as usize + 10, bounds.height() as usize + 10],
    );

    q.draw(|x, y, coverage| {
      image.set_pixel(
        [x as usize + 5, y as usize + 5],
        [(coverage * 255.0) as _; 3],
      )
    });

    image.flush()
  }

  if let Some(q) = font.outline_glyph_rsdf(glyph) {
    let bounds = q.px_bounds();

    let mut image = Image::new(
      "ab_glyph_example.png",
      [bounds.width() as usize + 10, bounds.height() as usize + 10],
    );
    let mut image_render = Image::new(
      "ab_glyph_example_render.png",
      [bounds.width() as usize + 10, bounds.height() as usize + 10],
    );

    q.draw(|x, y, field| {
      image.set_pixel([x as usize, y as usize], field);

      // find the median value
      let median = |a, b, c| {
        if (a <= b && b <= c) || (c <= b && b <= a) {
          b
        } else if (a <= c && c <= b) || (b <= c && c <= a) {
          c
        } else {
          a
        }
      };
      let value = median(field[0], field[1], field[2]);
      if value > 127 {
        image_render.set_pixel([x as usize, y as usize], [255; 3]);
      }
    });

    image.flush();
    image_render.flush();
  }
}
