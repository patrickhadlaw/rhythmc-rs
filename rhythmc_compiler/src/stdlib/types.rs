use super::math::{V2, V3, V4, Mat2, Mat2x3, Mat2x4, Mat3, Mat3x2, Mat3x4, Mat4, Mat4x2, Mat4x3};

pub mod layout {
  pub enum Type {
    Default,
    UniformBufferObject,
    StorageBufferObject,
  }

  pub enum Location {
    None,
    At(usize),
  }
}

pub struct Layout<T, Lt = layout::Type::Default, Ll = layout::Location::None>
where
  Lt: const layout::Type,
  Ll: const layout::Location,
{}

pub trait ShaderType {}

impl <T> ShaderType for T where T: Primative {}
impl <T> ShaderType for V2<T> where T: Primative {}
impl <T> ShaderType for V3<T> where T: Primative {}
impl <T> ShaderType for V4<T> where T: Primative {}
impl <T> ShaderType for Mat2<T> where T: Primative {}
impl <T> ShaderType for Mat2x3<T> where T: Primative {}
impl <T> ShaderType for Mat2x4<T> where T: Primative {}
impl <T> ShaderType for Mat3<T> where T: Primative {}
impl <T> ShaderType for Mat3x2<T> where T: Primative {}
impl <T> ShaderType for Mat3x4<T> where T: Primative {}
impl <T> ShaderType for Mat4<T> where T: Primative {}
impl <T> ShaderType for Mat4x2<T> where T: Primative {}
impl <T> ShaderType for Mat4x3<T> where T: Primative {}

pub struct Uniform<T> where T: ShaderType {
  value: T,
}

pub struct Attribute {
  value: T,
}
