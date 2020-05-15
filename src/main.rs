use ojisan_fuck::interpreter::Interpreter;

fn main() {
    let code = "++++++[>++++++++<-]++++++++++[>.+<-]";
    let mut interpreter = Interpreter::transpile_from(code);
    interpreter.eval();
}