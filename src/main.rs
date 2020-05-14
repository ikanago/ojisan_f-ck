use ojisan_fuck::interpreter::Interpreter;

fn main() {
    let mut interpreter = Interpreter::new("😅😘😘😘😘");
    interpreter.eval();
    println!("{:?}", interpreter.instructions);
    println!("{:?}", interpreter.memory);
}