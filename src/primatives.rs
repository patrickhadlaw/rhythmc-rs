type f16 = f32;

struct 

mod highp {
  pub struct Vec2(f64, f64);
  pub struct Vec3(f64, f64, f64);
  pub struct Vec4(f64, f64, f64, f64);
}

mod mediump {
  pub struct Vec2(f32, f32);
  pub struct Vec3(f32, f32, f32);
  pub struct Vec4(f32, f32, f32, f32);
}

mod lowp {
  pub struct Vec2(f16, f16);
  pub struct Vec3(f16, f16, f16);
  pub struct Vec4(f16, f16, f16, f16);
}

type Vec2 = mediump::Vec2;
type Vec3 = mediump::Vec3;
type Vec3 = mediump::Vec4;
