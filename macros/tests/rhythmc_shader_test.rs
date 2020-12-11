#![feature(custom_inner_attributes)]
#[macro_use]
extern crate macros;

mod root {
  pub mod myshader {
    #![rhythmc_shader]
  }
}

#[test]
fn module_test() {
  assert_eq!(root::myshader::RHYTHMC_SHADER_MODULE_NAME, "rhythmc_shader_test::root::myshader");
}
