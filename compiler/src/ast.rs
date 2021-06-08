pub struct Import {
  target: String,
  built_in: bool,
}

pub enum Layout {
  Std430,
  Std130,
  Location(usize),
}

pub struct Uniform {

}

pub struct Input {

}

pub struct Assignment {

}

pub enum ShaderStageType {
  Vertex,
  Fragment,

}

pub struct ShaderStage {

}

pub enum TreeNode {
  None,
  Import(Import),
  Layout(Layout),
  Uniform(Uniform),
  Input(Input),
  Assignment(Assignment),
  ShaderStage(),
  Statement(),
  Expression(),
  Identifier(),
  Bracket(TreeNode),
  Literal()
}

pub struct AST {
  root: TreeNode,
}

impl AST {
  pub fn root(&self) -> TreeNode {
    self.root
  }
}