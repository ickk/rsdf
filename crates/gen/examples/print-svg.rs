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
      .quadratic(0.0, 100.0,
                 20.0, 200.0)
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

  eprintln!("{}", shape.svg());

  #[rustfmt::skip]
  let shape_a = Shape::build()
    .viewbox(0.0, 100.0, 0.0, 100.0)
    .contour()
      .start(81.0, 83.0)
      .line(63.0, 84.0)
      .line(60.0, 70.0)
      .line(41.0, 70.0)
      .line(39.0, 83.0)
      .line(21.0, 83.0)
      .line(40.0, 17.0)
      .line(61.0, 17.0)
    .contour()
      .start(57.0, 58.0)
      .line(51.0, 30.0)
      .line(44.0, 58.0)
    .finalise();

  eprintln!("{}", shape_a.svg());
}
