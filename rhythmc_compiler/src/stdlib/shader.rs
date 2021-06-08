use crate::stdlib::math::Vec4;

pub struct Uniform<T>(pub T);

pub struct VertexStage<T> {
  position: Vec4,
  point_size: f32,
  thing: T,
  clip_distance: [f32],
}

pub trait VertexShader<T> {
  fn vertex(&self, stage: &mut VertexStage<T>);
}

pub trait GeometryShader<T> {
  fn geometry(&self) -> T;
}

pub trait TessalationControlShader<T> {
  fn control(&self, inputs: &T) -> T;
}

pub trait TessalationEvaluateShader<T> {
  fn tessalate(&self, inputs: &T) -> T;
}

pub trait FragmentShader<T> {
  fn fragment(&self, inputs: &T) -> T;
}

pub trait ComputeShader<T> {
  fn compute(&self) -> T;
}
