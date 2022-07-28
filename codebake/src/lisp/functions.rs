//! The embedding of operations in the lisp as well as helper
//! functions for doing that are implemented here.
//!
//! The rust functions defined here actually return an `Expression`
//! of variant `Expression::Func`. An `Expression::Func` contains an `Rc`
//! that points to the closure to call from the lisp.
//!

use crate::lisp::{Environment, Error, Expression, LispResult};
use crate::{EMPTY_ARGS, Dish, OperationArguments, OperationArg, OperationArgType, OperationInfo};
use std::cell::RefCell;
use std::rc::Rc;

pub fn embed_operation(oi: &'static OperationInfo, env: &mut Environment) {
    // if the operation has no arguments, don't add the argument parsing
    // wrapper closure. just embed it raw
    if oi.arguments.len() == 0 {
        env.data.insert(oi.name.to_string(), Expression::Func(Rc::new(
            move |args: &[Expression]| -> LispResult {
                if args.len() != 1 {
                    return Err(Error("expected an argument".to_string()));
                }
                if let Expression::Dish(dish) = &args[0] {
                    dish.borrow_mut().apply(oi.op, &EMPTY_ARGS);
                    Ok(Expression::Dish(dish.clone()))
                } else {
                    Err(Error("must be dish".to_string()))
                }
            },
        )));
        return;
    }
    
    // otherwise, INCLUDE the wrapper closure that parses args
    env.data.insert(
        oi.name.to_string(),
        Expression::Func(Rc::new(move |args: &[Expression]| -> LispResult {
        let hargs = parse_args(oi, args)?;
        Ok(Expression::Func(Rc::new(
            move |args: &[Expression]| -> LispResult {
                if args.len() != 1 {
                    return Err(Error("expected an argument".to_string()));
                }
                if let Expression::Dish(dish) = &args[0] {
                    dish.borrow_mut().apply(oi.op, &hargs);
                    Ok(Expression::Dish(dish.clone()))
                } else {
                    Err(Error("must be dish".to_string()))
                }
            },
        )))
    })));
}

fn parse_arg(typ: &OperationArgType, expr: &Expression) -> Result<OperationArg, Error> {
    match typ {
        OperationArgType::Integer => {
            if let Expression::Number(n) = expr {
                Ok(OperationArg::Integer(*n as i64))
            } else {
                Err(Error("expected integer".to_string()))
            }
        },
        OperationArgType::String => {
            Ok(OperationArg::String(expr.to_string()))
        },
    }
}

fn parse_args(
    oi: &OperationInfo,
    exprs: &[Expression],
) -> Result<OperationArguments, Error> {
    if oi.arguments.len() != exprs.len() {
        return Err(Error("incorrect number of arguments".to_string()));
    }
    let mut ret: OperationArguments = OperationArguments::new();

    for ((name, typ), expr) in oi.arguments.iter().zip(exprs) {
        ret.insert(name, parse_arg(typ, expr)?);
    }

    Ok(ret)
}

// add function
pub fn lisp_add() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        let sum = parse_list_of_floats(args)?
            .iter()
            .fold(0.0, |sum, a| sum + a);
        Ok(Expression::Number(sum))
    }))
}

// subtract function
pub fn lisp_subtract() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        let floats = parse_list_of_floats(args)?;
        let first = *floats
            .first()
            .ok_or_else(|| Error("expected at least one number".to_string()))?;
        let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);

        Ok(Expression::Number(first - sum_of_rest))
    }))
}

pub fn lisp_apply() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        if args.len() != 2 {
            return Err(Error(format!("expected 2 arguments, got {}", args.len())));
        }
        match &args[0] {
            Expression::Func(f) => match &args[1] {
                Expression::List(l) => f(l),
                _ => Err(Error("second argument to `apply` must be a list".to_string())),
            }
            _ => Err(Error("first argument to `apply` must be a function".to_string())),
        }
    }))
}

pub fn lisp_head() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        if args.len() != 1 {
            return Err(Error("incorrect number of arguments".to_string()));
        }
        match &args[0] {
            Expression::List(v) => {
                if v.len() == 0 {
                    return Ok(Expression::Symbol("nil".to_string()));
                }
                Ok(v.get(0)
                    .map(|x| x.clone())
                    .unwrap())
            },
            _ => Err(Error("expected list".to_string())),
        }
    }))
}

pub fn lisp_last() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        if args.len() != 1 {
            return Err(Error("incorrect number of arguments".to_string()));
        }
        match &args[0] {
            Expression::List(v) => {
                v.get(v.len() - 1)
                    .ok_or_else(|| Error("empty list".to_string()))
                    .map(|x| x.clone())
            },
            _ => Err(Error("expected list".to_string())),
        }
    }))
}

pub fn lisp_rest() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        if args.len() != 1 {
            return Err(Error("incorrect number of arguments".to_string()));
        }
        match &args[0] {
            Expression::List(v) => {
                let mut iter = v.iter();
                iter.next();
                Ok(Expression::List(
                    iter.map(|x| x.clone()).collect::<Vec<Expression>>()
                ))
            },
            _ => Err(Error("expected list".to_string())),
        }
    }))
}

pub fn lisp_init() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        if args.len() != 1 {
            return Err(Error("incorrect number of arguments".to_string()));
        }
        match &args[0] {
            Expression::List(v) => {
                Ok(Expression::List(
                    v.iter().take(v.len() - 1).map(|x| x.clone()).collect::<Vec<Expression>>()
                ))
            },
            _ => Err(Error("expected list".to_string())),
        }
    }))
}

pub fn lisp_dish() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        if args.len() != 1 {
            return Err(Error("`dish` takes a single argument".to_string()));
        }
        match &args[0] {
            Expression::String(s) => Ok(Expression::Dish(Rc::new(RefCell::new(
                Dish::from_string(s.clone()),
            )))),
            _ => Err(Error(
                "unsupported expression type for Dish (must be string)".to_string(),
            )),
        }
    }))
}

pub fn lisp_recipe() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        if args.len() == 0 {
            return Err(Error("`recipe` takes 2 arguments".to_string()));
        }

        let mut funcs: Vec<Expression> = Vec::new();
        for expr in args {
            match expr {
                Expression::Func(_) => funcs.push(expr.clone()),
                _ => return Err(Error("expected funcction".to_string())),
            }
        }

        Ok(Expression::List(funcs))
    }))
}

pub fn lisp_bake() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        if args.len() != 2 {
            return Err(Error("`bake` takes 2 arguments".to_string()));
        }
        
        let recipe = match &args[0] {
            Expression::List(v) => Ok(v),
            _ => Err(Error("expected list".to_string())),
        }?;

        match &args[1] {
            Expression::Dish(_) => Ok(()),
            _ => Err(Error("expected Dish".to_string()))
        }?;

        // i cannot believe it inferred the type of the Vec here
        let mut funcs = Vec::new();
        for expr in recipe {
            match expr {
                Expression::Func(f) => {
                    funcs.push(f.clone())
                },
                _ => {
                    return Err(Error("recipe must be list of functions".to_string()))
                }
            }
        }

        for func in funcs {
            func(&[args[1].clone()])?;  
        }

        Ok(args[1].clone())
    }))
}

pub fn lisp_empty() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        if args.len() != 1 {
            return Err(Error(format!("expected 1 argument, got {}", args.len())));
        }

        let nil = Expression::Symbol("nil".to_string());

        Ok(match &args[0] {
            Expression::List(v) => Expression::Bool(v.is_empty()),
            Expression::String(s) => Expression::Bool(s.is_empty()),
            Expression::Dish(d) => {
                match &*d.borrow() {
                    Dish::Success(data) => {
                        Expression::Bool(data.as_bytes().len() == 0)
                    },
                    _ => Expression::Bool(false)
                }
            },
            _ => nil
        })
    }))
}

pub fn lisp_cons() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        if args.len() != 2 {
            return Err(Error(format!("expected 2 arguments, got {}", args.len())));
        }

        if let Expression::List(mut l) = args[1].clone() {
            l.insert(0, args[0].clone());
            Ok(Expression::List(l))
        } else {
            Err(Error("expected 2nd argument to be a list".to_string()))
        }
    }))
}

pub fn lisp_eq() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        if args.len() < 1 {
            return Err(Error(format!("expected at least 1 argument, got {}", args.len())));
        }

        let mut iter = args.iter();
        let fst = iter.next().unwrap();
        Ok(Expression::Bool(iter.all(|x| x == fst)))
    }))
}

fn parse_list_of_floats(args: &[Expression]) -> Result<Vec<f64>, Error> {
    args.iter().map(parse_single_float).collect()
}

fn parse_single_float(expr: &Expression) -> Result<f64, Error> {
    match expr {
        Expression::Number(num) => Ok(*num),
        _ => Err(Error("expected a number".to_string())),
    }
}
