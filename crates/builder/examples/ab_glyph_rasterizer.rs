use ab_glyph_rasterizer::Rasterizer;
use rsdf_core::Image;

fn main() {
  let mut raster = Rasterizer::new(97, 86);

  // upper-right of heart
  raster.draw_cubic(
    (48.0, 14.5).into(),
    (54., 0.5).into(),
    (80.5, 2.).into(),
    (89.0, 17.5).into(),
  );
  // letter 'f'
  raster.draw_cubic(
    (89.0, 17.5).into(),
    (81., 17.).into(),
    (77., 20.).into(),
    (77., 24.).into(),
  );
  raster.draw_line((77., 24.).into(), (77., 28.).into());
  raster.draw_line((77., 28.).into(), (73., 28.).into());
  raster.draw_line((73., 28.).into(), (73., 33.).into());
  raster.draw_line((73., 33.).into(), (77., 33.).into());
  raster.draw_line((77., 33.).into(), (77., 48.).into());
  raster.draw_line((77., 48.).into(), (84., 48.).into());
  raster.draw_line((84., 48.).into(), (84., 33.).into());
  raster.draw_line((84., 33.).into(), (90., 33.).into());
  raster.draw_line((90., 33.).into(), (91., 28.).into());
  raster.draw_line((91., 28.).into(), (84., 28.).into());
  raster.draw_line((84., 28.).into(), (84., 25.).into());
  raster.draw_cubic(
    (84., 25.).into(),
    (84., 23.).into(),
    (87., 21.5).into(),
    (91., 23.).into(),
  );
  // lower-portion of heart
  raster.draw_cubic(
    (91., 23.).into(),
    (94.5, 33.).into(),
    (93.5, 40.).into(),
    (85.5, 48.7).into(),
  );
  raster.draw_line((85.5, 48.7).into(), (48., 80.).into());
  raster.draw_line((48., 80.).into(), (10., 48.).into());
  //     // letter 'r'
  raster.draw_line((10., 48.).into(), (20., 48.).into());
  raster.draw_line((20., 48.).into(), (20., 44.).into());
  raster.draw_line((20., 44.).into(), (15., 44.).into());
  raster.draw_line((15., 44.).into(), (15., 38.).into());
  raster.draw_quad((15., 38.).into(), (17., 33.).into(), (21., 32.).into());
  raster.draw_line((21., 32.).into(), (21., 36.).into());
  raster.draw_line((21., 36.).into(), (25., 36.).into());
  raster.draw_line((25., 36.).into(), (26., 27.).into());
  raster.draw_quad((26., 27.).into(), (18., 24.5).into(), (14.2, 31.).into());
  raster.draw_line((14.2, 31.).into(), (13.5, 27.).into());
  raster.draw_line((13.5, 27.).into(), (6., 27.).into());
  raster.draw_line((6., 27.).into(), (6., 31.).into());
  raster.draw_line((6., 31.).into(), (9., 31.5).into());
  raster.draw_line((9., 31.5).into(), (9., 44.).into());
  raster.draw_line((9., 44.).into(), (7., 44.).into());
  // upper-left of heart
  raster.draw_cubic(
    (7., 44.).into(),
    (0.5, 33.).into(),
    (0.5, 20.).into(),
    (9., 12.).into(),
  );
  raster.draw_cubic(
    (9., 12.).into(),
    (20., 1.).into(),
    (40., 0.5).into(),
    (48., 14.5).into(),
  );
  // letter 's'
  raster.draw_line((44., 34.).into(), (46., 29.).into());
  raster.draw_cubic(
    (46., 29.).into(),
    (38., 23.).into(),
    (28.5, 26.).into(),
    (28.5, 33.).into(),
  );
  raster.draw_quad((28.5, 33.).into(), (29., 38.).into(), (35.5, 39.).into());
  raster.draw_quad((35.5, 39.).into(), (40., 39.7).into(), (40., 42.).into());
  raster.draw_cubic(
    (40., 42.).into(),
    (40., 44.3).into(),
    (34., 45.).into(),
    (30., 41.5).into(),
  );
  raster.draw_line((30., 41.5).into(), (27., 46.).into());
  raster.draw_cubic(
    (27., 46.).into(),
    (34., 51.).into(),
    (46.5, 50.).into(),
    (47., 42.).into(),
  );
  raster.draw_quad((47., 42.).into(), (47., 37.5).into(), (40.5, 35.5).into());
  raster.draw_quad((40.5, 35.5).into(), (34., 34.5).into(), (35., 32.).into());
  raster.draw_cubic(
    (35., 32.).into(),
    (35.5, 30.).into(),
    (40., 30.).into(),
    (44., 34.).into(),
  );
  // letter 'd'
  raster.draw_line((63., 46.5).into(), (63., 48.).into());
  raster.draw_line((63., 48.).into(), (69.5, 48.).into());
  raster.draw_line((69.5, 48.).into(), (69.5, 18.5).into());
  raster.draw_line((69.5, 18.5).into(), (63., 17.).into());
  raster.draw_line((63., 17.).into(), (63., 27.).into());
  raster.draw_quad((63., 27.).into(), (61., 25.6).into(), (57.5, 26.).into());
  raster.draw_cubic(
    (57.5, 26.).into(),
    (47.5, 26.5).into(),
    (47.5, 47.).into(),
    (57.5, 49.).into(),
  );
  raster.draw_quad((57.5, 49.).into(), (61., 49.).into(), (63., 46.5).into());
  raster.draw_cubic(
    (63., 41.).into(),
    (61., 45.5).into(),
    (56., 43.).into(),
    (56., 38.).into(),
  );
  raster.draw_cubic(
    (56., 38.).into(),
    (56., 31.).into(),
    (61., 30.).into(),
    (63., 33.5).into(),
  );
  raster.draw_line((63., 33.5).into(), (63., 41.).into());

  // output
  let output_filename = "ab_glyph_rasterizer.png";
  let mut image = Image::new(&output_filename, [97, 86]);
  raster.for_each_pixel_2d(|x, y, alpha| {
    let value = (alpha * 255.0) as u8;
    let (mut r_output, mut g_output, mut b_output) = (13, 17, 23);
    if value > 123 {
      r_output = 255;
      g_output = 255;
      b_output = 255;
    }
    image.set_pixel([x as _, y as _], [r_output, g_output, b_output]);
  });
  image.flush();
}
