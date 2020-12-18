#![feature(custom_inner_attributes)]
extern crate macros as rhythmc;

mod root {
  pub mod myfirstshader {
    #![rhythmc::shader]
  }

  pub mod mysecondshader {
    #![rhythmc::shader]
  }
}

#[test]
fn module_test() {
  assert_eq!(
    root::myfirstshader::RHYTHMC_SHADER_MODULE_NAME,
    "macros::root::myfirstshader"
  );
  assert_eq!(
    root::myfirstshader::RHYTHMC_SHADER_MODULE_IMPORTS,
    Vec::<String>::new()
  );
  assert_eq!(
    root::mysecondshader::RHYTHMC_SHADER_MODULE_NAME,
    "macros::root::mysecondshader"
  );
}
