use crate::{DishData, DishError, DishResult, OperationArgType, OperationArguments, OperationInfo};


pub static OPINFO_TAKE_BYTES: OperationInfo = OperationInfo {
    name: "take-bytes",
    description: "takes the specified amount of bytes from the input and discards the rest",
    authors: &["s-viour"],
    category: "Utility",
    arguments: &[("n", OperationArgType::Integer)],
    op: take_bytes,
};

fn take_bytes(args: &OperationArguments, dish: &mut DishData) -> DishResult {
    let ni = args.get_integer("n")?;
    if ni < 0 {
        return Err(DishError("amount to take must be nonnegative".to_string()));
    }
    let n = ni as usize;
    
    match dish {
        DishData::Str(s) => {
            let mut v: Vec<u8> = s.as_bytes().to_vec();
            v.truncate(n);
            *dish = DishData::Bin(v);
        },
        DishData::Bin(v) => {
            v.truncate(n);
        }
    }
    
    Ok(())
}

pub static OPINFO_DROP_BYTES: OperationInfo = OperationInfo {
    name: "drop-bytes",
    description: "drops the first `n` bytes from the input and leaves the rest",
    authors: &["s-viour"],
    category: "Utility",
    arguments: &[("n", OperationArgType::Integer)],
    op: drop_bytes,
};

fn drop_bytes(args: &OperationArguments, dish: &mut DishData) -> DishResult {
    let ni = args.get_integer("n")?;
    if ni < 0 {
        return Err(DishError("integer must be nonnegative".to_string()));
    }
    let n = ni as usize;
    
    match dish {
        DishData::Str(s) => {
            let v: Vec<u8> = s
                .as_bytes()
                .to_vec()
                .into_iter()
                .skip(n)
                .collect();
            
            *dish = DishData::Bin(v);
        },
        DishData::Bin(v) => {
            *v = v
                .iter()
                .skip(n)
                .cloned()
                .collect();
        }
    }
    
    Ok(())
}
