use sdf_gen::*;

fn main() {
  #[rustfmt::skip]
  let shape = Shape::build()
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

  eprintln!("{}\n", shape.svg(true));

  eprintln!("corners:");
  for corner in shape.contours().iter().next().unwrap().corners() {
    eprintln!("  {corner:?}");
  }

  eprintln!("splines:");
  for spline in shape.contours().iter().next().unwrap().splines() {
    eprintln!("  {spline:?}");
  }
  eprintln!("");

  let coloured_shape = colour(shape);

  eprintln!("coloured_shape: {coloured_shape:?}\n");

  eprintln!("colours: {:?}\n", coloured_shape.svg());

}
