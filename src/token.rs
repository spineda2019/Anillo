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
