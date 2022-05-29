mod parse;

use crate::bval::BVal;

/// Decode bencoded data.
/// 
/// # Arguments
/// 
/// * `content` - data to decode
pub fn decode(content: &[u8]) -> BVal {
    if let Some(val_res) = parse::any(&mut content.iter().peekable()) {
        val_res
    } else {
        panic!("failed decoding");
    }
}
