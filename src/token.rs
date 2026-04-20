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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::KeywordExtern => write!(f, "Token::KeywordExtern"),
            Token::KeywordWithLevel => write!(f, "Token::KeywordWithLevel"),
            Token::KeywordUser => write!(f, "Token::KeywordUser"),
            Token::KeywordSuper => write!(f, "Token::KeywordSuper"),
            Token::KeywordIsr => write!(f, "Token::KeywordIsr"),
            Token::Identifier(id) => write!(f, "Token::Identifier({})", id),
            Token::LeftParen => write!(f, "Token::LeftParen"),
            Token::RightParen => write!(f, "Token::RightParen"),
            Token::LeftBracket => write!(f, "Token::LeftBracket"),
            Token::RightBracket => write!(f, "Token::RightBracket"),
        }
    }
}

#[derive(Debug)]
pub enum Ring {
    Super,
    User,
}

#[derive(Debug)]
pub enum FuncArgTypeBitCount {
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

#[derive(Debug)]
pub struct FuncArgType {
    /// Some power of 2 in the rang [8-64] inclusive
    bit_count: FuncArgTypeBitCount,
    signed: bool,
}

#[derive(Debug)]
pub struct FuncArg {
    name: String,

    /// `type` is likely reserved by the rust language for use so I can't use the darn name
    type_T: FuncArgType,
}

#[derive(Debug)]
pub struct ExternalFunctionNode {
    name: String,
    args: Vec<FuncArg>,
    privilege: Option<Ring>,
}

#[derive(Debug)]
pub struct IsrNode {}

#[derive(Debug)]
pub enum Ingot {
    ExternalFunction(ExternalFunctionNode),
    Isr(IsrNode),
}

#[derive(Debug)]
pub struct Ast(pub Vec<Ingot>);
