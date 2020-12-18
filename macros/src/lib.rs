#![feature(
  custom_inner_attributes,
  proc_macro_span,
  proc_macro_diagnostic,
  proc_macro_def_site
)]
use compiler::utils::syn::resolve_module_path;

#[macro_use]
extern crate lazy_static;

use glsl::syntax::TranslationUnit;
use proc_macro::{Delimiter, Span, TokenStream, TokenTree};

use std::collections::HashMap;
use std::str::FromStr;

lazy_static! {
  /// Stores a map of already compiled module names to their respective GLSL IR.
  /// This is used to look up compiled translation unit that are being imported.
  static ref IR_MAP: HashMap<String, TranslationUnit> = HashMap::new();
}

fn compile_error(message: &str) -> TokenStream {
  TokenStream::from_str(message).unwrap()
}

#[proc_macro_attribute]
pub fn shader(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let module_path = if let Some(path) = resolve_module_path(
    Span::call_site().into(),
    Span::call_site().source_file().path(),
  ) {
    path
  } else {
    return compile_error("rhythmc failed to resolve this module path");
  };

  let mut result = Vec::new();
  let mut iter = item.into_iter();
  loop {
    let token = iter.next();
    if token.is_none() {
      return compile_error(
        "invalid usage of 'rhythmc::shader', expected module",
      );
    }
    result.push(TokenStream::from(token.clone().unwrap()));
    if let TokenTree::Group(group) = token.clone().unwrap() {
      if group.delimiter() == Delimiter::Brace {
        return compile_error(
          "invalid usage of 'rhythmc::shader', expected module",
        );
      }
    }
    if token.unwrap().to_string() == "mod" {
      break;
    }
  }
  result.push(TokenStream::from(iter.next().unwrap()));
  if let TokenTree::Group(group) = iter.next().unwrap() {
    assert_eq!(group.delimiter(), Delimiter::Brace);
    result.push(
      TokenStream::from_str(
        format!(
          "{{ 
        pub static RHYTHMC_SHADER_MODULE_NAME: &str = \"{}\";
        pub static RHYTHMC_SHADER_MODULE_IMPORTS: Vec<String> = vec![];
        {}
      }}",
          module_path,
          group.stream()
        )
        .as_str(),
      )
      .unwrap(),
    );
  } else {
    return compile_error(
      "invalid usage of 'rhythmc::shader', failed to parse",
    );
  }
  assert!(iter.next().is_none());
  let result = result.iter().cloned().collect();
  result
}

#[proc_macro_attribute]
pub fn import(_attr: TokenStream, item: TokenStream) -> TokenStream {
  // Find RHYTHM_SHADER_MODULE_IMPORTS and add an entry for an external source
  item
}

// #[proc_macro_derive]
// pub fn rhythmc_struct
