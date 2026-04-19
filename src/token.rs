pub enum Token {
    KeywordExtern,
    KeywordRingLevel,
    KeywordUser,
    KeywordSuper,
    KeywordIsr,

    Identifier(String),

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
}
