//! The Anillo Compiler
//!
//! The Anillo compiler serves as the currently (Work In Progress) reference implementation
//! of the Anillo Language.

use std::{collections::VecDeque, error::Error};

use argparse::{ArgumentParser, Store, StoreTrue, StoreOption};

mod error;
mod lexer;
mod parser;
mod token;
mod compilerc;

use parser::Parser;
use token::TokenInfo;

use crate::compilerc::CompilerC;

/// The main just parses command line args and drives the lexer, parser, and AST
/// validator.
///
/// # Arguments
/// * **_input_**
///   Single positional argument that dictates the .ani Anillo file to parse
/// * **_--verbose_**
///   Enables in-progress printing of the AST as it is being built as
///   well as other useful information as the compiler runs (such as
///   the initial token buffer produced by the lexer)
/// * **_--comp \<filename>_**
///   Produces 2 C files, _\<filename>.h_ and _\<filename>.c_, which declare and define a function _AnilloISRRegister()_ that initializes and registers the described IDT from the Anillo source \<UNFINISHED> 
/// * **_-h_** or **_--help_**
///   Prints command line help
fn main() -> Result<(), Box<dyn Error>> {
    let mut input_file = std::path::PathBuf::new();
    let mut verbose: bool = false;
    let mut output_file: Option<String> = None;

    {
        let mut arg_parser: ArgumentParser = ArgumentParser::new();
        arg_parser.set_description("Parse your Anillo file");
        arg_parser
            .refer(&mut input_file)
            .add_argument("input", Store, "anillo file to parse")
            .required();
        arg_parser.refer(&mut verbose).add_option(
            &["--verbose"],
            StoreTrue,
            "Print in progress Lexer, parser, and AST state",
        );
        arg_parser.refer(&mut output_file).add_option(
            &["--comp"],
            StoreOption,
            "File name to store compiled C code for ISR registration",
        );

        arg_parser.parse_args_or_exit();
    }

    if let Some(ext) = input_file.extension()
        && let Some(ext_str) = ext.to_str()
        && ext_str == "ani"
    {
        println!("Parsing input: {}", input_file.display());
    } else {
        eprintln!(
            "Not an anillo file ending in '.ani': {}",
            input_file.display()
        );
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Not an anillo file ending in '.ani'",
        )));
    }

    let mut lexer = lexer::Lexer::new(&input_file)?;
    let tokens: VecDeque<TokenInfo> = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);

    if verbose {
        println!(
            "********************************************************************************"
        );
        println!("Starting token buffer:");
        dbg!(&parser);
        println!(
            "********************************************************************************"
        );
    }

    let ast = parser.run(verbose)?;

    if verbose {
        println!(
            "********************************************************************************"
        );
        println!("Final AST:");
        dbg!(&ast);
        println!(
            "********************************************************************************"
        );
    }

    ast.verify()?;
    println!("{} passed verification!", input_file.display());


    if let Some(filename) = output_file {
        let compiler = CompilerC::new(&filename, &ast, compilerc::Target::X86_64);
        compiler.compile()?;
        println!("C code written to {0}.h and {0}.c", &filename);
    }

    Ok(())
}
