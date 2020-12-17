#![feature(custom_inner_attributes)]
extern crate rhythmc;

mod root {
  pub mod myshader {
    #![rhythmc::shader]
  }
}

#[test]
fn module_test() {
  assert_eq!(root::myshader::RHYTHMC_SHADER_MODULE_NAME, "rhythmc::root::myshader");
  assert_eq!(root::myshader::RHYTHMC_SHADER_MODULE_IMPORTS, Vec::<String>::new());
}
