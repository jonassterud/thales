use std::collections::BTreeMap;

/// Enum to represent Bencoded data values.
#[derive(Debug, PartialEq)]
pub enum BVal {
    /// Integer
    Number(i64),
    /// Byte string
    ByteString(Vec<u8>),
    /// List of values
    List(Vec<BVal>),
    /// Dictionary of values
    Dictionary(BTreeMap<Vec<u8>, BVal>),
}