use std::{collections::VecDeque, error::Error, io::Read, str::FromStr};

use crate::{
    error::CompilationError,
    token::{Ring, Token, TokenInfo},
};

pub struct Lexer {
    file: std::io::BufReader<std::fs::File>,
}

impl Lexer {
    pub fn new(path: &std::path::Path) -> std::io::Result<Lexer> {
        let f = std::fs::File::open(path)?;
        Ok(Lexer {
            file: std::io::BufReader::new(f),
        })
    }

    pub fn tokenize(&mut self) -> Result<VecDeque<TokenInfo>, Box<dyn Error>> {
        let mut tokens: VecDeque<TokenInfo> = VecDeque::new();
        let mut buf: [u8; 1] = [0; 1];
        let mut str_token_buf: Vec<char> = Vec::new();

        let mut line: u32 = 1;
        let mut col: u32 = 1;

        while self.file.read_exact(&mut buf).is_ok() {
            let letter = buf[0] as char;
            match letter {
                '(' | '{' | ')' | '}' => {
                    col += 1;
                    if !str_token_buf.is_empty() {
                        let remaining: String = str_token_buf.iter().collect();
                        str_token_buf.clear();
                        let tok: Token = Self::parse_str(&remaining);
                        tokens.push_back(TokenInfo::new(tok, line, col));
                    }
                    let tok: Token = match letter {
                        '(' => Token::LeftParen,
                        '{' => Token::LeftBracket,
                        ')' => Token::RightParen,
                        '}' => Token::RightBracket,
                        _ => unreachable!(),
                    };

                    tokens.push_back(TokenInfo::new(tok, line, col));
                }
                ',' => {
                    col += 1;
                    if !str_token_buf.is_empty() {
                        let str_token: String = str_token_buf.iter().collect();
                        let tok: Token = Self::parse_str(&str_token);
                        tokens.push_back(TokenInfo::new(tok, line, col));
                        str_token_buf.clear();
                    }

                    tokens.push_back(TokenInfo::new(Token::Comma, line, col));
                }
                white if white.is_ascii_whitespace() => {
                    if white == '\n' {
                        line += 1;
                        col = 0;
                    } else {
                        col += 1;
                    }
                    if !str_token_buf.is_empty() {
                        let str_token: String = str_token_buf.iter().collect();
                        let tok: Token = Self::parse_str(&str_token);
                        tokens.push_back(TokenInfo::new(tok, line, col));
                        str_token_buf.clear();
                    }
                }
                asc if asc.is_ascii_alphanumeric() => {
                    col += 1;
                    str_token_buf.push(letter);
                }
                other => {
                    col += 1;
                    return Err(Box::new(CompilationError::new(
                        line,
                        col,
                        format!("Unrecognized character type: {}", other),
                    )));
                }
            }
        }

        Ok(tokens)
    }

    fn parse_str(tok: &str) -> Token {
        match tok {
            "extern" => Token::KeywordExtern,
            "WithLevel" => Token::KeywordWithLevel,
            "User" => Token::KeywordPrivilege(Ring::User),
            "Super" => Token::KeywordPrivilege(Ring::Super),
            "isr" => Token::KeywordIsr,
            id => Token::Identifier(String::from_str(id).expect("Invalid str for identifier")),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn valid_tokens() {
        const EXAMPLE: &str = "extern Func()    ";
    }

    #[test]
    fn invalid_token() {
        const EXAMPLE: &str = "extern Func() #$%^";
    }
}
