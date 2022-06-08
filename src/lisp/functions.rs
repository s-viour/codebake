/// The embedding of operations in the lisp as well as helper
/// functions for doing that are implemented here.
/// 
/// The rust functions defined here actually return an `Expression`
/// of variant `Expression::Func`. An `Expression::Func` contains an `Rc`
/// that points to the closure to call from the lisp. 
/// 
/// To take an operation in src/ops.rs and embed it in the lisp:
///   1. If you learn by example, just take a look at the `rot13 operation`
///      and the `reverse operation` comments below. They should be a little
///      self-explanatory.
/// 
///   2. If your operation is parameterized, you'll have to parse
///      the arguments. Start by using the `param_operation` macro to
///      define the name of your function and give the macro a closure
///      as the second argument like so:
///        `param_operation!(lisp_rot13, |args: &[Expression]| -> LispResult {...}`
///      Inside the closure, you'll have access to the slice of `Expression`s
///      that you should parse from. Do this however you want. After you've parsed
///      all your arguments, the last expression in your closure should be:
///        `Ok(embed_operation(Rc::new(your_op(your, args, etc))))`
/// 
///   3. If your operation is not parameterized, just use the `noparam_operation`
///      macro like so:
///        `noparam_operation!(your_opname, your_op_fxn);`
///

use std::cell::RefCell;
use std::rc::Rc;
use crate::{Dish, DishData, DishResult};
use crate::lisp::{Expression, Error, LispResult};
use crate::ops;


/// Constructs the function for an unparamaterized operation
/// 
macro_rules! noparam_operation {
    ($name:ident, $fxn:expr) => {
        pub fn $name() -> Expression {
            embed_operation(Rc::new($fxn()))
        }
    }
}

/// Constructs the function for a paramaterized operation
/// by embedding argument handling code in a function that ultimately
/// calls embed_function
/// 
macro_rules! param_operation {
    ($name:ident, $closure:expr) => {
        pub fn $name() -> Expression {
            Expression::Func(Rc::new($closure))
        }
    }
}

// add function
pub fn lisp_add() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        let sum = parse_list_of_floats(args)?.iter().fold(0.0, |sum, a| sum + a);
        Ok(Expression::Number(sum))
    }))
}

// subtract function
pub fn lisp_subtract() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        let floats = parse_list_of_floats(args)?;
        let first = *floats.first()
            .ok_or(Error("expected at least one number".to_string()))?;
        let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);

        Ok(Expression::Number(first - sum_of_rest))
    }))
}

// dish function
param_operation!(lisp_dish, |args: &[Expression]| -> LispResult {
    if args.len() != 1 {
        return Err(Error("`dish` takes a single argument".to_string()));
    }
    match &args[0] {
        Expression::String(s) => Ok(Expression::Dish(Rc::new(RefCell::new(Dish::from_string(s.clone()))))),
        _ => Err(Error("unsupported expression type for Dish (must be string)".to_string())),
    }
});

// rot13 operation
param_operation!(lisp_rot13, |args: &[Expression]| -> LispResult {
    let n = match args.get(0).ok_or(Error("expected argument 0".to_string()))? {
        Expression::Number(f) => *f as i64,
        _ => return Err(Error("expected integer".to_string())),
    };

    Ok(embed_operation(Rc::new(ops::rot13(n))))
});

// reverse operation
noparam_operation!(lisp_reverse, ops::reverse);

/// Helper function for embedding raw operations inside
/// operation closures
/// 
fn embed_operation(fxn: Rc<dyn Fn(&mut DishData) -> DishResult>) -> Expression {
    Expression::Func(Rc::new(move |args: &[Expression]| -> LispResult {
        match require_arg(args, 0)? {
            Expression::Dish(dish) => {
                Dish::apply(&mut *dish.borrow_mut(), &*fxn);
                Ok(Expression::Dish(dish.clone()))
            },
            _ => Err(Error("expected dish".to_string())),
        }
    }))
}

/// Helper function for requiring a positional argument in argument parsing
/// 
fn require_arg(args: &[Expression], n: usize) -> Result<&Expression, Error> {
    Ok(args.get(n).ok_or(Error(format!("expected argument {}", n)))?)
}

fn parse_list_of_floats(args: &[Expression]) -> Result<Vec<f64>, Error> {
    args
        .iter()
        .map(|x| parse_single_float(x))
        .collect()
}

fn parse_single_float(expr: &Expression) -> Result<f64, Error> {
    match expr {
        Expression::Number(num) => Ok(*num),
        _ => Err(Error("expected a number".to_string())),
    }
}
