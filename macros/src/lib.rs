#![feature(custom_inner_attributes)]
use proc_macro::{TokenStream, TokenTree, Delimiter};
use std::str::FromStr;

#[proc_macro_attribute]
pub fn rhythmc_shader(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let mut result = Vec::new();
  let mut iter = item.clone().into_iter();
  loop {
    let token = iter.next();
    if token.is_none() {
      return TokenStream::from_str("compile_error!(\"invalid usage of 'rhythm_shader', expected module\")").unwrap();
    }
    result.push(TokenStream::from(token.clone().unwrap()));
    if let TokenTree::Group(group) = token.clone().unwrap() {
      if group.delimiter() == Delimiter::Brace {
        return TokenStream::from_str("compile_error!(\"invalid usage of 'rhythm_shader', expected module\")").unwrap();
      }
    }
    if token.unwrap().to_string() == "mod" {
      break
    }
  }
  result.push(TokenStream::from(iter.next().unwrap()));
  if let TokenTree::Group(group) = iter.next().unwrap() {
    assert_eq!(group.delimiter(), Delimiter::Brace);
    result.push(TokenStream::from_str(format!(
      "{{ 
        pub static RHYTHMC_SHADER_MODULE_NAME: &str = module_path!();
        pub static RHYTHMC_SHADER_MODULE_IMPORTS: Vec<String> = vec![];
        {}
      }}",
      group.stream()
    ).as_str()).unwrap());
  } else {
    return TokenStream::from_str("compile_error!(\"invalid usage of 'rhythm_shader', failed to parse\")").unwrap();
  }
  assert!(iter.next().is_none());
  let result = result.iter().cloned().collect();
  println!("YOYOYOYOOYO {}", result);
  result
}

#[proc_macro_attribute]
pub fn import(_attr: TokenStream, item: TokenStream) -> TokenStream {
  // Find RHYTHM_SHADER_MODULE_IMPORTS and add an entry for an external source
  item
}

// #[proc_macro_derive]
// pub fn rhythmc_struct