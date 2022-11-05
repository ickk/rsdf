#![allow(unused)]

use sdf_gen::*;

fn main() {
  do_thing()
}

pub fn do_thing() {
  use Segment::*;

  println!("doing_thing");

  let point_a = Point { x: 2.0, y: 5.0 };
  let point_b = Point { x: 8.0, y: 2.0 };
  let point_c = Point { x: 5.0, y: 8.0 };

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

  let contour = Contour {
    segments: vec![line_ab, line_bc, line_ca],
    corners: Some(vec![0, 1, 2]),
    corner_rays: Some(vec![corner_rays_ab, corner_rays_bc, corner_rays_ca]),
    channels: Some(vec![channels_ab, channels_bc, channels_ca]),
  };

  let shape = Shape {
    contours: vec![contour],
  };

  let mut image = Image::new("test_image.png", [20, 20]);

  for y in 0..image.height {
    for x in 0..image.width {
      let point: Point = (x as f32 * 0.5, y as f32 * 0.5).into();
      let val = shape.sample(point);
      let val = distance_color(val);
      image.set_pixel([x, y], [val, val, 0]);
    }
  }

  image.flush();
}

pub fn do_the_thing() {
  use Segment::*;

  println!("doing_thing");

  let point_a = Point { x: 2.0, y: 5.0 };
  let point_b = Point { x: 8.0, y: 2.0 };
  let point_c = Point { x: 5.0, y: 8.0 };

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

  let contour = Contour {
    segments: vec![line_ab, line_bc, line_ca],
    corners: Some(vec![0, 1, 2]),
    corner_rays: Some(vec![corner_rays_ab, corner_rays_bc, corner_rays_ca]),
    channels: Some(vec![channels_ab, channels_bc, channels_ca]),
  };

  let shape = Shape {
    contours: vec![contour],
  };

  let mut image = Image::new("test_image.png", [40, 40]);

  for y in 0..image.height {
    for x in 0..image.width {
      let point: Point = (x as f32 * 0.25, y as f32 * 0.25).into();

      let spline = shape.contours[0].splines().next().unwrap();
      let mut red = false;
      for segment in spline.segments {
        red = segment.inside_ray_start(spline.corner_rays.start, point)
        && segment.inside_ray_end(spline.corner_rays.end, point);
      }

      let mut green = false;
      let spline = shape.contours[0].splines().skip(1).next().unwrap();
      for segment in spline.segments {
        green = segment.inside_ray_start(spline.corner_rays.start, point)
        && segment.inside_ray_end(spline.corner_rays.end, point);
      }

      let mut blue = false;
      let spline = shape.contours[0].splines().skip(2).next().unwrap();
      for segment in spline.segments {
        blue = segment.inside_ray_start(spline.corner_rays.start, point)
        && segment.inside_ray_end(spline.corner_rays.end, point);
      }

      let r = if red {255} else {0};
      let g = if green {255} else {0};
      let b = if blue {255} else {0};

      image.set_pixel([x, y], [r, g, b]);
    }
  }

  image.flush();
}
