#[derive(Debug)]
pub enum Token {
    KeywordExtern,
    KeywordWithLevel,
    KeywordPrivilege(Ring),
    KeywordIsr,
    KeywordCall,

    Identifier(String),

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    /// '$' is a special Anillo token only allowed within an ISR. It directly
    /// refers to the number of the ISR itself.
    Dollar,
}

#[derive(Debug)]
pub struct TokenInfo {
    token: Token,
    line: u32,
    col: u32,
}

impl TokenInfo {
    pub fn new(token: Token, line: u32, col: u32) -> TokenInfo {
        TokenInfo { token, line, col }
    }

    pub fn borrow_token(&self) -> &Token {
        &self.token
    }

    pub fn line(&self) -> u32 {
        self.line
    }

    pub fn col(&self) -> u32 {
        self.col
    }
}

impl std::fmt::Display for TokenInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TokenInfo {{token = {}, line = {}, col = {}}}",
            self.token, self.line, self.col
        )
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::KeywordExtern => write!(f, "Token::KeywordExtern"),
            Token::KeywordWithLevel => write!(f, "Token::KeywordWithLevel"),
            Token::KeywordPrivilege(p) => write!(f, "Token::KeywordPrivilege({})", p),
            Token::KeywordIsr => write!(f, "Token::KeywordIsr"),
            Token::Identifier(id) => write!(f, "Token::Identifier({})", id),
            Token::LeftParen => write!(f, "Token::LeftParen"),
            Token::RightParen => write!(f, "Token::RightParen"),
            Token::LeftBracket => write!(f, "Token::LeftBracket"),
            Token::RightBracket => write!(f, "Token::RightBracket"),
            Token::Comma => write!(f, "Token::Comma"),
            Token::Dollar => write!(f, "Token::Dollar"),
            Token::KeywordCall => write!(f, "Token::KeywordCall"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Ring {
    Super,
    User,
}

impl std::fmt::Display for Ring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ring::({})",
            match self {
                Ring::User => "User",
                Ring::Super => "Super",
            }
        )
    }
}

/// Some power of 2 in the rang [8-64] inclusive
#[derive(Debug, Clone, Copy)]
pub enum FuncArgType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
}

#[derive(Debug)]
pub struct FuncArg {
    name: String,

    /// `type` is likely reserved by the rust language for use so I can't use the darn name
    type_t: FuncArgType,
}

impl FuncArg {
    pub fn new(name: String, type_t: FuncArgType) -> FuncArg {
        FuncArg { name, type_t }
    }
}

#[derive(Debug)]
pub struct ExternalFunctionNode {
    name: String,
    args: Vec<FuncArg>,
    privilege: Option<Ring>,
}

impl ExternalFunctionNode {
    pub fn new(name: String, args: Vec<FuncArg>, privilege: Option<Ring>) -> ExternalFunctionNode {
        ExternalFunctionNode {
            name,
            args,
            privilege,
        }
    }
}

#[derive(Debug)]
pub enum CallArg {
    Var(String),
    Dollar,
}

#[derive(Debug)]
pub struct ExternalFunctionCall {
    name: String,
    args: Vec<CallArg>,
}

impl ExternalFunctionCall {
    pub fn new(name: String, args: Vec<CallArg>) -> ExternalFunctionCall {
        ExternalFunctionCall { name, args }
    }
}

#[derive(Debug)]
pub struct IsrNode {
    name: String,
    id: u8,
    privilege: Option<Ring>,
    calling_func: Option<ExternalFunctionCall>,
}

impl IsrNode {
    pub fn new(
        name: String,
        id: u8,
        privilege: Option<Ring>,
        calling_func: Option<ExternalFunctionCall>,
    ) -> IsrNode {
        IsrNode {
            name,
            id,
            privilege,
            calling_func,
        }
    }
}

#[derive(Debug)]
pub enum Ingot {
    ExternalFunction(ExternalFunctionNode),
    Isr(IsrNode),
}

#[derive(Debug)]
pub struct Ast(Vec<Ingot>);

impl Ast {
    pub fn new(vec: Vec<Ingot>) -> Ast {
        Ast(vec)
    }

    pub fn verify(&self) {}
}
