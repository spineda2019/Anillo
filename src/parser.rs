use std::collections::VecDeque;

use crate::token::{Ast, ExternalFunctionNode, FuncArg, Ingot, IsrNode, Token};

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

        Ast(ast_vec)
    }

    fn parse_extern(&mut self) -> ExternalFunctionNode {
        if let Some(Token::Identifier(func_name)) = self.tokens.pop_front()
            && let Some(Token::LeftParen) = self.tokens.pop_front()
        {
            // We're lucky we have chained let binding now. IIRC this was unstable for a bit

            let args = self.parse_func_args();
            if let Some(&Token::KeywordWithLevel) = self.tokens.front() {
            } else {
                return ExternalFunctionNode {
                    name: func_name,
                    args,
                    privilege: None,
                };
            }
            match self.tokens.front() {
                Some(&Token::KeywordWithLevel) => {}
                _ => {}
            }
            dbg!(&func_name, &self.tokens);
            todo!();
        } else {
            panic!("");
        }
    }

    fn parse_func_args(&mut self) -> Vec<FuncArg> {
        todo!()
    }

    fn parse_isr(&mut self) -> IsrNode {
        todo!();
    }
}
