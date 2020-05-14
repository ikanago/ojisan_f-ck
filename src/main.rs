use ojisan_fuck::instractions;
use std::str::FromStr;

fn main() {
    let i = instractions::Instractions::from_str("😘").unwrap();
    println!("{:?}", i);
}