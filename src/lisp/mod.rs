/// The lisp that codebake embeds as a scripting language
/// 
/// This file contains the struct and enum definitions as well as
/// some top-level functions like `default_env` and `run_repl`.
/// 
/// Much of this lisp was written using this AMAZING tutorial!
/// https://stopa.io/post/222
///

mod functions;
mod parser;
mod eval;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};
use std::rc::Rc;
use crate::{Dish, DishData};
use crate::lisp::parser::parse_eval;
use crate::lisp::functions::*;

pub type LispResult = std::result::Result<Expression, Error>;

/// Every expression in the embedded lisp is a variant
/// of this enumeration:
///   * Symbol - a raw symbol
///   * Number - a floating point number
///   * Bool   - a boolean value (`true` and `false`)
///   * String - a string
///   * List   - a list of expressions
///   * Func   - a pointer to a function object 
///   * Lambda - an expression with a set of captured variables
///   * Dish   - a pointer to a **mutable** Dish object
/// 
#[derive(Clone)]
pub enum Expression {
    Symbol(String),
    Number(f64),
    Bool(bool),
    String(String),
    List(Vec<Expression>),
    Func(Rc<dyn Fn(&[Expression]) -> LispResult>),
    Lambda(Lambda),
    Dish(Rc<RefCell<Dish>>),
}

/// Just a newtype'd String
/// since we don't need complex error representation
#[derive(Debug)]
pub struct Error(String);

#[derive(Clone)]
/// The environment that the lisp is operating in.
/// 
/// The `data` field contains a hashmap of Strings -> Expressions
/// for the interpreter
/// 
pub struct Environment<'a> {
    data: HashMap<String, Expression>,
    outer: Option<&'a Environment<'a>>
}

#[derive(Clone)]
pub struct Lambda {
    params: Rc<Expression>,
    body: Rc<Expression>,
}


impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Expression::Symbol(k) => k.clone(),
            Expression::Number(k) => k.to_string(),
            Expression::Bool(k) => k.to_string(),
            Expression::String(k) => k.clone(),
            Expression::List(k) => {
                let xs: Vec<String> = k
                    .iter()
                    .map(|x| x.to_string())
                    .collect();
                format!("({})", xs.join(" "))
            },
            Expression::Func(_) => "built-in function".to_string(),
            Expression::Lambda(_) => "lambda function".to_string(),
            Expression::Dish(dish) => {
                // so much deref
                let deref = &*dish;
                match &*deref.borrow() {
                    Dish::Success(data) => match data {
                        DishData::Str(s) => format!("Dish(\"{}\")", s),
                        DishData::Bin(b) => format!("Dish([{:?}])", b),
                    },
                    Dish::Failure(e) => format!("dish error: {}", e.reason),
                }
            }
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Starts a repl on stdin and blocks until either
/// an error occurs or stdin is closed
/// 
pub fn run_repl(env: Option<&mut Environment>) {
    let mut maybeenv: Box<Environment>;
    let mut env = match env {
        Some(env) => env,
        None => {
            maybeenv = Box::new(default_env());
            &mut maybeenv
        }
    };
    let stdin = io::stdin();

    loop {
        let mut expr = String::new();
        print!("; ");
        io::stdout().flush()
            .expect("failed to flush output");
    
        match stdin.read_line(&mut expr) {
            Ok(0) => return,
            Ok(_) => {},
            Err(e) => panic!("{}", e),
        }

        match parse_eval(expr, &mut env) {
            Ok(res) => println!("{}", res),
            Err(e) => println!("error: {}", e),
        }
    }
}

/// Returns an instance of Environment that contains
/// all the builtin functions and values
/// 
fn default_env<'a>() -> Environment<'a> {
    let mut data: HashMap<String, Expression> = HashMap::new();
    data.insert("+".to_string(), lisp_add());
    data.insert("-".to_string(), lisp_subtract());
    data.insert("dish".to_string(), lisp_dish());
    data.insert("rot13".to_string(), lisp_rot13());
    data.insert("reverse".to_string(), lisp_reverse());

    Environment { data, outer: None, }
}
