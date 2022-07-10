//! The raw functions for operations on Dishes are implemented in this module
//! Each category has its own file (data_format.rs, textual.rs, etc.)
//!
//! An operation is a function that takes an `Option<&HashMap<String, OperationArg>>`
//! and an `&mut Dish`. The HashMap corresponds to the parameters of the operation,
//! and is optional since not all operations are parameterized. The Dish is the actual
//! data that the operation works on.
//!
//! To implement a new operation:
//!   1. Create the *rust* function that actually performs the operation.
//!      You can safely assume that the HashMap contains all necessary params
//!      for your function to work properly, so just extract and unwrap those.
//!      Then, you want to unpack the `DishData` and operate on it. You should
//!      prefer to perform the operation successfully under as many circumstances
//!      as possible. Only return an error if it's completely unreasonable to
//!      return success. An example of when an operation would be expected to
//!      fail is when trying to decompress data that does not have correct headers.
//!   
//!   2. Create the OperationInfo struct for your operation. The `arguments` field
//!      is a list of tuples of the form ("argument name", OperationArgType::ArgumentType).
//!      This lets you declaratively specify what arguments your operation takes
//!      and in what order. *There are no optional/default arguments.* All arguments
//!      you specify are required.
//!
//!   3. Add your OperationInfo declaration to the list below!
//!

mod data_format;
mod textual;

use crate::OperationInfo;
use data_format::*;
use textual::*;


/// This is the list of ALL OperationInfo structures
pub static OPERATIONS: &[&OperationInfo] = &[
    &OPINFO_ROT13,       &OPINFO_REVERSE,   &OPINFO_FROMBASE64, &OPINFO_TOBASE64,
    &OPINFO_FROMDECIMAL, &OPINFO_TODECIMAL, &OPINFO_FROMOCTAL,  &OPINFO_TOOCTAL,
    &OPINFO_TOHEX,       &OPINFO_FROMHEX,   &OPINFO_FROMBINARY, &OPINFO_TOBINARY,
    &OPINFO_FROMRADIX,   &OPINFO_TORADIX,
];
