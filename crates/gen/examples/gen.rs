#![allow(unused)]

use sdf_gen::*;

fn main() {
  do_thing_hollow()
}

pub fn do_thing_hollow() {
  use Segment::*;

  println!("doing_thing");

  let contour_1 = {
    let point_a = Point { x: 14.0, y: 8.0 };
    let point_b = Point { x: 46.0, y: 8.0 };
    let point_c = Point { x: 30.0, y: 40.0 };

    let vec_ab = point_a.vector_to(point_b);
    let vec_bc = point_b.vector_to(point_c);
    let vec_ca = point_c.vector_to(point_a);

    let ray_a = (vec_ca.norm() + -vec_ab.norm()).norm();
    let ray_b = (vec_ab.norm() + -vec_bc.norm()).norm();
    let ray_c = (vec_bc.norm() + -vec_ca.norm()).norm();

    let line_ab = Line {
      start: point_a,
      end: point_b,
    };
    let line_bc = Line {
      start: point_b,
      end: point_c,
    };
    let line_ca = Line {
      start: point_c,
      end: point_a,
    };

    let channels_ab: Channels = 0b101.into();
    let channels_bc: Channels = 0b110.into();
    let channels_ca: Channels = 0b011.into();

    let corner_rays_ab = CornerRays {
      start: ray_a,
      end: ray_b,
    };
    let corner_rays_bc = CornerRays {
      start: ray_b,
      end: ray_c,
    };
    let corner_rays_ca = CornerRays {
      start: ray_c,
      end: ray_a,
    };

    Contour {
      segments: vec![line_ab, line_bc, line_ca],
      corners: Some(vec![0, 1, 2]),
      corner_rays: Some(vec![corner_rays_ab, corner_rays_bc, corner_rays_ca]),
      channels: Some(vec![channels_ab, channels_bc, channels_ca]),
    }
  };

  let contour_2 = {
    let point_d = Point { x: 20.0, y: 11.5 };
    let point_e = Point { x: 40.0, y: 11.5 };
    let point_f = Point { x: 30.0, y: 34.0 };

    let vec_df = point_d.vector_to(point_f);
    let vec_fe = point_f.vector_to(point_e);
    let vec_ed = point_e.vector_to(point_d);

    let ray_a = (vec_df.norm() + -vec_fe.norm()).norm();
    let ray_b = (vec_df.norm() + -vec_fe.norm()).norm();
    let ray_c = (vec_df.norm() + -vec_fe.norm()).norm();

    let line_df = Line {
      start: point_d,
      end: point_f,
    };
    let line_fe = Line {
      start: point_f,
      end: point_e,
    };
    let line_ed = Line {
      start: point_e,
      end: point_d,
    };

    let channels_df: Channels = 0b101.into();
    let channels_fe: Channels = 0b011.into();
    let channels_ed: Channels = 0b110.into();

    let corner_rays_ab = CornerRays {
      start: ray_a,
      end: ray_b,
    };
    let corner_rays_bc = CornerRays {
      start: ray_b,
      end: ray_c,
    };
    let corner_rays_ca = CornerRays {
      start: ray_c,
      end: ray_a,
    };

    Contour {
      segments: vec![line_df, line_fe, line_ed],
      corners: Some(vec![0, 1, 2]),
      corner_rays: Some(vec![corner_rays_ab, corner_rays_bc, corner_rays_ca]),
      channels: Some(vec![channels_df, channels_fe, channels_ed]),
    }
  };



  let shape = Shape {
    contours: vec![contour_1, contour_2],
  };

  let mut image = Image::new("test_image.png", [60, 60]);

  let mut min = 0.;
  let mut max = f32::MAX;

  let start_time = std::time::Instant::now();

  for y in 0..image.height {
    for x in 0..image.width {
      let point: Point = (x as f32, y as f32).into();

      let pixel = shape.sample(point);

      let pixel = pixel.map(|sp| distance_color(sp));

      image.set_pixel([x, y], pixel);
    }
  }

  let duration_time = std::time::Instant::now() - start_time;

  dbg!(duration_time);

  dbg!(min, max);

  image.flush();
}
