mod parse;

use crate::val::Val;

pub fn decode(content: &[u8]) -> Val {
    if let Some(val_res) = parse::any(&mut content.iter().peekable()) {
        val_res
    } else {
        panic!("failed decoding");
    }
}
