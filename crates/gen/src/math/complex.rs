#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Complex {
  real: f32,
  imaginary: f32,
}

impl Complex {
  pub const J: Self = Complex {
    real: 0.,
    imaginary: 1.,
  };

  #[inline]
  pub const fn new(real: f32, imaginary: f32) -> Self {
    Complex { real, imaginary }
  }

  #[inline]
  pub fn from_polar(modulus: f32, argument: f32) -> Self {
    Complex {
      real: modulus * argument.cos(),
      imaginary: modulus * argument.sin(),
    }
  }

  #[inline]
  pub const fn j() -> Self {
    Self::J
  }

  #[inline]
  pub fn conjugate(self) -> Self {
    Complex {
      real: self.real,
      imaginary: -self.imaginary,
    }
  }

  #[inline]
  pub fn square(self) -> Self {
    Complex {
      real: self.real * self.real - self.imaginary * self.imaginary,
      imaginary: 2. * self.real * self.imaginary,
    }
  }

  #[inline]
  pub fn cube(self) -> Self {
    let re_2 = self.real * self.real;
    let im_2 = self.imaginary * self.imaginary;
    Complex {
      real: self.real * re_2 - 3. * self.real * im_2,
      imaginary: 3. * re_2 * self.imaginary - im_2 * self.imaginary,
    }
  }

  #[inline]
  pub fn reciprocal(self) -> Self {
    let denominator = self.real * self.real + self.imaginary * self.imaginary;
    Complex {
      real: self.real / denominator,
      imaginary: -self.imaginary / denominator,
    }
  }

  #[inline]
  pub const fn real(self) -> f32 {
    self.real
  }

  #[inline]
  pub const fn imaginary(self) -> f32 {
    self.imaginary
  }

  #[inline]
  pub fn abs(self) -> f32 {
    self.modulus()
  }

  #[inline]
  pub fn modulus(self) -> f32 {
    (self.real * self.real + self.imaginary * self.imaginary).sqrt()
  }

  #[inline]
  pub fn arg(self) -> f32 {
    f32::atan2(self.imaginary, self.real)
  }

  #[inline]
  pub fn sqrt(self) -> Self {
    let half_modulus = 0.5 * self.modulus();
    let half_real = 0.5 * self.real;

    Complex {
      real: (half_real + half_modulus).sqrt(),
      imaginary: (half_modulus - half_real).sqrt().copysign(self.imaginary),
    }
  }

  #[inline]
  pub fn cbrt(self) -> Self {
    let mod_cbrt = self.modulus().cbrt();
    let arg_div_n = self.arg() / 3.;

    Complex {
      real: mod_cbrt * arg_div_n.cos(),
      imaginary: mod_cbrt * arg_div_n.sin(),
    }
  }

  #[inline]
  pub fn powi(self, power: i32) -> Self {
    let mod_pow_n = self.modulus().powi(power);
    let arg_mul_n = self.arg() * power as f32;
    Complex {
      real: mod_pow_n * arg_mul_n.cos(),
      imaginary: mod_pow_n * arg_mul_n.sin(),
    }
  }
}

impl From<f32> for Complex {
  #[inline]
  fn from(real: f32) -> Self {
    Complex {
      real,
      imaginary: 0.,
    }
  }
}

impl std::ops::Add<Complex> for Complex {
  type Output = Self;

  #[inline]
  fn add(self, rhs: Self) -> Complex {
    Complex {
      real: self.real + rhs.real,
      imaginary: self.imaginary + rhs.imaginary,
    }
  }
}

impl std::ops::Add<f32> for Complex {
  type Output = Self;

  #[inline]
  fn add(self, rhs: f32) -> Complex {
    Complex {
      real: self.real + rhs,
      imaginary: self.imaginary,
    }
  }
}

impl std::ops::Add<Complex> for f32 {
  type Output = Complex;

  #[inline]
  fn add(self, rhs: Complex) -> Complex {
    Complex {
      real: self + rhs.real,
      imaginary: rhs.imaginary,
    }
  }
}

impl std::ops::Sub<Complex> for Complex {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: Self) -> Complex {
    Complex {
      real: self.real - rhs.real,
      imaginary: self.imaginary - rhs.imaginary,
    }
  }
}

impl std::ops::Sub<f32> for Complex {
  type Output = Self;

  #[inline]
  fn sub(self, rhs: f32) -> Complex {
    Complex {
      real: self.real - rhs,
      imaginary: self.imaginary,
    }
  }
}

impl std::ops::Sub<Complex> for f32 {
  type Output = Complex;

  #[inline]
  fn sub(self, rhs: Complex) -> Complex {
    Complex {
      real: self - rhs.real,
      imaginary: -rhs.imaginary,
    }
  }
}

impl std::ops::Mul<Complex> for Complex {
  type Output = Self;

  #[inline]
  fn mul(self, rhs: Self) -> Complex {
    Complex {
      real: self.real * rhs.real - self.imaginary * rhs.imaginary,
      imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
    }
  }
}

impl std::ops::Mul<f32> for Complex {
  type Output = Self;

  #[inline]
  fn mul(self, rhs: f32) -> Complex {
    Complex {
      real: self.real * rhs,
      imaginary: self.imaginary * rhs,
    }
  }
}

impl std::ops::Mul<Complex> for f32 {
  type Output = Complex;

  #[inline]
  fn mul(self, rhs: Complex) -> Complex {
    Complex {
      real: self * rhs.real,
      imaginary: self * rhs.imaginary,
    }
  }
}

impl std::ops::Div<Complex> for Complex {
  type Output = Self;

  #[inline]
  fn div(self, rhs: Self) -> Complex {
    let denominator = rhs.real * rhs.real + rhs.imaginary * rhs.imaginary;
    let real = (self.real * rhs.real + self.imaginary * rhs.imaginary) / denominator;
    let imaginary = (self.imaginary * rhs.real - self.real * rhs.imaginary) / denominator;
    Complex { real, imaginary }
  }
}

impl std::ops::Div<f32> for Complex {
  type Output = Self;

  #[inline]
  fn div(self, rhs: f32) -> Complex {
    Complex {
      real: self.real / rhs,
      imaginary: self.imaginary / rhs,
    }
  }
}

impl std::ops::Div<Complex> for f32 {
  type Output = Complex;

  #[inline]
  fn div(self, rhs: Complex) -> Complex {
    Complex::from(self).div(rhs)
  }
}

#[cfg(test)]
mod tests {
  use super::super::*;
  use super::*;
  use float_cmp::assert_approx_eq;

  fn approx_eq(z: Complex, w: Complex) -> bool {
    dbg!(z, w);
    (z.real - w.real).abs() < 0.0001 && (z.imaginary - w.imaginary).abs() < 0.0001
  }

  #[test]
  fn from_polar() {
    let z = Complex::from_polar(1., FRAC_PI_4);
    let expected = Complex::new(SQRT_2 / 2., SQRT_2 / 2.);

    assert!(approx_eq(z, expected));
  }

  #[test]
  fn j() {
    let z = Complex::j();
    let expected = Complex::new(0., 1.);

    assert!(approx_eq(z, expected));
  }

  #[test]
  fn conjugate() {
    let z = Complex::new(1.3, 3.4);
    let w = z.conjugate();
    let expected = Complex::new(1.3, -3.4);

    assert!(approx_eq(w, expected));
  }

  #[test]
  fn square() {
    let z = Complex::from_polar(3., FRAC_PI_4);
    let z_squared = z.square();
    let expected = Complex::new(0., 9.);

    assert!(approx_eq(z_squared, expected));
  }

  #[test]
  fn cube() {
    let z = Complex::from_polar(3., FRAC_PI_2);
    let z_squared = z.cube();
    let expected = Complex::new(0., -27.);

    assert!(approx_eq(z_squared, expected));
  }

  #[test]
  fn reciprocal() {
    let z = Complex::from_polar(5., 3. * PI / 4.);
    let z_reciprocal = z.reciprocal();
    let expected = Complex::from_polar(0.2, -3. * PI / 4.);

    assert!(approx_eq(z_reciprocal, expected));
  }

  #[test]
  fn real() {
    let z = Complex::new(1., 2.);
    let real = z.real();
    let expected = 1.;

    assert_eq!(real, expected);
  }

  #[test]
  fn imaginary() {
    let z = Complex::new(1., 2.);
    let im = z.imaginary();
    let expected = 2.;

    assert_eq!(im, expected);
  }

  #[test]
  fn modulus() {
    let z = Complex::new(1., 1.);
    let modulus = z.modulus();
    let expected = SQRT_2;

    assert_approx_eq!(f32, modulus, expected);
  }

  #[test]
  fn arg() {
    let z = Complex::new(0., 1.);
    let arg = z.arg();
    let expected = FRAC_PI_2;

    assert_approx_eq!(f32, arg, expected);
  }

  #[test]
  fn sqrt() {
    let z = Complex::new(0., 1.);
    let sqrt = z.sqrt();
    let expected = Complex::new(SQRT_2 / 2., SQRT_2 / 2.);

    assert!(approx_eq(sqrt, expected));
  }

  #[test]
  fn cbrt() {
    let z = Complex::new(0., 1.);
    let cbrt = z.cbrt();
    let expected = Complex::new(3f32.sqrt() / 2., 0.5);

    assert!(approx_eq(cbrt, expected));
  }

  #[test]
  fn powi() {
    let z = Complex::new(SQRT_2 / 2., SQRT_2 / 2.);

    let pow2 = z.powi(2);
    let expected = Complex::new(0., 1.);
    assert!(approx_eq(pow2, expected));

    let pow3 = z.powi(3);
    let expected = Complex::new(-SQRT_2 / 2., SQRT_2 / 2.);
    assert!(approx_eq(pow3, expected));

    let pow5 = z.powi(5);
    let expected = Complex::new(-SQRT_2 / 2., -SQRT_2 / 2.);
    assert!(approx_eq(pow5, expected));

    let pow8 = z.powi(8);
    let expected = Complex::new(1., 0.);
    assert!(approx_eq(pow8, expected));
  }

  #[test]
  fn from_f32() {
    let z = Complex::from(13.);
    let expected = Complex::new(13., 0.);

    assert!(approx_eq(z, expected));
  }

  #[test]
  fn add() {
    let z = Complex::new(1.5, 4.0);
    let w = Complex::new(0.2, 0.3);
    let result = z + w;
    let expected = Complex::new(1.7, 4.3);

    assert!(approx_eq(result, expected));
  }

  #[test]
  fn add_f32() {
    let z = Complex::new(1.5, 4.0);
    let result = z + 13.;
    let expected = Complex::new(14.5, 4.);
    assert!(approx_eq(result, expected));

    let z = Complex::new(1.5, 4.0);
    let result = 13. + z;
    let expected = Complex::new(14.5, 4.);
    assert!(approx_eq(result, expected));
  }

  #[test]
  fn sub() {
    let z = Complex::new(1.5, 4.0);
    let w = Complex::new(0.2, 0.3);
    let result = z - w;
    let expected = Complex::new(1.3, 3.7);

    assert!(approx_eq(result, expected));
  }

  #[test]
  fn sub_f32() {
    let z = Complex::new(1.5, 4.0);
    let result = z - 13.;
    let expected = Complex::new(-11.5, 4.0);
    assert!(approx_eq(result, expected));

    let z = Complex::new(1.5, 4.0);
    let result = 13. - z;
    let expected = Complex::new(11.5, -4.0);
    assert!(approx_eq(result, expected));
  }

  #[test]
  fn mul() {
    let z = Complex::new(3., 3.);
    let w = Complex::new(SQRT_2 / 2., SQRT_2 / 2.);
    let result = z * w;
    let expected = Complex::new(0., 3. * SQRT_2);

    assert!(approx_eq(result, expected));
  }

  #[test]
  fn mul_f32() {
    let z = Complex::new(3., 1.);
    let result = z * 1.5;
    let expected = Complex::new(4.5, 1.5);
    assert!(approx_eq(result, expected));

    let z = Complex::new(3., 1.);
    let result = 1.5 * z;
    let expected = Complex::new(4.5, 1.5);
    assert!(approx_eq(result, expected));
  }

  #[test]
  fn div() {
    let z = Complex::new(-SQRT_2, SQRT_2);
    let w = Complex::from_polar(2., FRAC_PI_4);
    let result = z / w;
    let expected = Complex::new(0., 1.);

    assert!(approx_eq(result, expected));
  }

  #[test]
  fn div_f32() {
    let z = Complex::new(3., 1.);
    let result = z / 1.5;
    let expected = Complex::new(2., 2. / 3.);
    assert!(approx_eq(result, expected));

    let z = Complex::from_polar(1., FRAC_PI_4);
    let result = 2. / z;
    let expected = Complex::new(SQRT_2, -SQRT_2);
    assert!(approx_eq(result, expected));
  }
}
