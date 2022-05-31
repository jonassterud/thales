mod parse;

use crate::val::Val;
use anyhow::Result;

/// Decode bencoded data.
///
/// # Arguments
///
/// * `content` - data to decode
pub fn decode(content: &[u8]) -> Result<Val> {
    parse::any(&mut content.iter().peekable())
}
