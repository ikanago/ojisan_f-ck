#[macro_use]
extern crate clap;

use ojisan_fuck::interpreter::Interpreter;
use std::io::{self, Read};

fn main() -> Result<(), io::Error>{
    let matches = clap_app!(ojisan_fuck =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg CODE: +required "Input source code.")
        (@arg transpile: --transpile "If specified, transpile from basic brainfuck code.")
    )
    .get_matches();

    if let Some(ref code) = matches.value_of("CODE") {
        if matches.is_present("transpile") {
            println!("{}", ojisan_fuck::transpile_from(code));
        } else {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_string(&mut buffer)?;

            let mut interpreter = Interpreter::new(code, buffer);
            match interpreter.eval() {
                Ok(_) => println!("{}", interpreter.output_buffer.iter().collect::<String>()),
                Err(err) => {
                    eprintln!("{:?}", err);
                }
            };
        };
    } else {
        eprintln!("Specify source code.");
        panic!();
    }
    Ok(())
}
