// #![rhythm_shader]
// use rhythmc::std::*;

// #[shader_uniform(std430)]

// // or maybe

// shader_globals!{
//   layout!(rhythmc::layout::Std430) struct
// }

// [#export{
//   layout = std430
// }]
// pub struct MyGLSLStruct {}

// [#shader_stage{
//   stage = "vertex",
//   layout!(rhythmc::layout::Std430) lowp::Vec3
// }]
// [#shader_input()]
// pub fn vertex()

// Transforms to

// Global shader map: static _rhythmc_shader_map: Map<String, Box<dyn ShaderProgram>>
// Where trait ShaderProgram has method fn translation_unit() -> ast::Shader; which
// is autoimplemented by compiler

fn main() {}
