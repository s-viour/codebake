//! The lisp that codebake embeds as a scripting language
//!
//! This file contains the struct and enum definitions as well as
//! some top-level functions like `default_env` and `run_repl`.
//!
//! Much of this lisp was written using this AMAZING tutorial!
//! https://stopa.io/post/222
//!

mod eval;
mod functions;
mod parser;
mod functions_nonnative;

pub use crate::lisp::parser::parse_eval;
use crate::ops::OPERATIONS;
use crate::Dish;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};
use std::rc::Rc;
use regex::Regex;
use lazy_static::lazy_static;

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
    outer: Option<&'a Environment<'a>>,
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
                let xs: Vec<String> = k.iter().map(|x| x.to_string()).collect();
                format!("({})", xs.join(" "))
            }
            Expression::Func(_) => "built-in function".to_string(),
            Expression::Lambda(_) => "lambda function".to_string(),
            Expression::Dish(dish) => {
                // so much deref
                let deref = &*dish;
                format!("{}", deref.borrow())
            },
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// i'm unsure if there's a better way to implement
// PartialEq for this. if anyone has a better way, lmk!
//
impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        // if the enumerations aren't 
        if std::mem::discriminant(self) != std::mem::discriminant(other) {
            return false;
        }

        if let Expression::Symbol(s1) = self {
            if let Expression::Symbol(s2) = other {
                return s1 == s2;
            } else {
                return false;
            }
        } else if let Expression::String(s1) = self {
            if let Expression::String(s2) = other {
                return s1 == s2
            } else {
                return false;
            }
        } else if let Expression::Number(s1) = self {
            if let Expression::Number(s2) = other {
                return s1 == s2;
            } else {
                return false;
            }
        } else if let Expression::Bool(s1) = self {
            if let Expression::Bool(s2) = other {
                return s1 == s2;
            } else {
                return false;
            }
        } else if let Expression::Dish(s1) = self {
            if let Expression::Dish(s2) = other {
                match &*s1.borrow() {
                    Dish::Failure(_) => return false,
                    Dish::Success(d1) => match &*s2.borrow() {
                        Dish::Failure(_) => return false,
                        Dish::Success(d2) => return d1 == d2,
                    }
                }
            } else {
                return false
            }
        }

        false
    }
}

/// Starts a repl on stdin and blocks until either
/// an error occurs or stdin is closed
///
pub fn run_repl(env: Option<&mut Environment>) {
    let mut maybeenv: Box<Environment>;
    let env = match env {
        Some(env) => env,
        None => {
            maybeenv = Box::new(default_env());
            &mut maybeenv
        }
    };
    let stdin = io::stdin();

    loop {
        let mut expr = String::new();
        print!("codebake> ");
        io::stdout().flush().expect("failed to flush output");

        loop {
            match stdin.read_line(&mut expr) {
                Ok(0) => return,
                Ok(_) => {}
                Err(e) => panic!("{}", e),
            }

            if check_parens(&expr) {
                break;
            }
        }

        match parse_eval(expr, env) {
            Ok(res) => println!("{}", res),
            Err(e) => println!("error: {}", e),
        }
    }
}

fn check_parens(s: &String) -> bool {
    let mut count = 0;
    let mut string_mode = false;
    for i in s.chars() {
        match i {
            '(' => if !string_mode { count += 1 },
            ')' => if !string_mode { count -= 1 },
            '\"' => string_mode = !string_mode,
            _ => {},
        }
        if count < 0 {
            return false;
        }
    }

    count == 0
}

/// Returns an instance of Environment that contains
/// all the builtin functions and values
///
pub fn default_env<'a>() -> Environment<'a> {
    let mut data: HashMap<String, Expression> = HashMap::new();
    data.insert("+".to_string(), functions::lisp_add());
    data.insert("-".to_string(), functions::lisp_subtract());
    data.insert("=".to_string(), functions::lisp_eq());
    data.insert("apply".to_string(), functions::lisp_apply());
    data.insert("first".to_string(), functions::lisp_head());
    data.insert("rest".to_string(), functions::lisp_rest());
    data.insert("butlast".to_string(), functions::lisp_init());
    data.insert("last".to_string(), functions::lisp_last());
    data.insert("empty?".to_string(), functions::lisp_empty());
    data.insert("cons".to_string(), functions::lisp_cons());

    data.insert("dish".to_string(), functions::lisp_dish());
    data.insert("recipe".to_string(), functions::lisp_recipe());
    data.insert("bake".to_string(), functions::lisp_bake());

    let mut env = Environment { data, outer: None };

    for oi in OPERATIONS {
        functions::embed_operation(oi, &mut env);
    }
    
    for fxn in functions_nonnative::FUNCTIONS_NONNATIVE {
        parse_eval(fxn.to_string(), &mut env)
            .expect("non-native function failed to evaluate!");
    }
    

    env
}
