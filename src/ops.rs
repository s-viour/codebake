/// The raw functions for operations on Dishes are implemented here
/// 
/// An operation is a regular rust function that *returns* a closure
/// that takes a `&mut DishData` and returns a `DishResult`. The return type
/// for all operation functions is: `impl Fn(&mut DishData) -> DishResult`.
/// **DO NOT FORGET** to put `move` in front of the closure definition! This ensures
/// that your closure will take ownership of the passed-in variables to the function.
/// For information on `DishData` and `DishResult`, see src/lib.rs
/// 
/// To implement a new operation:
///   1. Create the *rust* function that actually performs the operation.
///      In this function, you should unpack the DishData and operate on it
///      mutably if reasonably possible. You'll see how that's the case in
///      the rot13 function. You have access to *all* rust facilities when
///      defining the operation function.
///   2. Head over to src/lisp/functions.rs for further instruction on making
///      your new operation available in the lisp.
/// 

use crate::{DishData, DishResult};


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

pub fn rot13(n: i64) -> impl Fn(&mut DishData) -> DishResult {
    move |dish| { 
        match dish {
            DishData::Str(s) => {rot13_helper_str(n, s); Ok(())},
            DishData::Bin(b) => {rot13_helper_bin(n, b); Ok(())},
        }
    }
}

pub fn reverse() -> impl Fn(&mut DishData) -> DishResult {
    move |dish| {
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
}
