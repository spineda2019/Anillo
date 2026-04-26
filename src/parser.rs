use std::{collections::VecDeque, error::Error};

use crate::{
    error::CompilationError,
    token::{
        Ast, CallArg, ExternalFunctionCall, ExternalFunctionNode, FuncArg, FuncArgType, Ingot,
        IsrNode, Ring, Token, TokenInfo,
    },
};

#[derive(Debug)]
pub struct Parser {
    tokens: VecDeque<TokenInfo>,
}

impl Parser {
    pub fn new(tokens: VecDeque<TokenInfo>) -> Parser {
        Parser { tokens }
    }

    pub fn run(&mut self, verbose: bool) -> Result<Ast, Box<dyn Error>> {
        let mut ast_vec: Vec<Ingot> = Vec::new();

        while let Some(token) = self.tokens.pop_front() {
            if verbose {
                println!(
                    "###########################################################################"
                );
                println!("Consumed token: {}", token);
                println!("Remaining tokens:");
                dbg!(&self.tokens);
                println!("AST in progress:");
                dbg!(&ast_vec);
                println!();
                println!(
                    "###########################################################################"
                );
            }

            match token.borrow_token() {
                Token::KeywordExtern => {
                    let ext = self.parse_extern(token.line(), token.col())?;
                    ast_vec.push(Ingot::ExternalFunction(ext));
                }
                Token::KeywordIsr => {
                    let isr: IsrNode = self.parse_isr(token.line(), token.col())?;
                    ast_vec.push(Ingot::Isr(isr));
                }
                other => {
                    return Err(Box::new(CompilationError::new(
                        token.line(),
                        token.col(),
                        format!("Expected the start of an Ingot, got {}", other),
                    )));
                }
            }
        }

        Ok(Ast::new(ast_vec))
    }

    fn parse_extern(
        &mut self,
        last_line: u32,
        last_col: u32,
    ) -> Result<ExternalFunctionNode, CompilationError> {
        match self.tokens.pop_front() {
            // found something, need to see if it's a func
            Some(func) => match func.borrow_token() {
                // found a func name, need to see if '(' is next
                Token::Identifier(func_name) => match self.tokens.pop_front() {
                    // found something, need to see if it's '('
                    Some(lparen) => match lparen.borrow_token() {
                        Token::LeftParen => {
                            let args = self.parse_func_args()?;
                            let mut privilege: Option<Ring> = None;
                            if let Some(token) = self.tokens.front()
                                && let &Token::KeywordWithLevel = token.borrow_token()
                            {
                                privilege =
                                    Some(self.parse_withlevel(lparen.line(), lparen.col())?);
                            }

                            // weird rust return expression
                            Ok(ExternalFunctionNode::new(
                                func_name.clone(),
                                args,
                                privilege,
                            ))
                        }
                        other_token => Err(CompilationError::new(
                            lparen.line(),
                            lparen.col(),
                            format!(
                                "Expected '(' after extern declaration, found: {}",
                                other_token
                            ),
                        )),
                    },
                    None => Err(CompilationError::new(
                        func.line(),
                        func.col(),
                        String::from("Expected '(' after extern declaration, found EOF"),
                    )),
                },
                other_token => Err(CompilationError::new(
                    func.line(),
                    func.col(),
                    format!(
                        "Expected function name after extern declaration, found: {}",
                        other_token
                    ),
                )),
            },
            None => Err(CompilationError::new(
                last_line,
                last_col,
                String::from("Expected function name after extern declaration, found EOF"),
            )),
        }
    }

    fn parse_func_args(&mut self) -> Result<Vec<FuncArg>, CompilationError> {
        #[derive(Clone, Copy)]
        enum SeekState {
            Start,
            SeekArg,
            SeekType,
        }
        let mut args: Vec<FuncArg> = Vec::new();
        let mut state: SeekState = SeekState::Start;

        let mut type_t: FuncArgType = FuncArgType::U8;

        while let Some(token) = self.tokens.pop_front() {
            match (token.borrow_token(), state) {
                (Token::RightParen, SeekState::SeekType | SeekState::Start) => break,
                (Token::RightParen, SeekState::SeekArg) => {
                    return Err(CompilationError::new(
                        token.line(),
                        token.col(),
                        String::from("Expected arg name for function argument, found ')'"),
                    ));
                }
                (Token::Identifier(type_str), SeekState::SeekType | SeekState::Start) => {
                    type_t = match type_str.as_str() {
                        "u8" => FuncArgType::U8,
                        "i8" => FuncArgType::I8,
                        "u16" => FuncArgType::U16,
                        "i16" => FuncArgType::I16,
                        "u32" => FuncArgType::U32,
                        "i32" => FuncArgType::I32,
                        "u64" => FuncArgType::U64,
                        "i64" => FuncArgType::I64,
                        other => {
                            return Err(CompilationError::new(
                                token.line(),
                                token.col(),
                                format!("Invalid c type: {}", other),
                            ));
                        }
                    };

                    state = SeekState::SeekArg;
                }
                (Token::Comma, SeekState::Start) => {
                    return Err(CompilationError::new(
                        token.line(),
                        token.col(),
                        String::from("Expected ')' or fun arg, got ','"),
                    ));
                }
                (Token::Comma, SeekState::SeekType) => continue,
                (Token::Comma, SeekState::SeekArg) => continue,
                (Token::Identifier(arg), SeekState::SeekArg) => {
                    state = SeekState::SeekType;
                    args.push(FuncArg::new(arg.clone(), type_t));
                }
                (unexpected, _) => {
                    return Err(CompilationError::new(
                        token.line(),
                        token.col(),
                        format!("Unexpected token while parsing func_args: {}", unexpected),
                    ));
                }
            }
        }

        Ok(args)
    }

    fn parse_withlevel(
        &mut self,
        last_line: u32,
        last_column: u32,
    ) -> Result<Ring, CompilationError> {
        match self.tokens.pop_front() {
            Some(token) => match token.borrow_token() {
                Token::KeywordWithLevel => match self.tokens.pop_front() {
                    Some(lparen) => match lparen.borrow_token() {
                        Token::LeftParen => match self.tokens.pop_front() {
                            Some(priv_level) => match priv_level.borrow_token() {
                                Token::KeywordPrivilege(level) => match self.tokens.pop_front() {
                                    Some(token) => match token.borrow_token() {
                                        Token::RightParen => Ok(*level),
                                        other_token => Err(CompilationError::new(
                                            token.line(),
                                            token.col(),
                                            format!(
                                                "Expected ')' after privilege level, found {}",
                                                other_token
                                            ),
                                        )),
                                    },
                                    None => Err(CompilationError::new(
                                        priv_level.line(),
                                        priv_level.col(),
                                        String::from(
                                            "Expected ')' after privilege level, found EOF",
                                        ),
                                    )),
                                },
                                other_token => Err(CompilationError::new(
                                    priv_level.line(),
                                    priv_level.col(),
                                    format!("Expected privilege level, found: {}", other_token),
                                )),
                            },
                            None => Err(CompilationError::new(
                                lparen.line(),
                                lparen.col(),
                                String::from("Expected privilege level, found EOF"),
                            )),
                        },
                        other_token => Err(CompilationError::new(
                            lparen.line(),
                            lparen.col(),
                            format!("Expected '(' after WithLevel, found: {}", other_token),
                        )),
                    },
                    None => Err(CompilationError::new(
                        token.line(),
                        token.col(),
                        String::from("Expected '(' after WithLevel, found EOF"),
                    )),
                },
                other_token => Err(CompilationError::new(
                    token.line(),
                    token.col(),
                    format!("Expected 'WithLevel', found: {}", other_token),
                )),
            },
            None => Err(CompilationError::new(
                last_line,
                last_column,
                String::from("Expected 'WithLevel', found EOF"),
            )),
        }
    }

    fn parse_isr(&mut self, last_line: u32, last_column: u32) -> Result<IsrNode, CompilationError> {
        match self.tokens.pop_front() {
            Some(name) => match name.borrow_token() {
                Token::Identifier(isr_name) => match self.tokens.pop_front() {
                    Some(lparen) => match lparen.borrow_token() {
                        Token::LeftParen => match self.tokens.pop_front() {
                            Some(num) => match num.borrow_token() {
                                Token::Identifier(isr_num_str) => {
                                    // Isr Numbers above 255 are rare to my knowledge
                                    match isr_num_str.parse::<u8>() {
                                        Ok(isr_num) => match self.tokens.pop_front() {
                                            Some(rparen) => match rparen.borrow_token() {
                                                Token::RightParen => {
                                                    let mut privilege: Option<Ring> = None;
                                                    if let Some(token) = self.tokens.front()
                                                        && let Token::KeywordWithLevel =
                                                            token.borrow_token()
                                                    {
                                                        privilege = Some(self.parse_withlevel(
                                                            num.line(),
                                                            num.col(),
                                                        )?);
                                                    }

                                                    let calling_func =
                                                        self.parse_isr_body(num.line(), num.col())?;
                                                    Ok(IsrNode::new(
                                                        isr_name.clone(),
                                                        isr_num,
                                                        privilege,
                                                        calling_func,
                                                    ))
                                                }
                                                other_token => Err(CompilationError::new(
                                                    rparen.line(),
                                                    rparen.col(),
                                                    format!(
                                                        "Expected closing ')', found: {}",
                                                        other_token
                                                    ),
                                                )),
                                            },
                                            None => Err(CompilationError::new(
                                                num.line(),
                                                num.col(),
                                                String::from("Expected closing ')', found EOF"),
                                            )),
                                        },
                                        Err(e) => Err(CompilationError::new(
                                            num.line(),
                                            num.col(),
                                            format!("Expected numeric value, failed with: {}", e),
                                        )),
                                    }
                                }
                                other_token => Err(CompilationError::new(
                                    num.line(),
                                    num.col(),
                                    format!("Expected an Isr number, found: {}", other_token),
                                )),
                            },
                            None => Err(CompilationError::new(
                                lparen.line(),
                                lparen.col(),
                                String::from(""),
                            )),
                        },
                        other_token => Err(CompilationError::new(
                            lparen.line(),
                            lparen.col(),
                            format!("Expected '(' after Isr name, found: {}", other_token),
                        )),
                    },
                    None => Err(CompilationError::new(
                        name.line(),
                        name.col(),
                        String::from("Expected '(' after Isr name, found EOF"),
                    )),
                },
                other_token => Err(CompilationError::new(
                    name.line(),
                    name.col(),
                    format!("Expected ISR name, found: {}", other_token),
                )),
            },
            None => Err(CompilationError::new(
                last_line,
                last_column,
                String::from("Expected ISR name, found EOF"),
            )),
        }
    }

    fn parse_isr_body(
        &mut self,
        last_line: u32,
        last_column: u32,
    ) -> Result<Option<ExternalFunctionCall>, CompilationError> {
        match self.tokens.pop_front() {
            Some(lbracket) => match lbracket.borrow_token() {
                Token::LeftBracket => match self.tokens.pop_front() {
                    Some(remaining) => match remaining.borrow_token() {
                        Token::KeywordCall => Ok(Some(
                            self.parse_function_call(remaining.line(), remaining.col())?,
                        )),
                        Token::RightBracket => Ok(None),
                        other => Err(CompilationError::new(
                            remaining.line(),
                            remaining.col(),
                            format!("Expected call expression or closing '}}', found: {}", other),
                        )),
                    },
                    None => Err(CompilationError::new(
                        lbracket.line(),
                        lbracket.col(),
                        String::from("Expected call expression or closing '}', found EOF"),
                    )),
                },
                other_token => Err(CompilationError::new(
                    lbracket.line(),
                    lbracket.col(),
                    format!("Expected opening '{{', found: {}", other_token),
                )),
            },
            None => Err(CompilationError::new(
                last_line,
                last_column,
                String::from("Expected opening '{', found EOF"),
            )),
        }
    }

    fn parse_function_call(
        &mut self,
        last_line: u32,
        last_col: u32,
    ) -> Result<ExternalFunctionCall, CompilationError> {
        #[derive(Clone, Copy)]
        enum SeekState {
            Start,
            SeekArg,
            SeekComma,
        }

        match self.tokens.pop_front() {
            Some(func) => match func.borrow_token() {
                Token::Identifier(func_name) => match self.tokens.pop_front() {
                    Some(lparen) => match lparen.borrow_token() {
                        Token::LeftParen => {
                            let mut args: Vec<CallArg> = Vec::new();
                            let mut state: SeekState = SeekState::Start;

                            while let Some(token) = self.tokens.pop_front() {
                                match (token.borrow_token(), state) {
                                    (
                                        Token::RightParen,
                                        SeekState::Start | SeekState::SeekComma,
                                    ) => break,
                                    (Token::RightParen, SeekState::SeekArg) => {
                                        return Err(CompilationError::new(
                                            token.line(),
                                            token.col(),
                                            String::from("Expected argument, found ')'"),
                                        ));
                                    }
                                    (Token::Identifier(arg), SeekState::SeekArg) => {
                                        args.push(CallArg::Var(arg.clone()));
                                        state = SeekState::SeekComma;
                                    }
                                    (Token::Dollar, SeekState::Start | SeekState::SeekArg) => {
                                        args.push(CallArg::Dollar);
                                        state = SeekState::SeekComma;
                                    }
                                    (unexpected, SeekState::Start | SeekState::SeekArg) => {
                                        return Err(CompilationError::new(
                                            token.line(),
                                            token.col(),
                                            format!(
                                                "Unexpected token in call expression: {}",
                                                unexpected
                                            ),
                                        ));
                                    }
                                    (Token::Comma, SeekState::SeekComma) => {
                                        state = SeekState::SeekArg;
                                    }
                                    (unexpected, SeekState::SeekComma) => {
                                        return Err(CompilationError::new(
                                            token.line(),
                                            token.col(),
                                            format!(
                                                "Unexpected token in call expression: {}",
                                                unexpected
                                            ),
                                        ));
                                    }
                                }
                            }

                            match self.tokens.pop_front() {
                                Some(rbracket) => match rbracket.borrow_token() {
                                    Token::RightBracket => {
                                        Ok(ExternalFunctionCall::new(func_name.clone(), args))
                                    }
                                    other => Err(CompilationError::new(
                                        rbracket.line(),
                                        rbracket.col(),
                                        format!("Expected '}}', found: {}", other),
                                    )),
                                },
                                None => Err(CompilationError::new(
                                    func.line(),
                                    func.col(),
                                    String::from("Expected '}', found EOF"),
                                )),
                            }
                        }
                        other => Err(CompilationError::new(
                            lparen.line(),
                            lparen.col(),
                            format!("Expected '(', found: {}", other),
                        )),
                    },
                    None => Err(CompilationError::new(
                        func.line(),
                        func.col(),
                        String::from("Expected '(', found EOF"),
                    )),
                },
                other => Err(CompilationError::new(
                    func.line(),
                    func.col(),
                    format!("Expected function name, found: {}", other),
                )),
            },
            None => Err(CompilationError::new(
                last_line,
                last_col,
                String::from("Expected function name, found EOF"),
            )),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn total_parse() {}

    #[test]
    fn only_externs() {}

    #[test]
    fn only_isrs() {}
}
