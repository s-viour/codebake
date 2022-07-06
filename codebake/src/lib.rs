//! codebake is a toy data-processing framework and lisp
//! inspired by [Cyberchef](https://gchq.github.io/CyberChef)
//!
//! This file contains the top-level structures for working with
//! codebake `Dish`es
//!

extern crate base64;
extern crate lazy_static;
extern crate regex;

pub mod lisp;
pub mod ops;

use std::collections::HashMap;
use std::fmt;
use std::result;

/// An error that occurred while performing an operation
/// on some DishData. This is the `E` type in `codebake::Result`.
///
#[derive(Clone)]
pub struct DishError(String);

/// DishData represents both the type of data and
/// the data contained within it. The types are not very rich
/// and are just indicators of how the data should be handled.
/// This allows for operation fxns to handle textual and binary
/// data separately.
///
/// Str represents textual (unicode or ascii) data
/// Bin represents generic binary data
///
#[derive(Clone, PartialEq, Eq)]
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

/// Represents an argument to an Operation declaratively
///
#[derive(Debug)]
pub enum OperationArgType {
    Integer,
}

/// Actually holds an argument value for an Operation
///
#[derive(Clone, Debug)]
pub enum OperationArg {
    Integer(i64),
}

/// Function pointer to an operation
///
type Operation = fn(Option<&HashMap<String, OperationArg>>, &mut DishData) -> DishResult;

/// Entirely statically declared struct that holds all the information
/// about an Operation required for embedding it in the lisp
///
#[derive(Clone)]
pub struct OperationInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub arguments: &'static [(&'static str, OperationArgType)],
    pub op: Operation,
}

impl PartialEq for OperationInfo {
    fn eq(&self, other: &OperationInfo) -> bool {
        self.name == other.name
    }

    fn ne(&self, other: &OperationInfo) -> bool {
        self.name != other.name
    }
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
    pub fn apply(
        &mut self,
        op: Operation,
        args: Option<&HashMap<String, OperationArg>>,
    ) -> &mut Dish {
        if let Dish::Success(data) = self {
            let op = op;
            let v = op(args, data);
            if let Err(e) = v {
                *self = Dish::Failure(e);
            }
        }
        self
    }
}

impl DishData {
    /// Helper method that converts any DishData to bytes
    fn as_bytes(&self) -> &[u8] {
        match self {
            DishData::Str(s) => s.as_bytes(),
            DishData::Bin(b) => b,
        }
    }
}

impl OperationArg {
    fn integer(&self) -> Result<i64, DishError> {
        // remove this when we add more argument types :p
        #[allow(irrefutable_let_patterns)]
        if let OperationArg::Integer(i) = self {
            Ok(*i)
        } else {
            Err(DishError(format!("expected integer, got {}", self)))
        }
    }
}

impl fmt::Display for Dish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Dish::Success(data) => write!(f, "Dish({})", data),
            Dish::Failure(e) => write!(f, "error: {}", e),
        }
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
        write!(f, "dish error: {}", self.0)
    }
}

impl fmt::Display for OperationArg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            OperationArg::Integer(_) => "integer",
        };
        write!(f, "{}", s)
    }
}
