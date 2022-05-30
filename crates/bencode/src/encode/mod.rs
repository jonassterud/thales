mod parse;

use crate::val::Val;

/// Encode data to bencode.
///
/// # Arguments
///
/// * `content` - data to encode
pub fn encode(content: &Val) -> Vec<u8> {
    parse::any(content)
}
