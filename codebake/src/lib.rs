//! codebake is a toy data-processing framework and lisp
//! inspired by [Cyberchef](https://gchq.github.io/CyberChef)
//!
//! This file contains the top-level structures for working with
//! codebake `Dish`es
//!

extern crate base64;
extern crate lazy_static;
extern crate regex;
extern crate urlencoding;

pub mod lisp;
pub mod ops;

use std::collections::HashMap;
use std::convert::Into;
use std::fs::File;
use std::io::Read;
use std::iter::Iterator;
use std::str::Chars;
use std::slice::IterMut;
use std::fmt;
use std::result;

/// Constant for an empty OperationArguments (i.e the inner field is None)
///
pub static EMPTY_ARGS: OperationArguments = OperationArguments { inner: None };

/// Maximum file size to load into memory. Any amount larger and the file will be buffered
///
pub static MAX_FILE_MEM: u64 = 256 * 1_000_000;

/// An error that occurred while performing an operation
/// on some DishData. This is the `E` type in `codebake::Result`.
///
#[derive(Clone, Debug)]
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
#[derive(Clone, PartialEq, Eq, Debug)]
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
#[derive(Clone, Debug)]
pub enum Dish {
    Success(DishData),
    Failure(DishError),
}

pub struct NewDishData {
    str_data: Option<String>,
    bin_data: Option<Vec<u8>>,
    file: Option<File>,
}

enum NewDishDataBinIteratorKind {
    Bin,
    File,
}

pub struct NewDishDataBinIterator<'a> {
    data: Option<&'a mut NewDishData>,
    bin_iter: Option<IterMut<'a, u8>>,
    kind: NewDishDataBinIteratorKind,
}

/// Represents an argument to an Operation declaratively
///
#[derive(Debug)]
pub enum OperationArgType {
    Integer,
    String,
}

/// Actually holds an argument value for an Operation
///
#[derive(Clone, Debug)]
pub enum OperationArg {
    Integer(i64),
    String(String),
}

/// Function pointer to an operation
///
type Operation = fn(&OperationArguments, &mut DishData) -> DishResult;

/// Entirely statically declared struct that holds all the information
/// about an Operation required for embedding it in the lisp
///
/// Fields:
///   * name        - name of the operation; must be named `lowercase-with-dashes`
///   * description - short description of what the operation does to the dish
///   * authors     - list of the authors who have contributed to the operation; feel free to
///                   add yourself if you've worked on this operation, even if only a small change!
///   * category    - category the operation belongs to; valid categories are:
///                   `Textual`, `Data Format`
///   * arguments   - list of 2-tuples where the first element is the name of the argument
///                 and the second argument is the type of the argument
///   * op          - function pointer to the operation itself
///
#[derive(Clone)]
pub struct OperationInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub authors: &'static [&'static str],
    pub category: &'static str,
    pub arguments: &'static [(&'static str, OperationArgType)],
    pub op: Operation,
}

/// Storage container for arguments to operations, guaranteed to be valid
/// (i.e containing all required arguments) when passed as an argument to an Operation
///
/// Essentially acts as an Option<&HashMap<String, OperationArg>>
///
pub struct OperationArguments {
    inner: Option<HashMap<String, OperationArg>>,
}

/// The Result type of codebake
///
pub type DishResult = result::Result<(), DishError>;

impl PartialEq for OperationInfo {
    fn eq(&self, other: &OperationInfo) -> bool {
        self.name == other.name
    }

    fn ne(&self, other: &OperationInfo) -> bool {
        self.name != other.name
    }
}

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
    pub fn apply(&mut self, op: Operation, args: &OperationArguments) -> &mut Dish {
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

impl NewDishData {
    pub fn from_str(s: String) -> NewDishData {
        let str_data = Some(s);
        let bin_data = None;
        let file = None;
        NewDishData {
            str_data,
            bin_data,
            file,
        }
    }

    pub fn from_bin(b: Vec<u8>) -> NewDishData {
        let str_data = None;
        let bin_data = Some(b);
        let file = None;
        NewDishData {
            str_data,
            bin_data,
            file,
        }
    }

    pub fn from_file(mut f: File) -> NewDishData {
        let file_len = f.metadata()
            .expect("failed to retrieve file metadata")
            .len();

        let str_data = None;
        let mut data: Vec<u8> = Vec::new();
        if file_len < MAX_FILE_MEM {
            f.read_to_end(&mut data)
                .expect("failed to read file");

            NewDishData {
                str_data,
                bin_data: Some(data),
                file: None,
            }
        } else {
            NewDishData {
                str_data,
                bin_data: None,
                file: Some(f),
            }
        }
    }

    pub fn iter_bin(&mut self) -> NewDishDataBinIterator {
        if self.bin_data.is_some() {
            NewDishDataBinIterator {
                data: None,
                bin_iter: Some(self.bin_data.as_mut().unwrap().iter_mut()),
                kind: NewDishDataBinIteratorKind::Bin,
            }
        } else {
            NewDishDataBinIterator {
                data: Some(self),
                bin_iter: None,
                kind: NewDishDataBinIteratorKind::File,
            }
        }
    }
}

impl<'a> Iterator for NewDishDataBinIterator<'a> {
    type Item = &'a mut u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.kind {
            NewDishDataBinIteratorKind::Bin => self.bin_iter.as_mut().unwrap().next(),
            // here is where the magic needs to happen
            // basically, we need to perform file buffering here and be able to transform the data within the buffer
            // and commit those changes to the file when overwriting the buffer
            NewDishDataBinIteratorKind::File => todo!(),
        }
    }
}

impl OperationArguments {
    pub fn new() -> OperationArguments {
        OperationArguments {
            inner: Some(HashMap::new()),
        }
    }

    /// Polymorphic function to insert a value into the OperationArguments
    pub fn insert<T: Into<OperationArg>>(&mut self, name: &str, data: T) {
        let arg = data.into();
        if let Some(h) = &mut self.inner {
            h.insert(name.to_string(), arg);
        }
    }

    /// Get an integer out of the OperationArguments by-name
    ///
    pub fn get_integer(&self, name: &str) -> Result<i64, DishError> {
        match &self.inner {
            None => return Err(DishError("empty arguments".to_string())),
            Some(h) => match h.get(name) {
                None => Err(DishError("no such argument".to_string())),
                Some(arg) => {
                    if let OperationArg::Integer(i) = arg {
                        Ok(*i)
                    } else {
                        Err(DishError("wrong argument type".to_string()))
                    }
                }
            },
        }
    }

    /// Get a string out of the OperationArguments by name
    ///
    pub fn get_string(&self, name: &str) -> Result<String, DishError> {
        match &self.inner {
            None => return Err(DishError("empty arguments".to_string())),
            Some(h) => match h.get(name) {
                None => Err(DishError("no such argument".to_string())),
                Some(arg) => {
                    if let OperationArg::String(s) = arg {
                        Ok(s.clone())
                    } else {
                        Err(DishError("wrong argument type".to_string()))
                    }
                }
            },
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
            DishData::Str(s) => {
                let mut truncated = s.clone();
                let new_char = truncated.char_indices().nth(80);
                if new_char.is_some() {
                    truncated.truncate(new_char.unwrap().0);
                    truncated.push_str("...");
                }
                write!(f, "\"{}\"", truncated)
            }
            DishData::Bin(b) => {
                let mut truncated = String::from_utf8_lossy(b).into_owned();
                let new_char = truncated.char_indices().nth(32);
                if new_char.is_some() {
                    truncated.truncate(new_char.unwrap().0);
                    truncated.push_str("...");
                }
                write!(f, "[{}]", truncated)
            }
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
            OperationArg::String(_) => "string",
        };
        write!(f, "{}", s)
    }
}

impl Into<OperationArg> for i64 {
    fn into(self) -> OperationArg {
        OperationArg::Integer(self)
    }
}

impl Into<OperationArg> for String {
    fn into(self) -> OperationArg {
        OperationArg::String(self)
    }
}
