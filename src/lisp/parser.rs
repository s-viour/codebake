/// Most of the parsing/tokenizing/evaluation code for the lisp
/// 
/// Most of this code was taken from this amazing
/// tutorial: https://stopa.io/post/222
/// 

use std::num::ParseFloatError;
use lazy_static::lazy_static;
use regex::Regex;
use crate::lisp::{Expression, Error, Environment};


pub fn parse_eval(expr: String, env: &mut Environment) -> Result<Expression, Error> {
    let (parsed, _) = parse(&tokenize(expr))?;
    let evald = eval(&parsed, env)?;
    Ok(evald)
}

pub fn tokenize(expr: String) -> Vec<String> {
    lazy_static! {
        // i used cyberchef to build & test this regex
        // kinda funny since we're building a cyberchef clone
        // *i used the stones to destroy the stones*
        static ref RE: Regex = Regex::new("((\"(.*?)\")|\\w+|\\(|\\))").unwrap();
    }
    
    let spread = expr
        .replace("(", " ( ")
        .replace(")", " ) ");
    
    // we use a regex here so we can keep strings with spaces in them
    // as one token. so "blah blah blah" gets tokenized as ["blah blah blah"]
    // and not ["blah, blah, blah"]
    RE.find_iter(spread.as_str())
        .map(|x| x.as_str().to_string())
        .collect()
}

pub fn read_seq<'a>(tokens: &'a [String]) -> Result<(Expression, &'a [String]), Error> {
    let mut res: Vec<Expression> = Vec::new();
    let mut xs = tokens;

    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or(Error("could not find closing ')'".to_string()))?;

        if next_token == ")" {
            return Ok((Expression::List(res), rest));
        }
        let (exp, new_xs) = parse(&xs)?;
        res.push(exp);
        xs = new_xs;
    }
}

pub fn parse_atom(token: &str) -> Expression {
    if token == "true" {
        return Expression::Bool(true);
    } else if token == "false" {
        return Expression::Bool(false);
    }

    let mut chrs = token.chars();
    if chrs.nth(0).unwrap() == '\"' && chrs.nth_back(0).unwrap() == '\"' {
        return Expression::String(chrs.collect());
    }

    let potential_float: Result<f64, ParseFloatError> = token.parse();
    match potential_float {
        Ok(f) => Expression::Number(f),
        // the tutorial performs a `.clone()` here, dunno why.
        // don't think you need it tho
        Err(_) => Expression::Symbol(token.to_string()),
    }
}

pub fn parse<'a>(tokens: &'a [String]) -> Result<(Expression, &'a [String]), Error> {
    let (token, rest) = tokens.split_first()
        .ok_or(Error("could not get token".to_string()))?;
    
    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(Error("unexpected ')'".to_string())),
        _ => Ok((parse_atom(token), rest))
    }
}

pub fn eval(expr: &Expression, env: &mut Environment) -> Result<Expression, Error> {
    match expr {
        Expression::Symbol(k) => {
            env.data.get(k)
                .ok_or(Error(format!("unexpected symbol k: '{}'", k)))
                .map(|x| x.clone())
        },
        Expression::Number(_) => Ok(expr.clone()),
        Expression::Bool(_) => Ok(expr.clone()),
        Expression::String(_) => Ok(expr.clone()),
        Expression::List(list) => {
            let first_form = list
                .first()
                .ok_or(Error("expected non-empty list.to_string()".to_string()))?;
            
            let arg_forms = &list[1..];
            match eval_builtin_form(first_form, arg_forms, env) {
                Some(res) => res,
                None => {
                    let first_eval = eval(first_form, env)?;
                    match first_eval {
                        Expression::Func(f) => {
                            let args_eval = arg_forms
                                .iter()
                                .map(|x| eval(x, env))
                                .collect::<Result<Vec<Expression>, Error>>();
                            f(&args_eval?)
                        },
                        _ => Err(Error("first form must be a function".to_string())),
                    }
                }
            }
        },
        Expression::Func(_) => Err(Error("unexpected form".to_string())),
        Expression::Dish(_) => Ok(expr.clone()),
    }
}

pub fn eval_builtin_form(
    expr: &Expression, arg_forms: &[Expression], env: &mut Environment
) -> Option<Result<Expression, Error>> {
    match expr {
        Expression::Symbol(s) => match s.as_ref() {
            "if" => Some(eval_if_args(arg_forms, env)),
            "def" => Some(eval_def_args(arg_forms, env)),
            _ => None,
        }
        _ => None,
    }
}

pub fn eval_if_args(exprs: &[Expression], env: &mut Environment) -> Result<Expression, Error> {
    let test_form = exprs.first().ok_or(
        Error("expected expression after if".to_string())
    )?;
    let test_eval = eval(test_form, env)?;
    match test_eval {
        Expression::Bool(b) => {
            let form_idx = if b { 1 } else { 2 };
            let res_form = exprs.get(form_idx)
                .ok_or(Error(format!("expected branch: {}", form_idx)))?;
            eval(res_form, env)
        },
        _ => Err(Error(format!("unexpected test `{}`", test_form)))
    }
}

pub fn eval_def_args(exprs: &[Expression], env: &mut Environment) -> Result<Expression, Error> {
    let first_form = exprs.first()
        .ok_or(Error("expected symbol name".to_string()))?;
    let first_str = match first_form {
        Expression::Symbol(s) => Ok(s.clone()),
        other => Err(Error(format!("expected symbol, not {}", other)))
    }?;
    let second_form = exprs.get(1)
        .ok_or(Error("expected expression".to_string()))?;
    if exprs.len() > 2 {
        return Err(Error("`def` expression can only have 2 arguments".to_string()));
    }
    let second_eval = eval(second_form, env)?;
    env.data.insert(first_str, second_eval);
    
    Ok(first_form.clone())
}
