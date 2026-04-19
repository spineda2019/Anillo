use argparse::{ArgumentParser, Store};
mod error;
mod lexer;
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
    let tokens = lexer.tokenize();

    Ok(())
}
