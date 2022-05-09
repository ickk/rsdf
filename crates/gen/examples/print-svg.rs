use sdf_gen::*;

fn main() {
  #[rustfmt::skip]
  let shape = Shape::build()
    .viewbox(-5.0, 105.0, -5.0, 205.0)
    .contour()
      .line(100.0, 0.0)
      .line(100.0, 200.0)
      .line(80.0, 200.0)
      .cubic(100.0, 125.0,
             0.0, 75.0,
             50.0, 25.0)
      .quadratic(0.0, 50.0,
                 35.0, 200.0)
      .line(0.0, 200.0)
    .contour()
      .start(45.0, 145.0)
      .line(45.0, 155.0)
      .line(55.0, 155.0)
      .line(55.0, 145.0)
    .contour()
      .start(45.0, 45.0)
      .line(45.0, 55.0)
      .line(55.0, 55.0)
      .line(55.0, 45.0)
    .finalise();

  eprintln!("{}\n", shape.svg(false));
  eprintln!("{}\n", shape.svg(true));

  eprintln!("corners:");
  for corner in shape.contours().next().unwrap().corners() {
    eprintln!("  {corner:?}");
  }

  eprintln!("splines:");
  for spline in shape.contours().next().unwrap().splines() {
    eprintln!("  {spline:?}");
  }
  eprintln!("");

  eprintln!("!!!");
  let coloured_shape = ColouredShape::from_shape(shape);
  for contour in coloured_shape.contours() {
    eprintln!("{:?}", contour);
    for spline in contour.splines() {
      eprintln!("{:?}", spline);
    }
  }
  eprintln!("!!!");

  for (contour, exts) in coloured_shape.contours().zip(coloured_shape.corner_rays.iter()) {
    eprintln!("");
    for (spline, ext) in contour.splines().zip(exts.iter()) {
      eprintln!("spline: {spline:?}\n ext: {ext:?}");
    }
  }
  eprintln!("\n{}\n", coloured_shape.svg());

  eprintln!("coloured_shape: {coloured_shape:?}\n");

  // eprintln!("{}\n", coloured_shape.svg());

//   let shape2 = Shape::build()
//     .contour()
//       .line(0.0, 0.0)

//     .finalise();

  do_thing();
}
