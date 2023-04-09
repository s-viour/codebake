//! Most of the evaluation code for the lisp including builtin forms
//!
//! Most of this code was taken from this amazing
//! tutorial: https://stopa.io/post/222
//!

use crate::lisp::{Environment, Error, Expression, Lambda};
use std::collections::HashMap;
use std::rc::Rc;

pub fn eval(expr: &Expression, env: &mut Environment) -> Result<Expression, Error> {
    match expr {
        Expression::Symbol(k) => {
            env_get(k, env).ok_or_else(|| Error(format!("unexpected symbol '{}'.", k)))
        }
        Expression::Number(_) => Ok(expr.clone()),
        Expression::Bool(_) => Ok(expr.clone()),
        Expression::String(_) => Ok(expr.clone()),
        Expression::List(list) => {
            let first_form = list
                .first()
                .ok_or_else(|| Error("expected a non-empty list.".to_string()))?;

            let arg_forms = &list[1..];
            match eval_builtin_form(first_form, arg_forms, env) {
                Some(res) => res,
                None => {
                    let first_eval = eval(first_form, env)?;
                    match first_eval {
                        Expression::Func(f) => f(&eval_forms(arg_forms, env)?),
                        Expression::Lambda(f) => {
                            let new_env = &mut env_for_lambda(f.params, arg_forms, env)?;
                            eval(&f.body, new_env)
                        }
                        other => Err(Error(format!(
                            "expected first expression to be a function. got '{}'.",
                            other
                        ))),
                    }
                }
            }
        }
        Expression::Func(_) => Err(Error("cannot eval function.".to_string())),
        Expression::Lambda(_) => Err(Error("cannot eval lambda function.".to_string())),
        Expression::Dish(_) => Ok(expr.clone()),
    }
}

fn env_get(k: &str, env: &Environment) -> Option<Expression> {
    match env.data.get(k) {
        Some(expr) => Some(expr.clone()),
        None => match &env.outer {
            Some(outer_env) => env_get(k, outer_env),
            None => None,
        },
    }
}

fn eval_forms(arg_forms: &[Expression], env: &mut Environment) -> Result<Vec<Expression>, Error> {
    arg_forms.iter().map(|x| eval(x, env)).collect()
}

fn env_for_lambda<'a>(
    params: Rc<Expression>,
    arg_forms: &[Expression],
    outer_env: &'a mut Environment,
) -> Result<Environment<'a>, Error> {
    let ks = parse_list_of_symbol_strings(params)?;
    if ks.len() != arg_forms.len() {
        return Err(Error(format!(
            "expected {} arguments. got {}.",
            ks.len(),
            arg_forms.len()
        )));
    }
    let vs = eval_forms(arg_forms, outer_env)?;
    let mut data: HashMap<String, Expression> = HashMap::new();
    for (k, v) in ks.iter().zip(vs.iter()) {
        data.insert(k.clone(), v.clone());
    }
    Ok(Environment {
        data,
        outer: Some(outer_env),
    })
}

fn parse_list_of_symbol_strings(form: Rc<Expression>) -> Result<Vec<String>, Error> {
    let list = match form.as_ref() {
        Expression::List(s) => Ok(s.clone()),
        _ => Err(Error(format!(
            "expected argument to be a list. got '{}'.",
            form.as_ref()
        ))),
    }?;
    list.iter()
        .map(|x| match x {
            Expression::Symbol(s) => Ok(s.clone()),
            _ => Err(Error(format!("expected symbol. got '{}'.", x))),
        })
        .collect()
}

pub fn eval_builtin_form(
    expr: &Expression,
    arg_forms: &[Expression],
    env: &mut Environment,
) -> Option<Result<Expression, Error>> {
    match expr {
        Expression::Symbol(s) => match s.as_ref() {
            "if" => Some(eval_if_args(arg_forms, env)),
            "def" => Some(eval_def_args(arg_forms, env)),
            "fn" => Some(eval_lambda_args(arg_forms)),
            "defn" => Some(eval_defn_args(arg_forms, env)),
            "quote" => Some(eval_quote_args(arg_forms)),
            _ => None,
        },
        _ => None,
    }
}

pub fn eval_if_args(exprs: &[Expression], env: &mut Environment) -> Result<Expression, Error> {
    let test_form = exprs
        .first()
        .ok_or_else(|| Error("expected test expression. got nothing.".to_string()))?;
    let test_eval = eval(test_form, env)?;
    match test_eval {
        Expression::Bool(b) => {
            let form_idx = if b { 1 } else { 2 };
            let res_form = exprs
                .get(form_idx)
                .ok_or_else(|| Error(format!("expected branch. got '{}'.", form_idx)))?;
            eval(res_form, env)
        }
        _ => Err(Error(format!(
            "expected boolean expression. got '{}'.",
            test_form
        ))),
    }
}

pub fn eval_def_args(exprs: &[Expression], env: &mut Environment) -> Result<Expression, Error> {
    let first_form = exprs
        .first()
        .ok_or_else(|| Error("expected symbol name. got nothing.".to_string()))?;
    let first_str = match first_form {
        Expression::Symbol(s) => Ok(s.clone()),
        other => Err(Error(format!("expected symbol. got '{}'.", other))),
    }?;
    let second_form = exprs
        .get(1)
        .ok_or_else(|| Error("expected expression. got nothing.".to_string()))?;
    if exprs.len() > 2 {
        return Err(Error(
            "define expression must only have a symbol and an expression.".to_string(),
        ));
    }
    let second_eval = eval(second_form, env)?;
    env.data.insert(first_str, second_eval);

    Ok(first_form.clone())
}

pub fn eval_lambda_args(arg_forms: &[Expression]) -> Result<Expression, Error> {
    let params_expr = arg_forms
        .first()
        .ok_or_else(|| Error("expected parameters. got nothing.".to_string()))?;
    let body_expr = arg_forms
        .get(1)
        .ok_or_else(|| Error("expected function body. got nothing.".to_string()))?;
    if arg_forms.len() > 2 {
        return Err(Error(
            "function definition must only have an argument list and a body.".to_string(),
        ))?;
    }
    Ok(Expression::Lambda(Lambda {
        body: Rc::new(body_expr.clone()),
        params: Rc::new(params_expr.clone()),
    }))
}

pub fn eval_defn_args(exprs: &[Expression], env: &mut Environment) -> Result<Expression, Error> {
    let first_form = exprs
        .first()
        .ok_or_else(|| Error("expected symbol name. got nothing.".to_string()))?;
    let name = match first_form {
        Expression::Symbol(s) => Ok(s.clone()),
        other => Err(Error(format!("expected symbol. got '{}'.", other))),
    }?;
    let params_expr = exprs
        .get(1)
        .ok_or_else(|| Error("expected argument list".to_string()))?;
    let body_expr = exprs
        .get(2)
        .ok_or_else(|| Error("expected function body".to_string()))?;

    env.data.insert(
        name,
        Expression::Lambda(Lambda {
            body: Rc::new(body_expr.clone()),
            params: Rc::new(params_expr.clone()),
        }),
    );

    Ok(first_form.clone())
}

fn eval_quote_args(exprs: &[Expression]) -> Result<Expression, Error> {
    if exprs.len() != 1 {
        return Err(Error(format!(
            "expected exactly 1 argument. got {}.",
            exprs.len()
        )));
    }

    Ok(exprs[0].clone())
}
