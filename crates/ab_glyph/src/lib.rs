//! Provides an extension trait for `ab_glyph`'s `Font` types.

use ::ab_glyph::{
  point, Font, Glyph, Outline, OutlineCurve, Point, PxScaleFactor, Rect,
  ScaleFont,
};

pub struct RsdfGlyph {
  glyph: Glyph,
  px_bounds: Rect,
  scale_factor: PxScaleFactor,
  outline: Outline,
}

impl RsdfGlyph {
  /// Construct an `RsdfGlyph` from the source `Glyph`, pixel bounds &
  /// relatively positioned outline curves.
  pub fn new(
    glyph: Glyph,
    outline: Outline,
    scale_factor: PxScaleFactor,
  ) -> Self {
    let px_bounds = outline.px_bounds(scale_factor, glyph.position);

    RsdfGlyph {
      glyph,
      px_bounds,
      scale_factor,
      outline,
    }
  }

  /// Glyph info.
  pub fn glyph(&self) -> &Glyph {
    &self.glyph
  }

  /// Conservative whole number pixel boundeing box for this glyph.
  pub fn px_bounds(&self) -> Rect {
    self.px_bounds
  }

  /// Draw this glyph outline using a pixel & field handling function.
  ///
  /// The callback will be called for each `(x, y)` pixel coordinate inside the
  /// bounds with a field value indicating the colour channel values
  /// corresponding to that pixel.
  ///
  // TODO: Each channel value ranges between `-1.0` and `1.0`.
  pub fn draw<O: FnMut(u32, u32, [u8; 3])>(&self, mut output: O) {
    let h_factor = self.scale_factor.horizontal;
    let v_factor = -self.scale_factor.vertical;
    let offset = self.glyph.position - self.px_bounds.min;
    let (w, h) = (
      self.px_bounds.width() as usize,
      self.px_bounds.height() as usize,
    );
    let scale_up = |Point { x, y }| point(x * h_factor, y * v_factor);
    let get_first = |curve: &OutlineCurve| match curve {
      OutlineCurve::Line(p, _) => *p,
      OutlineCurve::Quad(p, _, _) => *p,
      OutlineCurve::Cubic(p, _, _, _) => *p,
    };
    let get_last = |curve: &OutlineCurve| match curve {
      OutlineCurve::Line(_, p) => *p,
      OutlineCurve::Quad(_, _, p) => *p,
      OutlineCurve::Cubic(_, _, _, p) => *p,
    };

    let mut rasterizer = Some(rsdf_builder::ShapeBuilder::new());
    let mut curves = self.outline.curves.iter().cloned().peekable();

    if curves.len() > 0 {
      let c = curves.peek().unwrap();
      let mut first = get_first(c);
      let mut contour_builder = None;

      for curve in curves {
        if contour_builder.is_none() {
          first = get_first(&curve);
          contour_builder = Some(
            rasterizer
              .take()
              .unwrap()
              .contour(point_from_point(scale_up(first) + offset)),
          );
        }
        match curve {
          OutlineCurve::Line(_, p1) => {
            contour_builder = Some(
              contour_builder
                .unwrap()
                .line(point_from_point(scale_up(p1) + offset)),
            );
          },
          OutlineCurve::Quad(_, p1, p2) => {
            contour_builder = Some(contour_builder.unwrap().quadratic_bezier(
              point_from_point(scale_up(p1) + offset),
              point_from_point(scale_up(p2) + offset),
            ));
          },
          OutlineCurve::Cubic(_, p1, p2, p3) => {
            contour_builder = Some(contour_builder.unwrap().cubic_bezier(
              point_from_point(scale_up(p1) + offset),
              point_from_point(scale_up(p2) + offset),
              point_from_point(scale_up(p3) + offset),
            ));
          },
        }
        if first == get_last(&curve) {
          rasterizer = Some(contour_builder.take().unwrap().end_contour());
        }
      }
    }

    const MARGIN: usize = 5;

    let rasterizer = rasterizer
      .expect("contour must not have terminated")
      .build();
    for x in 0..(w + MARGIN * 2) {
      for y in 0..(h + MARGIN * 2) {
        let sample = rasterizer
          .sample((x as f32 - MARGIN as f32, y as f32 - MARGIN as f32).into());
        let mut color @ [r, g, b] = sample.map(|sp| {
          let sp = -sp; // depends on chirality of font :(
          rsdf_core::distance_color(sp)
        });
        // clip remaining values when bulk is 0
        let sum = r as u16 + g as u16 + b as u16;
        if r as u16 == sum || g as u16 == sum || b as u16 == sum {
          color = [0; 3];
        }
        // clip when bulk is saturated
        if r == 255 && b == 255 || r == 255 && g == 255 || b == 255 && g == 255
        {
          color = [255; 3];
        }

        output(x as _, y as _, color)
      }
    }
  }
}

// TODO:
// - add max_dist to sample function
// - clip to bulk in sample function
// - return normalised values, between [0, 1] or [-1, 1]
//   probably the second, as it emphasizes the "signed"-ness of sdfs
// - output of ab_glyph draw function should be those normalised values

pub trait FontExtRsdf {
  fn outline_glyph_rsdf(&self, glyph: Glyph) -> Option<RsdfGlyph>;
}

impl<F> FontExtRsdf for F
where
  F: Font,
{
  fn outline_glyph_rsdf(&self, glyph: Glyph) -> Option<RsdfGlyph> {
    let outline = self.outline(glyph.id)?;
    let scale_factor = self.as_scaled(glyph.scale).scale_factor();
    Some(RsdfGlyph::new(glyph, outline, scale_factor))
  }
}

#[inline]
fn point_from_point(value: Point) -> rsdf_core::Point {
  rsdf_core::Point {
    x: value.x,
    y: value.y,
  }
}
