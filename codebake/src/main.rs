use codebake::lisp;
fn main() {
    let mut codebake = lisp::Interpreter::default();
    codebake.run_repl();
}
