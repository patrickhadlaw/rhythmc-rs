use crate::utils::syn::IsIgnore;
use syn::{ItemUse, Attribute, token};
use syn::parse::{Parse, ParseStream};

#[derive(Debug)]
pub struct ShaderModule {
  /// The shader module dependencies (dependencies are resolved during shader
  /// loading by injection of the corresponding syntax tree)
  pub external: Vec<UseTree>,
}

impl Parse for ShaderModule {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut result = ShaderModule {
      external: Vec::new(),
    };
    while !input.is_empty() {
      let lookahead = input.lookahead1();
      let command = if lookahead.peek(token::Pound) {
        if let Ok(cmd) = input.parse::<AttributeCommand>() {
          cmd
        } else {
          AttributeCommand::None
        }
      } else {
        AttributeCommand::None
      };
      let lookahead = input.lookahead1();
      if lookahead.peek(token::Use) {
        if command.ignore() {
          input.parse::<ItemUse>()?;
        } else {
          result.external.push(input.parse()?)
        }
      } else {
        return Err(input.error("[rhythmc] failed to parse shader module, unexpected token received"))
      }
    }
    Ok(result)
  }
}

#[derive(Debug)]
pub enum AttributeCommand {
  Ignore,
  None,
}

impl AttributeCommand {
  pub fn ignore(&self) -> bool {
    if let AttributeCommand::Ignore = self {
      true
    } else {
      false
    }
  }
}

impl Parse for AttributeCommand {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut ignore = false;
    for attr in input.call(Attribute::parse_outer)?.iter() {
      if attr.is_ignore() {
        ignore = true;
        break;
      }
    }
    if ignore {
      Ok(AttributeCommand::Ignore)
    } else {
      Ok(AttributeCommand::None)
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct UseName {
  pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct UsePath {
  pub path: String,
  pub next: Box<UseTree>,
}

#[derive(Debug, PartialEq)]
pub struct UseGroup {
  pub group: Vec<Box<UseTree>>,
}

#[derive(Debug, PartialEq)]
pub enum UseTree {
  All,
  Name(UseName),
  Path(UsePath),
  Group(UseGroup),
  // Renaming not supported
}

impl UseTree {
  pub fn from(tree: &syn::UseTree) -> syn::Result<Self> {
    Ok(match tree {
      syn::UseTree::Glob(_) => UseTree::All,
      syn::UseTree::Name(name) => UseTree::Name(UseName {
        name: format!("{}", name.ident),
      }),
      syn::UseTree::Path(path) => UseTree::Path(UsePath {
        path: format!("{}", path.ident),
        next: Box::new(UseTree::from(&*path.tree)?)
      }),
      syn::UseTree::Group(group) => UseTree::Group(UseGroup {
        group: group.items.iter()
          .map(|tree| match UseTree::from(tree) {
            Ok(val) => Ok(Box::new(val)),
            Err(err) => Err(err),
          })
          .collect::<syn::Result<Vec<Box<UseTree>>>>()?
      }),
      syn::UseTree::Rename(rename) => {
        return Err(syn::Error::new_spanned(
          rename.as_token,
          "[rhythmc] invalid use statement: renaming modules not supported"
        ))
      },
    })
  }
}

impl Parse for UseTree {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let item_use: ItemUse = input.parse()?;
    UseTree::from(&item_use.tree)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn use_parse_test() {
    let parsed = syn::parse_str::<UseTree>("use rhythmc::core::{thing1, module::{thing2, thing3}, all::*};").unwrap();
    let tree = UseTree::Path(UsePath {
      path: "rhythmc".to_owned(),
      next: Box::new(UseTree::Path(UsePath {
        path: "core".to_owned(),
        next: Box::new(UseTree::Group(UseGroup {
          group: vec![
            Box::new(UseTree::Name(UseName {
              name: "thing1".to_owned(),
            })),
            Box::new(UseTree::Path(UsePath {
              path: "module".to_owned(),
              next: Box::new(UseTree::Group(UseGroup {
                group: vec![
                  Box::new(UseTree::Name(UseName {
                    name: "thing2".to_owned(),
                  })),
                  Box::new(UseTree::Name(UseName {
                    name: "thing3".to_owned(),
                  }))
                ],
              })),
            })),
            Box::new(UseTree::Path(UsePath {
              path: "all".to_owned(),
              next: Box::new(UseTree::All),
            })),
          ]
        })),
      })),
    });
    assert_eq!(parsed, tree);
    // Renaming not supported
    syn::parse_str::<UseTree>("use rhythmc::core::{thing1, module::{thing2, thing3}, xyz as abc};").unwrap_err();
  }

  #[test]
  fn ignore_attribute_command_test() {
    let shader = syn::parse_str::<ShaderModule>("
      #[rhythmc::ignore]
      use rhythmc::core::{thing1, module::{thing2, thing3}, thing4};
    ").unwrap();
    assert!(shader.external.is_empty());
  }
}