use std::{collections::VecDeque, fmt::Display};

use argparse::{ArgumentParser, Store};

use crate::parser::Parser;
mod error;
mod lexer;
mod parser;
mod token;

fn main() -> std::io::Result<()> {
    let mut input_file = std::path::PathBuf::new();
    {
        let mut arg_parser: ArgumentParser = ArgumentParser::new();
        arg_parser.set_description("Parse your Anillo file");
        arg_parser
            .refer(&mut input_file)
            .add_argument("input", Store, "anillo file to parse")
            .required();

        arg_parser.parse_args_or_exit();
    }

    if !input_file.ends_with(".ani") {
        eprintln!("Not an anillo file ending in '.ani': {:?}", input_file);
    } else {
        println!("Parsing input: {:?}", input_file);
    }

    let mut lexer = lexer::Lexer::new(&input_file)?;
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(VecDeque::from(tokens));

    dbg!(&parser);

    let ast = parser.run();

    dbg!(&ast);

    Ok(())
}
