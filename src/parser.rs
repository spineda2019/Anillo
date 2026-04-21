use std::collections::VecDeque;

use crate::token::{Ast, ExternalFunctionNode, FuncArg, FuncArgType, Ingot, IsrNode, Ring, Token};

#[derive(Debug)]
pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: VecDeque<Token>) -> Parser {
        Parser { tokens }
    }

    pub fn run(&mut self) -> Ast {
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
            match token {
                Token::KeywordExtern => {
                    let ext = self.parse_extern();
                    ast_vec.push(Ingot::ExternalFunction(ext));
                }
                Token::KeywordIsr => {
                    let isr = self.parse_isr();
                    ast_vec.push(Ingot::Isr(isr));
                }
                other => panic!("Expected the start of an Ingot, got {}", other),
            }
        }

        Ast::new(ast_vec)
    }

    fn parse_extern(&mut self) -> ExternalFunctionNode {
        if let Some(Token::Identifier(func_name)) = self.tokens.pop_front()
            && let Some(Token::LeftParen) = self.tokens.pop_front()
        {
            // We're lucky we have chained let binding now. IIRC this was unstable for a bit

            let args = self.parse_func_args();
            let mut privilege: Option<Ring> = None;
            if let Some(&Token::KeywordWithLevel) = self.tokens.front() {
                privilege = match self.tokens.pop_front() {
                    None => None,
                    _ => todo!(),
                };
            }
            return ExternalFunctionNode::new(func_name, args, privilege);
        } else {
            panic!("Message TBD");
        }
    }

    fn parse_func_args(&mut self) -> Vec<FuncArg> {
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
            match (token, state) {
                (Token::RightParen, SeekState::SeekType | SeekState::Start) => break,
                (Token::RightParen, SeekState::SeekArg) => {
                    panic!("Expected arg name for function argument, found ')'")
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
                        other => panic!("Invalid c type: {}", other),
                    };

                    state = SeekState::SeekArg;
                }
                (Token::Comma, SeekState::Start) => panic!("Expected ')' or fun arg, got ','"),
                (Token::Comma, SeekState::SeekType) => continue,
                (Token::Comma, SeekState::SeekArg) => continue,
                (Token::Identifier(arg), SeekState::SeekArg) => {
                    state = SeekState::SeekType;
                    args.push(FuncArg::new(arg, type_t));
                }
                (unexpected, _) => {
                    panic!("Unexpected token while parsing func_args: {}", unexpected)
                }
            }
        }

        args
    }

    fn parse_isr(&mut self) -> IsrNode {
        if let Some(Token::LeftBracket) = self.tokens.pop_front() {
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
