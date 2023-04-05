//! Most of the parsing/tokenizing code for the lisp
//!
//! Most of this code was taken from this amazing
//! tutorial: https://stopa.io/post/222
//!

use crate::lisp::{Error, Expression};
use chumsky::prelude::*;
use chumsky::error::Cheap;

pub struct Reader {
    parser: Box<dyn Parser<char, Expression, Error = Cheap<char>>>,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            parser: Box::new(parser()),
        }
    }

    pub fn parse(&self, s: &String) -> Result<Expression, Error> {
        self.parser.parse(s.as_str().trim())
            .map_err(convert_cheaps_to_err)
    }
}

fn convert_cheaps_to_err<I, S: Clone>(cheaps: Vec<Cheap<I, S>>) -> Error {
    Error(cheaps.iter()
        .map(|cheap| cheap.label())
        .fold("".to_string(), |mut a, n| {
            match n {
                Some(s) => { a.push_str(s); a},
                _ => "".to_string(),
            }
        })
    )
}

fn parser() -> impl Parser<char, Expression, Error = Cheap<char>> {
    let ident = filter(is_ident_fchar)
        .repeated().at_least(1)
        .chain::<char, Vec<_>, _>(filter(is_ident_rchar).repeated())
        .padded()
        .collect::<String>()
        .map(Expression::Symbol);

    let num = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .from_str()
        .unwrapped()
        .map(Expression::Number);

    let string = filter(|c: &char| *c != '"')
        .repeated()
        .delimited_by(just('"'), just('"'))
        .collect::<String>()
        .map(Expression::String);

    //let single_quote = filter(|c: &char| *c == '\'');
    let atom = ident.or(num).or(string);
    let qatom = just('\'')
        .ignore_then(atom)
        .map(|e| Expression::List(vec![Expression::Symbol("quote".to_string()), e]));

    let qlist = recursive(|qlist| qlist
        .padded()
        .repeated()
        .map(Expression::List)
        .map(|e| Expression::List(vec![Expression::Symbol("quote".to_string()), e]))
        .delimited_by(just("'("), just(')'))
        .or(atom)
        .or(qatom));

    let list = recursive(|list| list
        .padded()
        .repeated()
        .map(Expression::List)
        .delimited_by(just('('), just(')'))
        .or(atom)
        .or(qatom)
        .or(qlist));

    recursive(|expr| expr
        .padded()
        .repeated()
        .map(Expression::List)
        .map(|e| Expression::List(vec![Expression::Symbol("quote".to_string()), e]))
        .delimited_by(just("'("), just(')'))
        .or(atom)
        .or(qatom)
        .or(list)).then_ignore(end())
}

fn is_ident_fchar(c: &char) -> bool {
    c.is_alphabetic() || "*+!-_?<>".contains(*c)
}

fn is_ident_rchar(c: &char) -> bool {
    c.is_alphanumeric() || "*+!-_?<>".contains(*c)
}
