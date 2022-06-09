/// The raw functions for operations on Dishes are implemented here
/// 
/// An operation is a function that takes an `Option<&HashMap<String, OperationArg>>`
/// and an `&mut Dish`. The HashMap corresponds to the parameters of the operation,
/// and is optional since not all operations are parameterized. The Dish is the actual
/// data that the operation works on.
/// 
/// To implement a new operation:
///   1. Create the *rust* function that actually performs the operation.
///      You can safely assume that the HashMap contains all necessary params
///      for your function to work properly, so just extract and unwrap those.
///      Then, you want to unpack the `DishData` and operate on it. You should
///      prefer to perform the operation successfully under as many circumstances
///      as possible. Only return an error if it's completely unreasonable to
///      return success. An example of when an operation would be expected to 
///      fail is when trying to decompress data that does not have correct headers.
///   
///   2. Create the OperationInfo struct for your operation. The `arguments` field
///      is a list of tuples of the form ("argument name", OperationArgType::ArgumentType).
///      This lets you declaratively specify what arguments your operation takes
///      and in what order. *There are no optional/default arguments.* All arguments
///      you specify are required. 
/// 
///   3. Add your OperationInfo declaration to the list below!
/// 

/// This is the list of ALL OperationInfo structures
pub static OPERATIONS: &'static [&OperationInfo] =
    &[&OPINFO_ROT13, &OPINFO_REVERSE];


use std::collections::HashMap;
use crate::{DishData, DishResult, OperationArgType, OperationArg, OperationInfo};


fn rot13_helper_bin(n: i64, s: &mut [u8]) {
    s.iter_mut().for_each(|c| {
        let cx = *c as i64;
        *c = if *c >= 65 && *c <= 90 {
            (((cx + n - 65) % 26) + 65) as u8
        } else if *c >= 97 && *c <= 122 {
            (((cx + n - 97) % 26) + 97) as u8
        } else {
            *c
        }
    });
}

fn rot13_helper_str(n: i64, s: &mut str) {
    unsafe {
        rot13_helper_bin(n, s.as_bytes_mut());
    }
}

static OPINFO_ROT13: OperationInfo = OperationInfo {
    name: "rot13",
    description: "rotates characters in the input by the specified amount",
    arguments: &[("n", OperationArgType::Integer)],
    op: rot13,
};

fn rot13(args: Option<&HashMap<String, OperationArg>>, dish: &mut DishData) -> DishResult {
    let n = args.unwrap().get("n").unwrap().integer()?;
    match dish {
        DishData::Str(s) => {rot13_helper_str(n, s); Ok(())},
        DishData::Bin(b) => {rot13_helper_bin(n, b); Ok(())},
    }
}

static OPINFO_REVERSE: OperationInfo = OperationInfo {
    name: "reverse",
    description: "reverses the input",
    arguments: &[],
    op: reverse,
};

fn reverse(_: Option<&HashMap<String, OperationArg>>, dish: &mut DishData) -> DishResult {
    match dish {
        DishData::Str(d) => {
            *dish = DishData::Str(d.chars().rev().collect());
            Ok(())
        },
        DishData::Bin(d) => {
            d.reverse();
            Ok(())
        }
    }
}
