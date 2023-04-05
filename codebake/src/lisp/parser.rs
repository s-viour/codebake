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
    ///! Constructs a `Reader` by building the parser and storing it
    ///!
    pub fn new() -> Reader {
        Reader {
            parser: Box::new(parser()),
        }
    }

    ///! Attempts to parse a string `s` as an expression
    ///!
    pub fn parse(&self, s: &String) -> Result<Expression, Error> {
        self.parser.parse(s.as_str().trim())
            .map_err(convert_cheaps_to_err)
    }
}

/// Converts a vector of `Cheap`s into a `lisp::Error`. This is utilized by `Reader::parse`
/// 
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

/// This implements the lisp parser!
/// 
fn parser() -> impl Parser<char, Expression, Error = Cheap<char>> {
    // parses a single symbol
    let symbol = filter(is_symbol_fchar)
        .repeated().at_least(1)
        .chain::<char, Vec<_>, _>(filter(is_symbol_rchar).repeated())
        .padded()
        .collect::<String>()
        .map(Expression::Symbol);

    // parses a single number
    let number = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .from_str()
        .unwrapped()
        .map(Expression::Number);

    // parses a single string
    let string = filter(|c: &char| *c != '"')
        .repeated()
        .delimited_by(just('"'), just('"'))
        .collect::<String>()
        .map(Expression::String);

    // parses a single atom
    let atom = symbol.or(number).or(string);
    // parses a quoted atom
    let qatom = just('\'')
        .ignore_then(atom)
        .map(|e| Expression::List(vec![Expression::Symbol("quote".to_string()), e]));

    // parses a quoted list
    let qlist = recursive(|qlist| qlist
        .padded()
        .repeated()
        .map(Expression::List)
        .map(|e| Expression::List(vec![Expression::Symbol("quote".to_string()), e]))
        .delimited_by(just("'("), just(')'))
        .or(atom)
        .or(qatom));

    // parses a regular list
    let list = recursive(|list| list
        .padded()
        .repeated()
        .map(Expression::List)
        .delimited_by(just('('), just(')'))
        .or(atom)
        .or(qatom)
        .or(qlist));

    // this is basically a superposition of qlist and list
    // this begins parsing from the top and supports quoting things at the top-level
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

/// predicate of whether or not a character can be the first character of a symbol name
fn is_symbol_fchar(c: &char) -> bool {
    c.is_alphabetic() || "*+!-_?<>".contains(*c)
}

/// predicate of whether or not a character can be anywhere else in a symbol name
fn is_symbol_rchar(c: &char) -> bool {
    c.is_alphanumeric() || "*+!-_?<>".contains(*c)
}

#[cfg(test)]
mod tests {
    use crate::lisp::{Expression, Reader};

    #[test]
    fn test_reader_basic() {
        let reader = Reader::new();
        let expr1 = "(+ 2 3)".to_string();
        let _exp1 = Expression::Number(5.0);
        let expr2 = "(def a (- 112.4 12.2))".to_string();
        let _exp2 = Expression::Symbol("a".to_string());

        assert!(matches!(
            reader.parse(&expr1),
            Ok(_exp1)
        ));
        assert!(matches!(
            reader.parse(&expr2),
            Ok(_exp2)
        ));
    }

    #[test]
    fn test_reader_quote() {
        let reader = Reader::new();
        let expr1 = "'(1 2 3)".to_string();
        let _exp1 = Expression::List(vec![
            Expression::Symbol("quote".to_string()),
            Expression::List(vec![Expression::Number(1.0), Expression::Number(2.0), Expression::Number(3.0)])
        ]);
        let expr2 = "(apply + '(3 4 5))".to_string();
        let _exp2 = Expression::Number(12.0);

        assert!(matches!(
            reader.parse(&expr1),
            Ok(_exp1)
        ));
        assert!(matches!(
            reader.parse(&expr2),
            Ok(_exp2)
        ));
    }
}
