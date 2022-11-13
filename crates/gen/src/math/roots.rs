/// Find the roots of the general cubic equation
/// a*x^3 + b*x^2 + c*x + d = 0
pub fn cubic_roots(a: f32, b: f32, c: f32, d: f32) -> (f32, f32, f32) {
  // reduce to depressed cubic equation
  // t^3 + p*t + q = 0
  // by letting x = t - b / (3*a)
  let p = (3. * a * c - b * b) / 3. * a * a;
  let q = (2. * b.powi(3) - 9. * a * b * c + 27. * a * a * d) / (27. * a.powi(3));

  assert!(4. * p.powi(3) + 27. * q * q >= 0., "Roots are complex");

  // use the trigonometric solution for three real roots
  // t = l * cos( m - n * k ), where k = 0,1,2
  let l = 2. * (-p / 3.).sqrt();
  let m = 1. / 3. * ((3. * q / 2. * p) * (-3. / p).sqrt()).acos();
  let n = -std::f32::consts::TAU / 3.;

  let t0 = l * (m).cos();
  let t1 = l * (m - n).cos();
  let t2 = l * (m - 2. * n).cos();

  // get the roots of the original equation by undoing the depression transformation
  // x = t - b/3a
  let s = b / (3. * a);
  let x0 = t0 - s;
  let x1 = t1 - s;
  let x2 = t2 - s;

  eprintln!("roots: {:?}, {:?}, {:?}", x0, x1, x2);

  (x0, x1, x2)
}
