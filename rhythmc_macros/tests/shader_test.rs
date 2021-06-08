#![feature(custom_inner_attributes)]
extern crate rhythmc_macros;

mod root {
  pub mod myfirstshader {
    #![rhythmc_macros::shader]
  }

  pub mod mysecondshader {
    #![rhythmc_macros::shader]
  }
}

#[test]
fn module_test() {
  assert_eq!(
    root::myfirstshader::RHYTHMC_SHADER_MODULE_NAME,
    "rhythmc_macros::root::myfirstshader"
  );
  assert_eq!(
    root::myfirstshader::RHYTHMC_SHADER_MODULE_IMPORTS,
    Vec::<String>::new()
  );
  assert_eq!(
    root::mysecondshader::RHYTHMC_SHADER_MODULE_NAME,
    "rhythmc_macros::root::mysecondshader"
  );
}
