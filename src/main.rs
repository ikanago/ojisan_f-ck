use clap::{clap_app, crate_description, crate_version};
use ojisan_fuck::interpreter::Interpreter;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let matches = clap_app!(ojisan_fuck =>
        (version: crate_version!())
        (about: crate_description!())
        (@arg INPUT: +required "Input source file/code.")
        (@arg code: -c --code "If specified, read source code from command line argument.")
        (@arg transpile: -t --transpile "If specified, transpile from basic brainfuck code.")
    )
    .get_matches();

    if let Some(input) = matches.value_of("INPUT") {
        if matches.is_present("transpile") {
            println!("{}", ojisan_fuck::transpile_from(input));
            return Ok(());
        }

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer)?;

        let code = if !matches.is_present("code") {
            let mut file = File::open(input)?;
            let mut code = String::new();
            file.read_to_string(&mut code)?;
            code
        } else {
            input.to_string()
        };

        let mut interpreter = Interpreter::new(&code, buffer);
        match interpreter.eval() {
            Ok(_) => println!("{}", interpreter.output()),
            Err(err) => {
                eprintln!("{:?}", err);
            }
        };
    }
    Ok(())
}
