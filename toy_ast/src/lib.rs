pub enum Statement {
    Function,
    Block(Block)
}

pub struct Ident {
    pub refers_to: String
}

pub struct Type {
    pub type_args: Vec<Type>,
    pub type_ident: Ident
}

pub struct Struct {
    pub ident: Ident
}

pub struct Function {
    pub is_async: bool,
    pub is_pub: bool,
    pub ident: Option<Ident>,
    pub args: Vec<(Ident, Type)>,
    pub block: Block
}

pub struct Block {
    pub statements: Vec<Statement>
}

pub trait Callable {}

impl Callable for Function {}
impl Callable for Struct {}
impl Callable for Ident {}

pub struct FunctionCall<T: Callable> {
    pub called_fn: T
}