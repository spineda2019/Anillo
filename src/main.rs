use std::{collections::VecDeque, error::Error};

use argparse::{ArgumentParser, Store};

mod error;
mod lexer;
mod parser;
mod token;

use parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
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

    if let Some(ext) = input_file.extension()
        && let Some(ext_str) = ext.to_str()
        && ext_str == "ani"
    {
        println!("Parsing input: {:?}", input_file);
    } else {
        eprintln!("Not an anillo file ending in '.ani': {:?}", input_file);
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Not an anillo file ending in '.ani'",
        )));
    }

    let mut lexer = lexer::Lexer::new(&input_file)?;
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(VecDeque::from(tokens));

    println!("********************************************************************************");
    println!("Starting token buffer:");
    dbg!(&parser);
    println!("********************************************************************************");

    let ast = parser.run();

    dbg!(&ast);

    Ok(())
}
