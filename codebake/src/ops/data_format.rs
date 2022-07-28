use crate::{
    DishData, DishError, DishResult, OperationArgType, OperationArguments, OperationInfo,
    EMPTY_ARGS,
};
use base64;
use regex::Regex;

pub static OPINFO_FROMBASE64: OperationInfo = OperationInfo {
    name: "from-base64",
    description: "converts from base64",
    authors: &["s-viour"],
    category: "Data Format",
    arguments: &[],
    op: from_base64,
};

fn from_base64(_: &OperationArguments, dish: &mut DishData) -> DishResult {
    let data = match dish {
        DishData::Str(s) => s.as_bytes(),
        DishData::Bin(_) => {
            return Err(DishError(
                "cannot convert binary data from base64".to_string(),
            ))
        }
    };

    match base64::decode(data) {
        Ok(d) => {
            *dish = DishData::Bin(d);
            Ok(())
        }
        Err(e) => Err(DishError(format!("base64 decode error: {}", e))),
    }
}

pub static OPINFO_TOBASE64: OperationInfo = OperationInfo {
    name: "to-base64",
    description: "converts to base64",
    authors: &["s-viour"],
    category: "Data Format",
    arguments: &[],
    op: to_base64,
};

fn to_base64(_: &OperationArguments, dish: &mut DishData) -> DishResult {
    *dish = DishData::Str(base64::encode(dish.as_bytes()));
    Ok(())
}

pub static OPINFO_FROMDECIMAL: OperationInfo = OperationInfo {
    name: "from-decimal",
    description: "converts a decimal-encoded string to its raw form",
    authors: &["s-viour"],
    category: "Data Format",
    arguments: &[],
    op: from_decimal,
};

fn from_decimal(_: &OperationArguments, dish: &mut DishData) -> DishResult {
    from_radix_helper(10, dish)
}

pub static OPINFO_TODECIMAL: OperationInfo = OperationInfo {
    name: "to-decimal",
    description: "converts data to a decimal string",
    authors: &["s-viour"],
    category: "Data Format",
    arguments: &[],
    op: to_decimal,
};

fn to_decimal(_: &OperationArguments, dish: &mut DishData) -> DishResult {
    *dish = DishData::Str(
        dish.as_bytes()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" "),
    );

    Ok(())
}

pub static OPINFO_FROMOCTAL: OperationInfo = OperationInfo {
    name: "from-octal",
    description: "converts an octal-encoded string to its raw form",
    authors: &["s-viour"],
    category: "Data Format",
    arguments: &[],
    op: from_octal,
};

fn from_octal(_: &OperationArguments, dish: &mut DishData) -> DishResult {
    from_radix_helper(8, dish)
}

pub static OPINFO_TOOCTAL: OperationInfo = OperationInfo {
    name: "to-octal",
    description: "converts data to an octal string",
    authors: &["s-viour"],
    category: "Data Format",
    arguments: &[],
    op: to_octal,
};

fn to_octal(_: &OperationArguments, dish: &mut DishData) -> DishResult {
    *dish = DishData::Str(
        dish.as_bytes()
            .iter()
            .map(|x| format!("{:o}", x))
            .collect::<Vec<String>>()
            .join(" "),
    );

    Ok(())
}

pub static OPINFO_FROMHEX: OperationInfo = OperationInfo {
    name: "from-hex",
    description: "converts a hexadecimal encoded string into its raw form",
    authors: &["s-viour"],
    category: "Data Format",
    arguments: &[],
    op: from_hex,
};

fn from_hex(_: &OperationArguments, dish: &mut DishData) -> DishResult {
    from_radix_helper(16, dish)
}

pub static OPINFO_TOHEX: OperationInfo = OperationInfo {
    name: "to-hex",
    description: "converts data into a hexadecimal encoded string",
    authors: &["s-viour"],
    category: "Data Format",
    arguments: &[],
    op: to_hex,
};

fn to_hex(_: &OperationArguments, dish: &mut DishData) -> DishResult {
    *dish = DishData::Str(
        dish.as_bytes()
            .iter()
            .map(|x| format!("{:x}", x))
            .collect::<Vec<String>>()
            .join(" "),
    );

    Ok(())
}

pub static OPINFO_FROMBINARY: OperationInfo = OperationInfo {
    name: "from-binary",
    description: "converts a binary encoded string into its raw form",
    authors: &["s-viour"],
    category: "Data Format",
    arguments: &[],
    op: from_binary,
};

fn from_binary(_: &OperationArguments, dish: &mut DishData) -> DishResult {
    from_radix_helper(2, dish)
}

pub static OPINFO_TOBINARY: OperationInfo = OperationInfo {
    name: "to-binary",
    description: "converts data into a binary-encoded string",
    authors: &["s-viour"],
    category: "Data Format",
    arguments: &[],
    op: to_binary,
};

fn to_binary(_: &OperationArguments, dish: &mut DishData) -> DishResult {
    *dish = DishData::Str(
        dish.as_bytes()
            .iter()
            .map(|x| format!("{:b}", x))
            .collect::<Vec<String>>()
            .join(" "),
    );

    Ok(())
}

pub static OPINFO_FROMRADIX: OperationInfo = OperationInfo {
    name: "from-radix",
    description: "converts data in a given radix back into its raw form",
    authors: &["s-viour"],
    category: "Data Format",
    arguments: &[("radix", OperationArgType::Integer)],
    op: from_radix,
};

fn from_radix(args: &OperationArguments, dish: &mut DishData) -> DishResult {
    let radix_res = args.get_integer("radix")?.try_into();

    match radix_res {
        Ok(r) => from_radix_helper(r, dish),
        Err(e) => Err(DishError(format!("invalid radix. {}", e))),
    }
}

pub static OPINFO_TORADIX: OperationInfo = OperationInfo {
    name: "to-radix",
    description: "converts data into an encoded string of a given radix",
    authors: &["s-viour"],
    category: "Data Format",
    arguments: &[("radix", OperationArgType::Integer)],
    op: to_radix,
};

fn to_radix(args: &OperationArguments, dish: &mut DishData) -> DishResult {
    // explicitly annotate the result here so we get a u32
    let radix_res = args.get_integer("radix")?.try_into();

    match radix_res {
        Ok(r) => {
            // radix_fmt doesn't support anything past 36
            if r > 36 || r == 1 {
                return Err(DishError(format!("unsupported radix `{}`", r)));
            }

            match r {
                // delegate to other functions if it's a specific radix
                2 => to_binary(&EMPTY_ARGS, dish),
                8 => to_octal(&EMPTY_ARGS, dish),
                10 => to_decimal(&EMPTY_ARGS, dish),
                16 => to_hex(&EMPTY_ARGS, dish),
                64 => to_base64(&EMPTY_ARGS, dish),
                // otherwise use radix_fmt
                _ => {
                    *dish = DishData::Str(
                        dish.as_bytes()
                            .iter()
                            .map(|x| format!("{}", radix_fmt::radix(*x, r)))
                            .collect::<Vec<String>>()
                            .join(" "),
                    );
                    Ok(())
                }
            }
        }
        Err(e) => Err(DishError(format!("invalid radix. {}", e))),
    }
}

/// helper function for things like `from-hex` and `from-octal`
/// takes the radix and the dish and performs the entire from-radix process
///
fn from_radix_helper(radix: u32, dish: &mut DishData) -> DishResult {
    let data = match dish {
        DishData::Str(s) => s.split_whitespace(),
        DishData::Bin(_) => {
            return Err(DishError(format!(
                "cannot convert binary data from radix {}",
                radix
            )))
        }
    };

    let data: Result<Vec<u8>, std::num::ParseIntError> =
        data.map(|x| u8::from_str_radix(x, radix)).collect();

    let data = match data {
        Ok(d) => d,
        Err(e) => return Err(DishError(format!("{}", e))),
    };

    match String::from_utf8(data.clone()) {
        Ok(s) => *dish = DishData::Str(s),
        Err(_) => *dish = DishData::Bin(data),
    }

    Ok(())
}

pub static OPINFO_REGEXMATCH: OperationInfo = OperationInfo {
    name: "regex-match",
    description: "finds substrings that match regex",
    authors: &["Egggggg"],
    category: "Data Format",
    arguments: &[("pattern", OperationArgType::String)],
    op: regex_match,
};

fn regex_match(args: &OperationArguments, dish: &mut DishData) -> DishResult {
    let pattern = args.get_string("pattern")?;
    let re = match Regex::new(&pattern) {
        Ok(r) => r,
        Err(e) => return Err(DishError(format!("{}", e))),
    };
    let mut out = Vec::new();
    let data = match dish {
        DishData::Str(s) => s,
        DishData::Bin(_) => return Err(DishError("dish should be string, got binary".to_string())),
    };

    for m in re.find_iter(&data) {
        out.push(m.as_str().to_string())
    }

    *dish = DishData::Str(out.join("\n"));

    Ok(())
}

pub static OPINFO_REGEXREPLACE: OperationInfo = OperationInfo {
    name: "regex-replace",
    description: "replaces substrings using regex groups",
    authors: &["Egggggg"],
    category: "Data Format",
    arguments: &[
        ("pattern", OperationArgType::String),
        ("replacement", OperationArgType::String),
    ],
    op: regex_replace,
};

fn regex_replace(args: &OperationArguments, dish: &mut DishData) -> DishResult {
    let pattern = args.get_string("pattern")?;
    let replacement = args.get_string("replacement")?;
    let re = match Regex::new(&pattern) {
        Ok(r) => r,
        Err(e) => return Err(DishError(format!("{}", e))),
    };
    let data = match dish {
        DishData::Str(s) => s,
        DishData::Bin(_) => return Err(DishError("dish should be string, got binary".to_string())),
    };

    *dish = DishData::Str(re.replace_all(&data, replacement).to_string());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::ops::data_format::*;
    use crate::{DishData, EMPTY_ARGS};

    #[test]
    fn test_to_octal() {
        let mut data = DishData::Bin(vec![42]);
        let _expected = DishData::Str(String::from("52"));
        assert!(matches!(to_octal(&EMPTY_ARGS, &mut data), Ok(())));
        assert!(matches!(data, _expected));
    }

    #[test]
    fn test_from_octal() {
        let mut data = DishData::Str("52".to_string());
        let _expected = DishData::Bin(vec![42]);
        assert!(matches!(from_octal(&EMPTY_ARGS, &mut data), Ok(())));
        assert!(matches!(data, _expected));
    }

    #[test]
    fn test_to_hex() {
        let mut data = DishData::Bin(vec![15]);
        let _expected = DishData::Str(String::from("0f"));
        assert!(matches!(to_hex(&EMPTY_ARGS, &mut data), Ok(())));
        assert!(matches!(data, _expected));

        let mut data = DishData::Bin(vec![26]);
        let _expected = DishData::Str(String::from("1a"));
        assert!(matches!(to_hex(&EMPTY_ARGS, &mut data), Ok(())));
        assert!(matches!(data, _expected));
    }

    #[test]
    fn test_from_hex() {
        let mut data = DishData::Str(String::from("0f"));
        let _expected = DishData::Bin(vec![15]);

        assert!(matches!(from_hex(&EMPTY_ARGS, &mut data), Ok(())));
        assert!(matches!(data, _expected));

        let mut data = DishData::Str(String::from("1a"));
        let _expected = DishData::Bin(vec![26]);
        assert!(matches!(from_hex(&EMPTY_ARGS, &mut data), Ok(())));
        assert!(matches!(data, _expected));
    }

    #[test]
    fn test_from_binary() {
        let mut data = DishData::Str("01101000 01100101 01101100 01101100 01101111".to_string());
        let _expected = DishData::Bin("hello".as_bytes().to_vec());

        assert!(matches!(from_binary(&EMPTY_ARGS, &mut data), Ok(())));
        assert!(matches!(data, _expected));
    }

    #[test]
    fn test_to_binary() {
        let mut data = DishData::Str("hello".to_string());
        let _expected = DishData::Str("01101000 01100101 01101100 01101100 01101111".to_string());

        assert!(matches!(to_binary(&EMPTY_ARGS, &mut data), Ok(())));
        assert!(matches!(data, _expected));
    }
}
