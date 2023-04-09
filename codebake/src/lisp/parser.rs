//! Most of the parsing/tokenizing code for the lisp
//!
//! Most of this code was taken from this amazing
//! tutorial: https://stopa.io/post/222
//!

use crate::lisp::{Error, Expression};
use chumsky::error::SimpleReason;
use chumsky::prelude::*;
use std::hash::Hash;
use std::rc::Rc;
use std::cell::RefCell;
use crate::Dish;

pub struct Reader {
    parser: Box<dyn Parser<char, Expression, Error = Simple<char>>>,
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
        self.parser
            .parse(s.as_str().trim())
            .map_err(convert_cheaps_to_err)
    }
}

/// Converts a vector of `Cheap`s into a `lisp::Error`. This is utilized by `Reader::parse`
///
fn convert_cheaps_to_err<I: Eq + Hash, S: Clone>(cheaps: Vec<Simple<I, S>>) -> Error {
    Error(
        cheaps
            .iter()
            .map(|cheap| cheap.reason())
            .map(|e| match e {
                SimpleReason::Unexpected => "unexpected input".to_string(),
                SimpleReason::Unclosed { .. } => "unclosed parenthesis".to_string(),
                SimpleReason::Custom(s) => s.to_string(),
            })
            .fold("".to_string(), |mut a, n| {
                a.push_str(&n);
                a
            }),
    )
}

/// This implements the lisp parser!
///
/// This function could be improved ***significantly*** because I don't really understand chumsky
/// all that well and I wasn't sure how to get embedded quoting working correctly. That's why there's
/// two `list` declarations and basically two `qlist` declarations.
///
fn parser() -> impl Parser<char, Expression, Error = Simple<char>> {
    // parses a single symbol
    let symbol = filter(is_symbol_fchar)
        .repeated()
        .at_least(1)
        .chain::<char, Vec<_>, _>(filter(is_symbol_rchar).repeated())
        .padded()
        .collect::<String>()
        .map(Expression::Symbol);

    // parses a single number
    let pos_number = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .from_str()
        .unwrapped()
        .map(Expression::Number);

    let number = filter(|c: &char| *c == '-')
        .repeated()
        .at_least(1)
        .ignore_then(pos_number)
        .map(|e| match e {
            Expression::Number(n) => Expression::Number(-n),
            _ => e,
        })
        .or(pos_number);

    // parses a single string
    let string = filter(|c: &char| *c != '"')
        .repeated()
        .delimited_by(just('"'), just('"'))
        .collect::<String>()
        .map(Expression::String);

    let byte = text::int::<_, Simple<char>>(10)
        .padded()
        .try_map(|s, span| s
            .parse::<u8>()
            .map_err(|e| Simple::custom(span, format!("{}", e))));

    let vector = byte
        .repeated()
        .delimited_by(just('['), just(']'))
        .map(|v| v.iter().map(|n| Expression::Number(*n as f64)).collect())
        .map(Expression::List);

    let dish_literal_str = just('d')
        .ignore_then(string)
        .map(|e| {
            if let Expression::String(s) = e {
                let dish = Rc::new(RefCell::new(Dish::from_string(s)));
                Expression::Dish(dish)
            } else {
                panic!("invalid expression passed to dish literal");
            }
        });

    let dish_literal_vec = just('d')
        .ignore_then(vector)
        .map(|e| {
            if let Expression::List(ns) = e {
                let data = ns.iter().map(|e| {
                    if let Expression::Number(n) = e {
                        *n as u8
                    } else {
                        panic!("invalid expression passed to dish literal");
                    }
                }).collect::<Vec<u8>>();
                let dish = Rc::new(RefCell::new(Dish::from_bytes(data)));
                Expression::Dish(dish)
            } else {
                panic!("invalid expression passed to dish literal");
            }
        });

    // parses a single atom
    let atom = dish_literal_str.or(dish_literal_vec).or(vector).or(number).or(symbol).or(string);
    // parses a quoted atom
    let qatom = just('\'')
        .ignore_then(atom)
        .map(|e| Expression::List(vec![Expression::Symbol("quote".to_string()), e]));

    // parses a single list of only atoms
    let list1 = recursive(|list| {
        list.padded()
            .repeated()
            .map(Expression::List)
            .delimited_by(just('('), just(')'))
            .or(atom)
            .or(qatom)
    });

    // parses a quoted list
    let qlist = recursive(|qlist| {
        qlist
            .padded()
            .repeated()
            .map(Expression::List)
            .map(|e| Expression::List(vec![Expression::Symbol("quote".to_string()), e]))
            .delimited_by(just("'("), just(')'))
            .or(atom)
            .or(qatom)
            .or(list1)
    });

    // parses a regular list
    let list2 = recursive(|list| {
        list.padded()
            .repeated()
            .map(Expression::List)
            .delimited_by(just('('), just(')'))
            .or(atom)
            .or(qatom)
            .or(qlist)
    });

    // this is basically a superposition of qlist and list
    // this begins parsing from the top and supports quoting things at the top-level
    recursive(|expr| {
        expr.padded()
            .repeated()
            .map(Expression::List)
            .map(|e| Expression::List(vec![Expression::Symbol("quote".to_string()), e]))
            .delimited_by(just("'("), just(')'))
            .or(atom)
            .or(qatom)
            .or(list2)
    })
    .then_ignore(end())
}

/// predicate of whether or not a character can be the first character of a symbol name
fn is_symbol_fchar(c: &char) -> bool {
    c.is_alphabetic() || "*=+!-_?<>:".contains(*c)
}

/// predicate of whether or not a character can be anywhere else in a symbol name
fn is_symbol_rchar(c: &char) -> bool {
    c.is_alphanumeric() || "=*+!-_?<>".contains(*c)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::Dish;
    use crate::lisp::{Expression, Reader};

    #[test]
    fn test_reader_string() {
        let reader = Reader::new();
        let expr1 = "\"string\"".to_string();
        let _exp1 = Expression::String("string".to_string());
        let expr2 = "\"this is a\tlong string\nmany spaces\"".to_string();
        let _exp2 = Expression::String("this is a\tlong string\nmany spaces".to_string());

        assert!(matches!(reader.parse(&expr1), Ok(_exp1)));
        assert!(matches!(reader.parse(&expr2), Ok(_exp2)));
    }

    #[test]
    fn test_reader_number() {
        let reader = Reader::new();
        let expr1 = "-12".to_string();
        let _exp1 = Expression::Number(-12.0);
        let expr2 = "-3.14159".to_string();
        let _exp2 = Expression::Number(-3.14159);
        let expr3 = "300.14159".to_string();
        let _exp3 = Expression::Number(300.14159);

        assert!(matches!(reader.parse(&expr1), Ok(_exp1)));
        assert!(matches!(reader.parse(&expr2), Ok(_exp2)));
        assert!(matches!(reader.parse(&expr3), Ok(_exp3)));
    }

    #[test]
    fn test_reader_list() {
        let reader = Reader::new();
        let expr1 = "(+ 2 3)".to_string();
        let _exp1 = Expression::Number(5.0);
        let expr2 = "(def a (- 112.4 12.2))".to_string();
        let _exp2 = Expression::Symbol("a".to_string());

        assert!(matches!(reader.parse(&expr1), Ok(_exp1)));
        assert!(matches!(reader.parse(&expr2), Ok(_exp2)));
    }

    #[test]
    fn test_reader_quote() {
        let reader = Reader::new();
        let expr1 = "'(1 2 3)".to_string();
        let _exp1 = Expression::List(vec![
            Expression::Symbol("quote".to_string()),
            Expression::List(vec![
                Expression::Number(1.0),
                Expression::Number(2.0),
                Expression::Number(3.0),
            ]),
        ]);
        let expr2 = "(apply + '(3 4 5))".to_string();
        let _exp2 = Expression::Number(12.0);

        assert!(matches!(reader.parse(&expr1), Ok(_exp1)));
        assert!(matches!(reader.parse(&expr2), Ok(_exp2)));
    }

    #[test]
    fn test_reader_dish_literal() {
        let reader = Reader::new();
        let expr1 = "d\"hello world\"".to_string();
        let _exp1 = Expression::Dish(Rc::new(RefCell::new(Dish::from_string("hello world".to_string()))));
        let expr2 = "d[24 25 26]".to_string();
        let _exp2 = Expression::Dish(Rc::new(RefCell::new(Dish::from_bytes(vec![24, 25, 26]))));

        assert!(matches!(reader.parse(&expr1), Ok(_exp1)));
        assert!(matches!(reader.parse(&expr2), Ok(_exp2)));
    }
}
