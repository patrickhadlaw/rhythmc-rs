use proc_macro2::Span;
use syn::{Item, Path};

use std::collections::VecDeque;
use std::path::PathBuf;

pub fn is_path(path: &Path, is: Vec<&str>) -> bool {
  if path.segments.len() != is.len() {
    false
  } else {
    for (i, item) in is.iter().enumerate() {
      if path.segments[i].ident != item {
        return false;
      }
    }
    true
  }
}

#[derive(Debug)]
enum LookFor {
  Module(String),
  Marker,
}

fn resolve_module_path_segment(
  syntax: &[Item],
  look_for: &LookFor,
) -> Option<String> {
  let mut result = Vec::new();
  for symbol in syntax.iter() {
    if let Item::Mod(item_mod) = symbol {
      if let Some(content) = &item_mod.content {
        if let LookFor::Marker = look_for {
          if item_mod
            .attrs
            .iter()
            .any(|x| is_path(&x.path, vec!["rhythmc", "marker"]))
          {
            result.push(item_mod.ident.to_string());
            return Some(result.join("::"));
          }
        }
        if let Some(value) = resolve_module_path_segment(&content.1, &look_for)
        {
          result.push(item_mod.ident.to_string());
          result.push(value);
          return Some(result.join("::"));
        }
      } else if let LookFor::Module(module) = look_for {
        if item_mod.ident == *module {
          result.push(module.clone());
          return Some(result.join("::"));
        }
      }
    }
  }
  None
}

pub fn resolve_module_path(call_site: Span, file: PathBuf) -> Option<String> {
  let mut path_segs: VecDeque<&str> = file.to_str()?.split('/').collect();
  let mut dir = PathBuf::new();
  dir.push(".");
  if path_segs.len() > 1 {
    dir.push(path_segs.pop_front()?);
  }

  let mut first = true;
  let mut result = std::env::var("CARGO_PKG_NAME").unwrap_or_default();
  while let Some(seg) = path_segs.pop_front() {
    let filepath = format!(
      "{}/{}",
      dir.to_str()?.to_owned(),
      if path_segs.is_empty() {
        seg
      } else if first {
        let check = dir.clone();
        dir.push("lib.rs");
        if check.as_path().exists() {
          "lib.rs"
        } else {
          "mod.rs"
        }
      } else {
        "mod.rs"
      }
    );
    let mut lines: Vec<String> = std::fs::read_to_string(&filepath)
      .ok()?
      .lines()
      .map(|x| x.to_owned())
      .collect();
    if call_site.end().line > lines.len() {
      return None;
    }
    lines.insert(call_site.end().line, "#![rhythmc::marker]".to_owned());
    let file = lines.join("\n");
    let syntax = syn::parse_file(&file).ok()?;
    let look_for = if path_segs.is_empty() {
      LookFor::Marker
    } else {
      LookFor::Module(seg.to_owned())
    };
    result += format!(
      "::{}",
      resolve_module_path_segment(&syntax.items, &look_for)?
    )
    .as_str();
    dir.push(seg);
    first = false;
  }
  Some(result)
}
