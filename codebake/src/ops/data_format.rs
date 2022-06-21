use crate::{DishData, DishError, DishResult, OperationArg, OperationInfo};
use std::collections::HashMap;
use base64;


pub static OPINFO_FROMBASE64: OperationInfo = OperationInfo {
    name: "from-base64",
    description: "converts from base64",
    arguments: &[],
    op: from_base64,
};

fn from_base64(_: Option<&HashMap<String, OperationArg>>, dish: &mut DishData) -> DishResult {
    let data = match dish {
        DishData::Str(s) => s.as_bytes(),
        DishData::Bin(_) => return Err(DishError("cannot convert binary data from base64".to_string())),
    };

    match base64::decode(data) {
        Ok(d) => {
            *dish = DishData::Bin(d);
            Ok(())
        },
        Err(e) => Err(DishError(format!("base64 decode error: {}", e))),
    }
}

pub static OPINFO_TOBASE64: OperationInfo = OperationInfo {
    name: "to-base64",
    description: "converts to base64",
    arguments: &[],
    op: to_base64,
};

fn to_base64(_: Option<&HashMap<String, OperationArg>>, dish: &mut DishData) -> DishResult {
    *dish = DishData::Str(base64::encode(dish.as_bytes()));
    Ok(())
}

pub static OPINFO_FROMDECIMAL: OperationInfo = OperationInfo {
    name: "from-decimal",
    description: "converts a decimal-encoded string to its raw form",
    arguments: &[],
    op: from_decimal,
};

fn from_decimal(_: Option<&HashMap<String, OperationArg>>, dish: &mut DishData) -> DishResult {
    let data = match dish {
        DishData::Str(s) => s.split_whitespace(),
        DishData::Bin(_) => return Err(DishError("cannot convert binary data from decimal".to_string())),
    };

    let data: Result<Vec<u8>, std::num::ParseIntError> = data
        .map(|x| x.parse::<u8>())
        .collect();

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

pub static OPINFO_TODECIMAL: OperationInfo = OperationInfo {
    name: "to-decimal",
    description: "converts data to a decimal string",
    arguments: &[],
    op: to_decimal,
};

fn to_decimal(_: Option<&HashMap<String, OperationArg>>, dish: &mut DishData) -> DishResult {
    *dish = DishData::Str(dish.as_bytes()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" "));
    
    Ok(())
}

pub static OPINFO_TOOCTAL: OperationInfo = OperationInfo {
    name: "to-octal",
    description: "converts data to an octal string",
    arguments: &[],
    op: to_octal,
};

fn to_octal(_: Option<&HashMap<String, OperationArg>>, dish: &mut DishData) -> DishResult {
    *dish = DishData::Str(dish.as_bytes()
        .iter()
        .map(|x| format!("{:o}", x))
        .collect::<Vec<String>>()
        .join(" "));

    Ok(())
}

pub static OPINFO_FROMOCTAL: OperationInfo = OperationInfo {
    name: "from-octal",
    description: "converts an octal-encoded string to its raw form",
    arguments: &[],
    op: from_octal,
};

fn from_octal(_: Option<&HashMap<String, OperationArg>>, dish: &mut DishData) -> DishResult {
    let data = match dish {
        DishData::Str(s) => s.split_whitespace(),
        DishData::Bin(_) => return Err(DishError("cannot convert binary data from octal".to_string())),
    };

    let data: Result<Vec<u8>, std::num::ParseIntError> = data
        .map(|x| u8::from_str_radix(x, 8))
        .collect();

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

pub static OPINFO_TOHEX: OperationInfo = OperationInfo {
    name: "to-hex",
    description: "converts data into a hexadecimal encoded string",
    arguments: &[],
    op: to_hex,
};

fn to_hex(_: Option<&HashMap<String, OperationArg>>, dish: &mut DishData) -> DishResult {
    *dish = DishData::Str(dish.as_bytes()
        .iter()
        .map(|x| format!("{:x}", x))
        .collect::<Vec<String>>()
        .join(" "));

    Ok(())
}

pub static OPINFO_FROMHEX: OperationInfo = OperationInfo {
    name: "from-hex",
    description: "converts a hexadecimal encoded string into its raw form",
    arguments: &[],
    op: from_hex,
};

fn from_hex(_: Option<&HashMap<String, OperationArg>>, dish: &mut DishData) -> DishResult {
    let data = match dish {
        DishData::Str(s) => s.split_whitespace(),
        DishData::Bin(_) => return Err(DishError("cannot convert binary data from hex".to_string())),
    };

    let data: Result<Vec<u8>, std::num::ParseIntError> = data
        .map(|x| u8::from_str_radix(x, 16))
        .collect();

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


#[cfg(test)]
mod tests {
    use crate::DishData;
    use crate::ops::data_format::*;

    #[test]
    fn test_to_octal() {
        let mut data = DishData::Bin(vec![42]);
        let _expected = DishData::Str(String::from("52"));
        assert!(matches!(to_octal(None, &mut data), Ok(())));
        assert!(matches!(data, _expected));
    }

    #[test]
    fn test_from_octal() {
        let mut data = DishData::Str("52".to_string());
        let _expected = DishData::Bin(vec![42]);
        assert!(matches!(from_octal(None, &mut data), Ok(())));
        assert!(matches!(data, _expected));
    }

    #[test]
    fn test_to_hex() {
        let mut data = DishData::Bin(vec![15]);
        let _expected = DishData::Str(String::from("0f"));
        assert!(matches!(to_hex(None, &mut data), Ok(())));
        assert!(matches!(data, _expected));

        let mut data = DishData::Bin(vec![26]);
        let _expected = DishData::Str(String::from("1a"));
        assert!(matches!(to_hex(None, &mut data), Ok(())));
        assert!(matches!(data, _expected));
    }

    #[test]
    fn test_from_hex() {
        let mut data = DishData::Str(String::from("0f"));
        let _expected = DishData::Bin(vec![15]);

        assert!(matches!(from_hex(None, &mut data), Ok(())));
        assert!(matches!(data, _expected));

        let mut data = DishData::Str(String::from("1a"));
        let _expected = DishData::Bin(vec![26]);
        assert!(matches!(from_hex(None, &mut data), Ok(())));
        assert!(matches!(data, _expected));
    }
}
