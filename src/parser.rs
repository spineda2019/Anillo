use std::{collections::VecDeque, error::Error, fmt::format};

use crate::{
    error::CompilationError,
    token::{
        Ast, ExternalFunctionNode, FuncArg, FuncArgType, Ingot, IsrNode, Ring, Token, TokenInfo,
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

    pub fn run(&mut self) -> Result<Ast, Box<dyn Error>> {
        let mut ast_vec: Vec<Ingot> = Vec::new();

        while let Some(token) = self.tokens.pop_front() {
            println!("###########################################################################");
            println!("Consumed token: {}", token);
            println!("Remaining tokens:");
            dbg!(&self.tokens);
            println!("AST in progress:");
            dbg!(&ast_vec);
            println!();
            println!("###########################################################################");

            match token.borrow_token() {
                Token::KeywordExtern => {
                    let ext = self.parse_extern(token.line(), token.col())?;
                    ast_vec.push(Ingot::ExternalFunction(ext));
                }
                Token::KeywordIsr => {
                    let isr = self.parse_isr();
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
                                _ => todo!(),
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

    fn parse_isr(&mut self) -> IsrNode {
        if let Some(token) = self.tokens.pop_front()
            && let Token::LeftBracket = token.borrow_token()
        {
            println!("TODO! ISR Body parsing");
        } else {
            panic!("Expected left bracket after ISR definition");
        }
        todo!();
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
