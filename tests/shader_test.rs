#![feature(custom_inner_attributes)]

mod root {
  pub mod myshader {
    #![rhythmc::shader]

    use rhythmc::core::{Vec3, Mat4};
    use rhythmc::core::types::{Attribute, Uniform};

    pub struct Vertex {
      position: Attribute<Vec3>,
      normal: Attribute<Vec3>,
      model: Uniform<Mat4>,
      view: Uniform<Mat4>,
      projection: Uniform<Mat4>,
    }

    impl VertexShader for Vertex {
      fn vertex(&self) -> {

      }
    }
  }
}

#[test]
fn module_test() {
  assert_eq!(
    root::myshader::RHYTHMC_SHADER_MODULE_NAME,
    "rhythmc::root::myshader"
  );
  assert_eq!(
    root::myshader::RHYTHMC_SHADER_MODULE_IMPORTS,
    Vec::<String>::new()
  );
}
