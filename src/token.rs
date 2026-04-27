//! This module provides definitions for the fundamental types used in a
//! traditional compiler.
//!
//! This includes items such as Tokens, AST Nodes, and the AST itself. These
//! definitions are meant to be able to be passed around between compilation
//! stages (such as between lexing, parsing, and AST validation). The types
//! that may exist on their own (such as the Lexer and Parser) exist in their
//! own modules

use std::iter::zip;

use crate::error::CompilationError;

/// The fundamental unit of a source code file. These are used to represent
/// the _lexemes_ within a source file
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

/// A small wrapper around a `Token` that also contains line and column info.
/// This just lets the lexer and parser output meaningful error messages so
/// a user knows where a mistake may have been made.
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

    /// Simply used to directly pattern match on the internal `token` field.
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

/// Represents a privilege level of an external function or an ISR
#[derive(Debug, Clone, Copy)]
pub enum Ring {
    Super,
    User,
}

/// All of these type traits look suspiciously similar to the Type Classes
/// covered in lecture...
impl PartialEq for Ring {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::Super, Self::Super) | (Self::User, Self::User)
        )
    }
}

impl Eq for Ring {}

impl PartialOrd for Ring {
    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (Ring::Super, _) => false,
            (Ring::User, Ring::Super) => true,
            (Ring::User, Ring::User) => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (Ring::User, _) => true,
            (Ring::Super, Ring::Super) => true,
            (Ring::Super, Ring::User) => false,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (Ring::Super, Ring::Super) => false,
            (Ring::Super, _) => true,
            (Ring::User, _) => false,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (Ring::Super, _) => true,
            (Ring::User, Ring::User) => true,
            (Ring::User, _) => false,
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ring {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.gt(other) {
            std::cmp::Ordering::Greater
        } else if self.lt(other) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.gt(&other) { self } else { other }
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.lt(&other) { self } else { other }
    }

    fn clamp(self, _min: Self, _max: Self) -> Self
    where
        Self: Sized,
    {
        todo!()
    }
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

/// Some power of 2 in the range [8-64] inclusive
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
    /// TODO(SEP): Do something with the func arg name when we have arbitrary arg usage
    _name: String,

    /// `type` is likely reserved by the rust language for use so I can't use the darn name
    type_t: FuncArgType,
}

impl FuncArg {
    pub fn new(name: String, type_t: FuncArgType) -> FuncArg {
        FuncArg {
            _name: name,
            type_t,
        }
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

    pub fn verify(&self) -> Result<(), CompilationError> {
        for node in &self.0 {
            match node {
                Ingot::ExternalFunction(extern_func) => {
                    if extern_func.args.len() > 1 {
                        eprintln!(
                            "Warning(Definition): Multiple args in external functions not yet supported"
                        );
                        eprintln!("Function: {}", extern_func.name);
                    }
                }
                Ingot::Isr(isr) => match &isr.calling_func {
                    None => {
                        eprintln!("Warning: Found ISR with no external function call.",);
                        eprintln!(
                            "This will generate code that effectively ignores this interrupt."
                        );
                    }
                    Some(callsite) => {
                        let calling_func = self
                            .0
                            .iter()
                            .filter_map(|ingot| match ingot {
                                Ingot::Isr(_) => None,
                                Ingot::ExternalFunction(candidate) => {
                                    if candidate.name.eq(&callsite.name) {
                                        Some(candidate)
                                    } else {
                                        None
                                    }
                                }
                            })
                            .collect::<Vec<&ExternalFunctionNode>>();

                        match calling_func[..] {
                            [] => {
                                return Err(CompilationError::new_without_src_info(format!(
                                    "Attempt to call an undefined function: {} from isr {}",
                                    callsite.name, isr.name
                                )));
                            }
                            [callee] => {
                                if callee.args.len() != callsite.args.len() {
                                    return Err(CompilationError::new_without_src_info(format!(
                                        "Mismatched arg count between definition and usage of {} within isr {}({})",
                                        callee.name, isr.name, isr.id
                                    )));
                                }

                                if callee.args.len() > 1 {
                                    eprintln!(
                                        "Warning(Callsite): Multiple args in external functions not yet supported"
                                    );
                                    eprintln!("Function: {}", callee.name);
                                }

                                for (def_arg, call_arg) in zip(&callee.args, &callsite.args) {
                                    match (def_arg.type_t, call_arg) {
                                        // a $ (AKA a U8) can safely upgrade to all other types
                                        // Except i8
                                        (FuncArgType::I8, CallArg::Dollar) => {
                                            return Err(CompilationError::new_without_src_info(
                                                format!(
                                                    "At callsite of {} in isr {}({}): '$' (AKA U8) not safely convertable to I8",
                                                    callee.name, isr.name, isr.name
                                                ),
                                            ));
                                        }
                                        (_, CallArg::Dollar) => {}
                                        (_, CallArg::Var(var)) => {
                                            return Err(CompilationError::new_without_src_info(
                                                format!(
                                                    "At callsite of {} in isr {}{}: arbitrary variable expressions ({}) not yet supported",
                                                    callee.name, isr.name, isr.id, var
                                                ),
                                            ));
                                        }
                                    }
                                }

                                // All ISRs run in kernel code. Their privilege is a matter of what
                                // type of software may trigger them, and thus what type of func
                                // they are allowed to call.
                                //
                                // Also MONAD RETURN MOMENT!!!!
                                let isr_privilege: Ring = match isr.privilege {
                                    None => Ring::Super, // implictly more restrictive
                                    Some(p) => p,
                                };
                                let callee_privilege: Ring = match callee.privilege {
                                    None => Ring::Super,
                                    Some(p) => p,
                                };

                                if isr_privilege < callee_privilege {
                                    return Err(CompilationError::new_without_src_info(format!(
                                        "Attempt to call higher privilege function ({}) from lower privilege ISR ({})",
                                        callee.name, isr.name
                                    )));
                                }
                            }
                            _ => {
                                return Err(CompilationError::new_without_src_info(format!(
                                    "Attempt to call ambiguous function: {} from isr {} (Did you accidentally define {} twice?)",
                                    callsite.name, isr.name, callsite.name
                                )));
                            }
                        }
                    }
                },
            }
        }

        Ok(())
    }
}
