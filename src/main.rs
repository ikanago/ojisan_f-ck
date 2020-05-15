#[macro_use]
extern crate clap;

use ojisan_fuck::interpreter::Interpreter;

fn main() {
    let matches = clap_app!(ojisan_fuck =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg CODE: +required "Input source code.")
        (@arg transpile: --transpile "If specified, transpile from basic brainfuck code.")
    )
    .get_matches();

    if let Some(ref code) = matches.value_of("CODE") {
        let mut interpreter = if matches.is_present("transpile") {
            Interpreter::transpile_from(code)
        } else {
            Interpreter::new(code)
        };
        interpreter.eval();
    } else {
        eprintln!("Specify source code.");
        panic!();
    }
}
