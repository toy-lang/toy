/// The backend for the final stage of compiling.
/// Examples:
/// ```rust
/// # use toy_codegen::{ToylangCodegenTarget, CodegenInput, CodegenOutput};
/// pub struct MyCodeGen {
///     
/// }
/// 
/// impl ToylangCodegenTarget for MyCodeGen {
///     fn startup() -> Self {
///         Self {}
///     }
///     fn compile(&mut self, ast: CodegenInput) -> CodegenOutput {
///         todo!()
///     }
/// }
/// ```
pub trait ToylangCodegenTarget {
    fn compile(&mut self, ast: CodegenInput) -> CodegenOutput;
    fn startup() -> Self;
}

pub struct CodegenInput {
    ast: toy_ast::Ast
}

pub struct CodegenOutput {

}