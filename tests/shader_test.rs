#![feature(custom_inner_attributes)]

mod root {
  pub mod myshader {
    #![rhythmc::shader]

    use rhythmc::core::{Vec3, Mat4};

    pub struct Vertex {
      #[rhythmc::input] position: Vec3,
      #[rhythmc::input] normal: Vec3,
      #[rhythmc::uniform] model: Mat4,
      view: Mat4,
      projection: Mat4,
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
