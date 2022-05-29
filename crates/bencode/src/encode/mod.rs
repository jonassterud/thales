mod parse;

use crate::bval::BVal;

/// Encode data to bencode.
/// 
/// # Arguments
/// 
/// * `content` - data to encode
pub fn encode(content: &BVal) -> Vec<u8> {
    parse::any(content)
}