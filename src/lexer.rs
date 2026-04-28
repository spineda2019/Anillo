use std::{collections::VecDeque, error::Error, io::Read, str::FromStr};

use crate::{
    error::CompilationError,
    token::{Ring, Token, TokenInfo},
};

/// The Anillo Lexer
///
/// The Anillo Lexer is purposefully simple. Lexemes (or Tokens here) are only
/// delimited by whitespace and "Special Characters" (these are defined in the
/// techincal specification, but generally punctuation like brackets, or special
/// semantic tokens like '$' count as "Special Characters")
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

    /// Eagerly lex the ModuleSource and return a buffer of `TokenInfo`s to the
    /// caller.
    ///
    /// Lexing here is done as simply as possible (and the language was designed
    /// with this in mind). There should be absolutely no look ahead peeking
    /// done at this level. Compilation errors at this level are supported, but
    /// rare (only really possible if an invalid unicode character is found).
    pub fn tokenize(&mut self) -> Result<VecDeque<TokenInfo>, Box<dyn Error>> {
        let mut tokens: VecDeque<TokenInfo> = VecDeque::new();
        let mut buf: [u8; 1] = [0; 1];
        let mut str_token_buf: Vec<char> = Vec::new();

        let mut line: u32 = 1;
        let mut col: u32 = 1;

        // Many conditions can trigger a "drain" of our current raw char-form
        // token buffer. This is just a macro that wraps up the repetitive
        // code into a single call.
        macro_rules! drain_if_needed {
            () => {
                if !str_token_buf.is_empty() {
                    let remaining: String = str_token_buf.iter().collect();
                    str_token_buf.clear();
                    let tok: Token = Self::parse_str(&remaining);
                    tokens.push_back(TokenInfo::new(tok, line, col));
                }
            };
        }

        while self.file.read_exact(&mut buf).is_ok() {
            let letter = buf[0] as char;
            match letter {
                '(' | '{' | ')' | '}' => {
                    col += 1;
                    drain_if_needed!();
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
                    drain_if_needed!();
                    tokens.push_back(TokenInfo::new(Token::Comma, line, col));
                }
                '$' => {
                    col += 1;
                    drain_if_needed!();
                    tokens.push_back(TokenInfo::new(Token::Dollar, line, col));
                }
                white if white.is_ascii_whitespace() => {
                    if white == '\n' {
                        line += 1;
                        col = 0;
                    } else {
                        col += 1;
                    }
                    drain_if_needed!();
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

    /// Helper function for `tokenize`.
    ///
    /// Essentially anything that isn't a Special Character will be lexed here.
    /// Where possible, we will store the lexeme in a rich token type (Like
    /// keywords). Everything else however will be parsed as identifiers (even
    /// numbers). The parser handles converting these to their expected types at
    /// AST generation time in `parser.rs`
    fn parse_str(tok: &str) -> Token {
        match tok {
            "extern" => Token::KeywordExtern,
            "WithLevel" => Token::KeywordWithLevel,
            "User" => Token::KeywordPrivilege(Ring::User),
            "Super" => Token::KeywordPrivilege(Ring::Super),
            "isr" => Token::KeywordIsr,
            "call" => Token::KeywordCall,
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
