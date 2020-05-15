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
        if matches.is_present("transpile") {
            println!("{}", ojisan_fuck::transpile_from(code));
        } else {
            let mut interpreter = Interpreter::new(code);
            interpreter.eval();
        };
    } else {
        eprintln!("Specify source code.");
        panic!();
    }
}
