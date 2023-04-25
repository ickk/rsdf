pub const MAX_DISTANCE: f32 = 5.;
pub const MAX_COLOUR: f32 = 256.0;
#[inline]
pub fn distance_color(distance: f32) -> u8 {
  let distance = distance.clamp(-MAX_DISTANCE, MAX_DISTANCE);
  (((distance + MAX_DISTANCE) / (2.0 * MAX_DISTANCE) * MAX_COLOUR) - 1.0) as u8
}
