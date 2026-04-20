#[derive(Debug)]
pub enum Token {
    KeywordExtern,
    KeywordWithLevel,
    KeywordUser,
    KeywordSuper,
    KeywordIsr,

    Identifier(String),

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
}

pub enum Ring {
    Super,
    User,
}

pub enum FuncArgTypeBitCount {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

pub struct FuncArgType {
    /// Some power of 2 in the rang [8-64] inclusive
    bit_count: FuncArgTypeBitCount,
    signed: bool,
}

pub struct FuncArg {
    name: String,

    /// `type` is likely reserved by the rust language for use so I can't use the darn name
    type_T: FuncArgType,
}

pub struct ExternalFunctionNode {
    name: String,
    args: Vec<FuncArg>,
    privilege: Option<Ring>,
}

pub struct IsrNode {}

pub enum Ingot {
    ExternalFunction(ExternalFunctionNode),
    Isr(IsrNode),
}

pub struct Ast(Vec<Ingot>);
