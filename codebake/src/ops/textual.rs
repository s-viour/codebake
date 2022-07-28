use crate::{DishData, DishResult, OperationArgType, OperationArguments, OperationInfo};

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

pub static OPINFO_ROT13: OperationInfo = OperationInfo {
    name: "rot13",
    description: "rotates characters in the input by the specified amount",
    authors: &["s-viour"],
    category: "Textual",
    arguments: &[("n", OperationArgType::Integer)],
    op: rot13,
};

fn rot13(args: &OperationArguments, dish: &mut DishData) -> DishResult {
    let n = args.get_integer("n")?;
    match dish {
        DishData::Str(s) => {
            rot13_helper_str(n, s);
            Ok(())
        }
        DishData::Bin(b) => {
            rot13_helper_bin(n, b);
            Ok(())
        }
    }
}

pub static OPINFO_REVERSE: OperationInfo = OperationInfo {
    name: "reverse",
    description: "reverses the input",
    authors: &["s-viour"],
    category: "Textual",
    arguments: &[],
    op: reverse,
};

fn reverse(_: &OperationArguments, dish: &mut DishData) -> DishResult {
    match dish {
        DishData::Str(d) => {
            *dish = DishData::Str(d.chars().rev().collect());
            Ok(())
        }
        DishData::Bin(d) => {
            d.reverse();
            Ok(())
        }
    }
}
