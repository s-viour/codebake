/// The embedding of operations in the lisp as well as helper
/// functions for doing that are implemented here.
/// 
/// The rust functions defined here actually return an `Expression`
/// of variant `Expression::Func`. An `Expression::Func` contains an `Rc`
/// that points to the closure to call from the lisp. 
/// 

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::{Dish, OperationInfo, OperationArgType, OperationArg};
use crate::lisp::{Expression, Error, LispResult, Environment};


pub fn embed_operation(oi: &'static OperationInfo, env: &mut Environment) {
    let fxn = Expression::Func(Rc::new(move |args: &[Expression]| -> LispResult {
        let hargs = parse_args(oi, args)?;
        Ok(Expression::Func(Rc::new(move |args: &[Expression]| -> LispResult {
            if let Expression::Dish(dish) = &args[0] {
                dish.borrow_mut().apply(oi.op, Some(&hargs));
                Ok(Expression::Dish(dish.clone()))
            } else {
                Err(Error("must be dish".to_string()))
            }
        })))
    }));
    env.data.insert(oi.name.to_string(), fxn);
}

fn parse_arg(
    typ: &OperationArgType,
    expr: &Expression
) -> Result<OperationArg, Error> {
    match typ {
        OperationArgType::Integer => if let Expression::Number(n) = expr {
            Ok(OperationArg::Integer(*n as i64))
        } else { 
            Err(Error("expected integer".to_string()))
        },
    }
}

fn parse_args(
    oi: &OperationInfo,
    exprs: &[Expression]
) -> Result<HashMap<String, OperationArg>, Error> {
    if oi.arguments.len() != exprs.len() {
        return Err(Error("incorrect number of arguments".to_string()));
    }
    let mut ret: HashMap<String, OperationArg> = HashMap::new();

    for ((name, typ), expr) in oi.arguments.iter().zip(exprs) {
        ret.insert(name.to_string(), parse_arg(typ, expr)?);
    }

    Ok(ret)
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

pub fn lisp_dish() -> Expression {
    Expression::Func(Rc::new(|args: &[Expression]| -> LispResult {
        if args.len() != 1 {
            return Err(Error("`dish` takes a single argument".to_string()));
        }
        match &args[0] {
            Expression::String(s) => Ok(Expression::Dish(Rc::new(RefCell::new(Dish::from_string(s.clone()))))),
            _ => Err(Error("unsupported expression type for Dish (must be string)".to_string())),
        }
    }))
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
