use ojisan_fuck::interpreter::Interpreter;

fn main() {
    let code = "😘😘😘😘😘😘😘😘✋😚😅😘😘😘😘😘😘😘😘😘😭🤟😅💦";
    let mut interpreter = Interpreter::new(code);
    interpreter.eval();
    println!("{:?}", interpreter.instructions);
    println!("{:?}", interpreter.memory);
}