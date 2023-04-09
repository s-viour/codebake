use codebake::lisp;
fn main() {
    // popy
    let mut codebake = lisp::Interpreter::default();
    codebake.run_repl();
}
