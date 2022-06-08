/// codebake is a toy data-processing framework and lisp
/// inspired by [Cyberchef](https://gchq.github.io/CyberChef)
/// 
/// This file contains the top-level structures for working with
/// codebake `Dish`es
/// 

extern crate regex;
extern crate lazy_static;

pub mod ops;
pub mod lisp;

use std::fmt;
use std::result;

/// An error that occurred while performing an operation
/// on some DishData. This is the `E` type in `codebake::Result`.
/// 
#[derive(Clone)]
pub struct DishError {
    failed_op: String,
    reason: String,
}

/// DishData represents both the type of data and
/// the data contained within it. The types are not very rich
/// and are just indicators of how the data should be handled.
/// This allows for operation fxns to handle textual and binary
/// data separately.
/// 
/// Str represents textual (unicode or ascii) data
/// Bin represents generic binary data
/// 
#[derive(Clone)]
pub enum DishData {
    Str(String),
    Bin(Vec<u8>),
}

/// A Dish is the core component of codebake, and is basically
/// just a wrapper around DishData and DishError. Haskellers may
/// think of it as an `Either DishError DishData`.
/// 
/// `Dish::apply` is the core function for operating on dishes.
///
#[derive(Clone)]
pub enum Dish {
    Success(DishData),
    Failure(DishError),
}

/// The Result type of codebake
/// 
pub type DishResult = result::Result<(), DishError>;

impl Dish {
    /// Consumes a `String` and produces a `Dish`
    pub fn from_string(data: String) -> Dish {
        Dish::Success(DishData::Str(data))
    }

    /// Consumes a `Vec` of bytes (`Vec<u8>`) and produces a `Dish`
    pub fn from_bytes(data: Vec<u8>) -> Dish {
        Dish::Success(DishData::Bin(data))
    }

    /// Takes a function of type `DishData -> DishResult` (AKA an operation)
    /// and consumes `self`, producing a new `Dish` with the 
    /// operation applied.
    pub fn apply(this: &mut Dish, op: &dyn Fn(&mut DishData) -> DishResult) {
        if let Dish::Success(data) = this {
            let v = op(data);
            if let Err(e) = v {
                *this = Dish::Failure(e);
            }
        };
    }
}

impl fmt::Display for DishData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DishData::Str(s) => write!(f, "{}", s),
            DishData::Bin(b) => write!(f, "{}", String::from_utf8_lossy(b)),
        }
    }
}

impl fmt::Display for DishError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "op: {}\nreason: {}", self.failed_op, self.reason)
    }
}
