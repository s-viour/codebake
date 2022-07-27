//! Most of the parsing/tokenizing code for the lisp
//!
//! Most of this code was taken from this amazing
//! tutorial: https://stopa.io/post/222
//!

use crate::lisp::{eval::eval, Environment, Error, Expression};
use lazy_static::lazy_static;
use regex::Regex;
use std::num::ParseFloatError;

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
        static ref RE: Regex = Regex::new("((\"(.*?)\")|[a-zA-Z0-9!@#\\?$&()=\\-`.+,/\"]+|\\(|\\))").unwrap();
        static ref RE_STR: Regex = Regex::new("(\"(.*)\")").unwrap();
    }

    // we use a regex here so we can keep strings with spaces in them
    // as one token. so "blah blah blah" gets tokenized as ["blah blah blah"]
    // and not ["blah, blah, blah"]
    let intermediate: String = RE.find_iter(expr.as_str())
        .map(|x| x.as_str().to_string())
        // hehe this is a hack ^-^
        // this code USED to apply this replacement to every token
        // now, we selectively apply it to only tokens that don't contain a double-quote character
        // this leaves strings 100% intact
        // before, "(" would have been turned into " ( "
        .map(|x| {
            if !x.contains('\"') {
                x.replace('(', " ( ").replace(')', " ) ")
            } else {
                x
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    let ret = RE.find_iter(intermediate.as_str())
        .map(|x| x.as_str().to_string())
        .collect();

    ret
}

pub fn read_seq(tokens: &[String]) -> Result<(Expression, &[String]), Error> {
    let mut res: Vec<Expression> = Vec::new();
    let mut xs = tokens;

    loop {
        let (next_token, rest) = xs
            .split_first()
            .ok_or_else(|| Error("could not find closing ')'".to_string()))?;

        if next_token == ")" {
            return Ok((Expression::List(res), rest));
        }
        let (exp, new_xs) = parse(xs)?;
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
    if chrs.next().unwrap() == '\"' && chrs.nth_back(0).unwrap() == '\"' {
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

pub fn parse(tokens: &[String]) -> Result<(Expression, &[String]), Error> {
    let (token, rest) = tokens
        .split_first()
        .ok_or_else(|| Error("could not get token".to_string()))?;

    match &token[..] {
        "(" => read_seq(rest),
        ")" => Err(Error("unexpected `)`".to_string())),
        _ => Ok((parse_atom(token), rest)),
    }
}
