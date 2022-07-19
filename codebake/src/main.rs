use codebake::lisp;

use regex::Regex;

fn main() {
    // lisp::run_repl(None);
    
    let re = Regex::new(".").unwrap();
    let v = Vec::new();
    
    for m in re.find_iter("nice") {
        let e = m.as_str().to_string();
    
        v.push(&e);
        println!("{}", e)
    }
    
    println!("{:#?}", v.join("\n"))
}
