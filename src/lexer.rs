use std::io::Read;

use crate::token;

pub struct Lexer {
    file: std::fs::File,
}

impl Lexer {
    pub fn new(path: &std::path::Path) -> std::io::Result<Lexer> {
        Ok(Lexer {
            file: std::fs::File::open(path)?,
        })
    }

    pub fn tokenize(&mut self) -> std::io::Result<Vec<token::Token>> {
        let reader: std::io::BufReader<_> = std::io::BufReader::new(&self.file);
        for letter in reader.bytes() {
            let letter = letter?;
            let _ = dbg!(letter as char);
        }

        Ok(vec![])
    }
}
