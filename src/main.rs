use ojisan_fuck::instractions;
use std::convert::From;

fn main() {
    let i = instractions::Instractions::from('😘');
    println!("{:?}", i);
}